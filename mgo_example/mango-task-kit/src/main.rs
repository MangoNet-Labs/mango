mod contract;

use std::{
    net::SocketAddr,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::{bail, Context, Result};
use axum::{
    body::{Body, Bytes},
    extract::{DefaultBodyLimit, Path, State},
    http::{header, HeaderMap, Request, StatusCode},
    middleware::{self, Next},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use contract::{bounded_integer, hex_id, valid_action_id, Domain, Ingest, Retry};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use sqlx::{postgres::PgPoolOptions, PgPool};
use subtle::ConstantTimeEq;

const VERSION: &str = "1.0.1-local";

#[derive(Clone)]
struct App {
    database: PgPool,
    tokens: Arc<Tokens>,
    domain: Arc<Domain>,
}

struct Tokens {
    reader: String,
    operator: String,
    fixture_ingest: String,
}

#[derive(Clone, Copy, PartialEq)]
enum Role {
    Reader,
    Operator,
    FixtureIngest,
}

struct ApiError(StatusCode, &'static str, &'static str);

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            self.0,
            Json(json!({"code": self.1, "message": self.2,
            "task_id": null, "retryable": self.0 == StatusCode::SERVICE_UNAVAILABLE})),
        )
            .into_response()
    }
}

type ApiResult<T> = std::result::Result<T, ApiError>;

fn invalid() -> ApiError {
    ApiError(
        StatusCode::BAD_REQUEST,
        "INVALID_REQUEST",
        "Invalid request",
    )
}

fn not_found() -> ApiError {
    ApiError(StatusCode::NOT_FOUND, "TASK_NOT_FOUND", "Task not found")
}

async fn local_headers(request: Request<Body>, next: Next) -> Response {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();
    headers.insert(
        header::CACHE_CONTROL,
        header::HeaderValue::from_static("no-store"),
    );
    headers.insert(
        header::X_CONTENT_TYPE_OPTIONS,
        header::HeaderValue::from_static("nosniff"),
    );
    headers.insert(header::CONTENT_SECURITY_POLICY, header::HeaderValue::from_static(
        "default-src 'none'; script-src 'self'; connect-src 'self'; style-src 'self'; form-action 'none'; frame-ancestors 'none'"));
    response
}

fn database_error(error: sqlx::Error) -> ApiError {
    eprintln!(
        "database operation failed ({})",
        error
            .as_database_error()
            .and_then(|e| e.code())
            .unwrap_or_default()
    );
    ApiError(
        StatusCode::SERVICE_UNAVAILABLE,
        "DEPENDENCY_UNAVAILABLE",
        "Database unavailable",
    )
}

fn auth(app: &App, headers: &HeaderMap, allowed: &[Role]) -> ApiResult<Role> {
    let token = headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(ApiError(
            StatusCode::UNAUTHORIZED,
            "UNAUTHORIZED",
            "A local access token is required",
        ))?;
    let options = [
        (Role::Reader, &app.tokens.reader),
        (Role::Operator, &app.tokens.operator),
        (Role::FixtureIngest, &app.tokens.fixture_ingest),
    ];
    let mut role = None;
    for (candidate, secret) in options {
        if token.as_bytes().ct_eq(secret.as_bytes()).into() {
            role = Some(candidate);
        }
    }
    match role {
        Some(role) if allowed.contains(&role) => Ok(role),
        Some(_) => Err(ApiError(
            StatusCode::FORBIDDEN,
            "FORBIDDEN",
            "This role cannot perform the requested action",
        )),
        None => Err(ApiError(
            StatusCode::UNAUTHORIZED,
            "UNAUTHORIZED",
            "Invalid local access token",
        )),
    }
}

fn decode<T: DeserializeOwned>(body: &Bytes) -> ApiResult<T> {
    serde_json::from_slice(body).map_err(|_| invalid())
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

const TASK_JSON: &str = r#"
SELECT jsonb_build_object(
    'task_id', t.task_id, 'task_revision', t.task_revision,
    'stable_route_id', t.stable_route_id, 'route_revision_snapshot', t.route_revision_snapshot,
    'business_payload_digest', t.business_payload_digest, 'state', t.state,
    'attempt_count', t.attempt_count, 'target_transaction_digest',
      (SELECT transaction_digest FROM submission_intents s WHERE s.task_id=t.task_id ORDER BY attempt_number DESC LIMIT 1),
    'review_required', t.review_required,
    'incident_refs', COALESCE((SELECT jsonb_agg(incident_id ORDER BY incident_id) FROM incidents i WHERE i.task_id=t.task_id), '[]'::jsonb),
    'reason_code', t.last_error_code,
    'next_action', CASE t.state
      WHEN 'OBSERVED' THEN 'WAIT_SOURCE' WHEN 'WAITING_SOURCE' THEN 'WAIT_SOURCE'
      WHEN 'SOURCE_CONFIRMED' THEN 'AUTHORIZE_FIXTURE' WHEN 'AUTHENTICATED' THEN 'VALIDATE_RECIPIENT'
      WHEN 'READY' THEN 'SUBMIT_IF_ACTIVE' WHEN 'TARGET_SUBMITTED' THEN 'RECONCILE'
      WHEN 'TARGET_OUTCOME_UNKNOWN' THEN 'RECONCILE' WHEN 'RETRY_WAIT' THEN 'WAIT_RETRY'
      WHEN 'MANUAL_REVIEW' THEN 'OPERATOR_REVIEW' ELSE 'NONE' END,
    'created_at_unix_ms', floor(extract(epoch FROM t.created_at)*1000)::bigint,
    'updated_at_unix_ms', floor(extract(epoch FROM t.updated_at)*1000)::bigint,
    'source_last_successful_poll_unix_ms',
      (SELECT floor(extract(epoch FROM max(sc.updated_at))*1000)::bigint FROM scanner_cursors sc WHERE sc.source_namespace=t.source_genesis_hash),
    'target_last_successful_poll_unix_ms', NULL)
FROM tasks t WHERE t.task_id=$1"#;

async fn task(app: &App, id: &str) -> ApiResult<Value> {
    if !hex_id(id, 32) {
        return Err(invalid());
    }
    let mut task: Value = sqlx::query_scalar(TASK_JSON)
        .bind(id)
        .fetch_optional(&app.database)
        .await
        .map_err(database_error)?
        .ok_or_else(not_found)?;
    let dependency_health_installed: bool =
        sqlx::query_scalar("SELECT to_regclass('public.task_kit_dependency_health') IS NOT NULL")
            .fetch_one(&app.database)
            .await
            .map_err(database_error)?;
    if dependency_health_installed {
        let last_success: Option<i64> = sqlx::query_scalar(
            "SELECT h.last_successful_poll_unix_ms FROM public.tasks t
             JOIN public.task_kit_dependency_health h ON h.dependency = 'mango_localnet'
               AND h.network_identity = t.route_snapshot ->> 'target_genesis_digest'
             WHERE t.task_id = $1",
        )
        .bind(id)
        .fetch_optional(&app.database)
        .await
        .map_err(database_error)?
        .flatten();
        task["target_last_successful_poll_unix_ms"] = json!(last_success);
    }
    Ok(task)
}

async fn ingest(State(app): State<App>, headers: HeaderMap, body: Bytes) -> ApiResult<Response> {
    auth(&app, &headers, &[Role::FixtureIngest])?;
    let ingest: Ingest = decode(&body)?;
    let row = ingest.task(&app.domain, now()).map_err(|code| match code {
        "INVALID_REQUEST" => invalid(),
        "UNKNOWN_ROUTE" => ApiError(
            StatusCode::CONFLICT,
            "UNKNOWN_ROUTE",
            "Local route not found",
        ),
        _ => ApiError(
            StatusCode::UNPROCESSABLE_ENTITY,
            "FIXTURE_POLICY_REJECTED",
            "The source, payload, or local authorization does not match",
        ),
    })?;
    let id = row["task_id"].as_str().ok_or_else(invalid)?;
    let incoming_digest = row["business_payload_digest"]
        .as_str()
        .ok_or_else(invalid)?;
    let existing_digest: Option<String> =
        sqlx::query_scalar("SELECT business_payload_digest FROM tasks WHERE task_id=$1")
            .bind(id)
            .fetch_optional(&app.database)
            .await
            .map_err(database_error)?;
    if existing_digest
        .as_deref()
        .is_some_and(|digest| digest != incoming_digest)
    {
        return Err(ApiError(
            StatusCode::CONFLICT,
            "PAYLOAD_CONFLICT",
            "This event is already bound to a different payload",
        ));
    }
    if existing_digest.is_none() && ingest.requested_route_revision != app.domain.route_revision {
        return Err(ApiError(
            StatusCode::CONFLICT,
            "UNSUPPORTED_VERSION",
            "Unsupported route revision",
        ));
    }
    let observed: bool = sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM source_observations o JOIN tasks t USING(task_id)
         WHERE o.task_id=$1 AND o.tx_hash=$2 AND o.block_hash=$3 AND o.source_height=$4 AND o.log_index=$5
         AND o.canonical_status='CANONICAL' AND t.business_payload_digest=$6)")
        .bind(id).bind(&ingest.source.source_transaction_hash).bind(&ingest.source.source_block_hash)
        .bind(ingest.source.source_height as i64).bind(ingest.source.source_log_index as i32)
        .bind(&ingest.source.emitted_payload_digest).fetch_one(&app.database).await.map_err(database_error)?;
    if !observed {
        return Err(ApiError(
            StatusCode::UNPROCESSABLE_ENTITY,
            "FIXTURE_POLICY_REJECTED",
            "The local scanner has yet to confirm this source observation",
        ));
    }
    let disposition: Value = sqlx::query_scalar("SELECT admit_task($1)")
        .bind(&row)
        .fetch_one(&app.database)
        .await
        .map_err(database_error)?;
    if disposition["disposition"] == "CONFLICT" {
        return Err(ApiError(
            StatusCode::CONFLICT,
            "PAYLOAD_CONFLICT",
            "This event is already bound to a different payload",
        ));
    }
    let status = if disposition["disposition"] == "CREATED" {
        StatusCode::ACCEPTED
    } else {
        StatusCode::OK
    };
    Ok((status, Json(task(&app, id).await?)).into_response())
}

async fn get_task(
    State(app): State<App>,
    Path(id): Path<String>,
    headers: HeaderMap,
) -> ApiResult<Response> {
    let role = auth(&app, &headers, &[Role::Reader, Role::Operator])?;
    let access = if role == Role::Operator {
        "operator"
    } else {
        "reader"
    };
    Ok(([("x-task-kit-role", access)], Json(task(&app, &id).await?)).into_response())
}

async fn receipt(
    State(app): State<App>,
    Path(id): Path<String>,
    headers: HeaderMap,
) -> ApiResult<Response> {
    auth(&app, &headers, &[Role::Reader, Role::Operator])?;
    let row = task(&app, &id).await?;
    match row["state"].as_str() {
        Some("SUCCEEDED") => {
            let receipt: Option<Value> =
                sqlx::query_scalar("SELECT confirmed_receipt FROM receipts WHERE task_id=$1")
                    .bind(&id)
                    .fetch_optional(&app.database)
                    .await
                    .map_err(database_error)?;
            let mut receipt = receipt.ok_or(ApiError(
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                "The success receipt is missing",
            ))?;
            receipt["review_required"] = row["review_required"].clone();
            receipt["incident_refs"] = row["incident_refs"].clone();
            Ok(Json(receipt).into_response())
        }
        Some("SOURCE_INVALIDATED" | "REJECTED") => Err(ApiError(
            StatusCode::CONFLICT,
            "RECEIPT_UNAVAILABLE",
            "A success receipt is unavailable for this task",
        )),
        _ => {
            let reason = row["reason_code"].as_str().unwrap_or("AWAITING_EVIDENCE");
            Ok((StatusCode::ACCEPTED, Json(json!({"task_id":id,"status":"PENDING","state":row["state"],
                "reason_code":reason,"retry_after_seconds":3,"review_required":row["review_required"],
                "incident_refs":row["incident_refs"]}))).into_response())
        }
    }
}

async fn retry(
    State(app): State<App>,
    Path(id): Path<String>,
    headers: HeaderMap,
    body: Bytes,
) -> ApiResult<Response> {
    auth(&app, &headers, &[Role::Operator])?;
    let request: Retry = decode(&body)?;
    if !valid_action_id(&request.action_id) || !bounded_integer(request.expected_task_revision) {
        return Err(invalid());
    }
    let row = task(&app, &id).await?;
    let action = match row["state"].as_str() {
        Some("TARGET_OUTCOME_UNKNOWN" | "TARGET_SUBMITTED") => "RECONCILE_ONLY",
        Some("RETRY_WAIT") if row["reason_code"] == "TARGET_CONFIRMED_FAILURE" => {
            "RETRY_CONFIRMED_FAILURE"
        }
        Some("RETRY_WAIT") => "RESUME_PHASE",
        _ => "RECONCILE_ONLY",
    };
    let expected = request.expected_task_revision as i64;
    let hash = hex::encode(Sha256::digest(
        serde_json::to_vec(&json!({
            "action_id":request.action_id, "expected_task_revision":expected,
        }))
        .map_err(|_| invalid())?,
    ));
    let ack = json!({"action_id":request.action_id,"task_id":id,
        "accepted_task_revision":expected + 1,"action":action,
        "task_identity_unchanged":true,"creates_target_attempt_now":false});
    let result: Value = sqlx::query_scalar("SELECT record_operator_action($1,$2,$3,$4,$5,$6)")
        .bind(&id)
        .bind(&request.action_id)
        .bind(hash)
        .bind(expected)
        .bind(action)
        .bind(&ack)
        .fetch_one(&app.database)
        .await
        .map_err(database_error)?;
    match result["disposition"].as_str() {
        Some("ACCEPTED") => {
            Ok((StatusCode::ACCEPTED, Json(result["response"].clone())).into_response())
        }
        Some("REPLAY") => Ok(Json(result["response"].clone()).into_response()),
        Some("NOT_FOUND") => Err(not_found()),
        _ => {
            let code = result["code"].as_str();
            let name = match code {
                Some("ACTION_ID_CONFLICT") => "ACTION_ID_CONFLICT",
                Some("TASK_REVISION_CONFLICT") => "TASK_REVISION_CONFLICT",
                _ => "RETRY_NOT_PERMITTED",
            };
            Err(ApiError(
                StatusCode::CONFLICT,
                name,
                "The task state, revision, or action ID prevents this request",
            ))
        }
    }
}

async fn ready(State(app): State<App>, headers: HeaderMap) -> ApiResult<Response> {
    auth(&app, &headers, &[Role::Reader, Role::Operator])?;
    let (database, dependency_health_installed) = sqlx::query_as::<_, (bool, bool)>(
        "SELECT to_regclass('public.tasks') IS NOT NULL,
                to_regclass('public.task_kit_dependency_health') IS NOT NULL",
    )
    .fetch_one(&app.database)
    .await
    .unwrap_or_default();
    let mut reasons = Vec::new();
    let (source, target) = if database && dependency_health_installed {
        match sqlx::query_as::<_, (Option<String>, Option<String>)>(
            "SELECT (SELECT status FROM public.task_kit_dependency_health
                      WHERE dependency = 'source_fixture' AND network_identity = $1),
                    (SELECT status FROM public.task_kit_dependency_health
                      WHERE dependency = 'mango_localnet' AND network_identity = $2)",
        )
        .bind(&app.domain.source_genesis_hash)
        .bind(&app.domain.target_genesis_digest)
        .fetch_one(&app.database)
        .await
        {
            Ok(health) => health,
            Err(error) => {
                database_error(error);
                reasons.push("DEPENDENCY_HEALTH_UNAVAILABLE");
                (None, None)
            }
        }
    } else {
        (None, None)
    };
    if !database {
        reasons.push("TASK_STORE_UNAVAILABLE");
    }
    let source = source.as_deref().unwrap_or("UNKNOWN");
    let target = target.as_deref().unwrap_or("UNKNOWN");
    if source == "DOWN" {
        reasons.push("SOURCE_FIXTURE_UNAVAILABLE");
    }
    if target == "DOWN" {
        reasons.push("MANGO_LOCALNET_UNAVAILABLE");
    }
    let ready = reasons.is_empty();
    let status = if ready {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };
    Ok((status, Json(json!({
        "ready":ready,"schema_version":"1.0.0-local-candidate",
        "dependency_health":{"database":if database {"UP"}else{"DOWN"},"source_fixture":source,"mango_localnet":target},
        "target_submission_enabled":false,
        "unready_reasons":reasons,
    }))).into_response())
}

fn token(name: &str) -> Result<String> {
    let value = std::env::var(name).with_context(|| name.to_owned())?;
    if value.len() < 32 || value.len() > 256 || !value.bytes().all(|b| b.is_ascii_graphic()) {
        bail!("{name} must contain 32–256 printable non-space ASCII bytes");
    }
    Ok(value)
}

#[tokio::main]
async fn main() -> Result<()> {
    let database = PgPoolOptions::new()
        .max_connections(8)
        .connect(&std::env::var("TASK_KIT_DATABASE_URL")?)
        .await?;
    let domain: Domain =
        serde_json::from_slice(&std::fs::read(std::env::var("TASK_KIT_DOMAIN")?)?)?;
    if !domain.validate() {
        bail!("Invalid local test domain");
    }
    let tokens = Tokens {
        reader: token("TASK_KIT_READER_TOKEN")?,
        operator: token("TASK_KIT_OPERATOR_TOKEN")?,
        fixture_ingest: token("TASK_KIT_INGEST_TOKEN")?,
    };
    if tokens.reader == tokens.operator
        || tokens.reader == tokens.fixture_ingest
        || tokens.operator == tokens.fixture_ingest
    {
        bail!("Local role tokens must be distinct");
    }
    let app = App {
        database,
        domain: Arc::new(domain),
        tokens: Arc::new(tokens),
    };
    let router = Router::new()
        .route("/favicon.ico", get(|| async { StatusCode::NO_CONTENT }))
        .route(
            "/",
            get(|| async { Html(include_str!("../ui/index.html")) }),
        )
        .route(
            "/ui/app.js",
            get(|| async {
                (
                    [(header::CONTENT_TYPE, "text/javascript")],
                    include_str!("../ui/app.js"),
                )
            }),
        )
        .route(
            "/ui/style.css",
            get(|| async {
                (
                    [(header::CONTENT_TYPE, "text/css")],
                    include_str!("../ui/style.css"),
                )
            }),
        )
        .route(
            "/health/live",
            get(|| async { Json(json!({"alive":true,"contract_version":VERSION})) }),
        )
        .route("/health/ready", get(ready))
        .route("/v1/tasks/ingest", post(ingest))
        .route("/v1/tasks/{task_id}", get(get_task))
        .route("/v1/tasks/{task_id}/receipt", get(receipt))
        .route("/v1/tasks/{task_id}/retry", post(retry))
        .layer(DefaultBodyLimit::max(65536))
        .layer(middleware::from_fn(local_headers))
        .with_state(app);
    let address: SocketAddr = std::env::var("TASK_KIT_LISTEN")
        .unwrap_or_else(|_| "127.0.0.1:18080".to_owned())
        .parse()?;
    if address.ip() != std::net::Ipv4Addr::LOCALHOST {
        bail!("Task Kit listens only on IPv4 loopback");
    }
    let listener = tokio::net::TcpListener::bind(address).await?;
    println!("Task Kit ready at {}", listener.local_addr()?);
    axum::serve(listener, router).await?;
    Ok(())
}
