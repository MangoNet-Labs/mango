BEGIN;

CREATE TYPE task_state AS ENUM (
    'OBSERVED', 'WAITING_SOURCE', 'SOURCE_CONFIRMED', 'AUTHENTICATED', 'READY',
    'TARGET_SUBMITTED', 'TARGET_OUTCOME_UNKNOWN', 'RETRY_WAIT', 'SUCCEEDED',
    'SOURCE_INVALIDATED', 'REJECTED', 'MANUAL_REVIEW'
);

CREATE TABLE tasks (
    task_id text PRIMARY KEY CHECK (task_id ~ '^0x[0-9a-f]{64}$'),
    source_genesis_hash text NOT NULL CHECK (source_genesis_hash ~ '^0x[0-9a-f]{64}$'),
    source_contract text NOT NULL CHECK (source_contract ~ '^0x[0-9a-f]{40}$'),
    business_event_id text NOT NULL CHECK (business_event_id ~ '^0x[0-9a-f]{64}$'),
    stable_route_id text NOT NULL CHECK (stable_route_id ~ '^[a-z0-9][a-z0-9._-]{0,63}$'),
    route_revision_snapshot bigint NOT NULL CHECK (route_revision_snapshot > 0),
    route_snapshot jsonb NOT NULL CHECK (jsonb_typeof(route_snapshot) = 'object'),
    policy_snapshot jsonb NOT NULL CHECK (jsonb_typeof(policy_snapshot) = 'object'),
    business_payload jsonb NOT NULL CHECK (jsonb_typeof(business_payload) = 'object'),
    business_payload_digest text NOT NULL CHECK (business_payload_digest ~ '^0x[0-9a-f]{64}$'),
    state task_state NOT NULL DEFAULT 'OBSERVED',
    task_revision bigint NOT NULL DEFAULT 1 CHECK (task_revision > 0),
    resume_phase task_state CHECK (resume_phase IN ('WAITING_SOURCE','SOURCE_CONFIRMED','AUTHENTICATED','READY')),
    attempt_count integer NOT NULL DEFAULT 0 CHECK (attempt_count >= 0),
    lease_owner text,
    lease_expires_at timestamptz,
    lease_fencing_token bigint NOT NULL DEFAULT 0 CHECK (lease_fencing_token >= 0),
    review_required boolean NOT NULL DEFAULT false,
    last_error_code text,
    created_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    updated_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    UNIQUE (source_genesis_hash, source_contract, business_event_id, stable_route_id),
    CHECK ((lease_owner IS NULL) = (lease_expires_at IS NULL)),
    CHECK ((state = 'RETRY_WAIT') = (resume_phase IS NOT NULL))
);

CREATE TABLE source_observations (
    observation_id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    task_id text NOT NULL REFERENCES tasks,
    tx_hash text NOT NULL CHECK (tx_hash ~ '^0x[0-9a-f]{64}$'),
    block_hash text NOT NULL CHECK (block_hash ~ '^0x[0-9a-f]{64}$'),
    source_height bigint NOT NULL CHECK (source_height >= 0),
    log_index integer NOT NULL CHECK (log_index >= 0),
    canonical_status text NOT NULL CHECK (canonical_status IN ('UNKNOWN','CANONICAL','INVALIDATED')),
    evidence jsonb NOT NULL CHECK (jsonb_typeof(evidence) = 'object'),
    observed_at timestamptz NOT NULL,
    UNIQUE (task_id, tx_hash, block_hash, log_index)
);

CREATE TABLE submission_intents (
    task_id text NOT NULL REFERENCES tasks,
    attempt_number integer NOT NULL CHECK (attempt_number > 0),
    fencing_token bigint NOT NULL CHECK (fencing_token > 0),
    route_snapshot jsonb NOT NULL,
    target_genesis_digest text NOT NULL CHECK (target_genesis_digest ~ '^0x[0-9a-f]{64}$'),
    target_package_id text NOT NULL CHECK (target_package_id ~ '^0x[0-9a-f]{64}$'),
    transaction_digest text NOT NULL CHECK (length(transaction_digest) BETWEEN 1 AND 256),
    signed_transaction_bytes bytea NOT NULL CHECK (octet_length(signed_transaction_bytes) > 0),
    signed_bytes_sha256 text NOT NULL CHECK (signed_bytes_sha256 ~ '^[0-9a-f]{64}$'),
    outcome text NOT NULL CHECK (outcome IN ('INTENT_PERSISTED','BROADCAST_UNKNOWN','SUBMITTED','CONFIRMED_FAILURE','CONFIRMED_SUCCESS')),
    created_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    PRIMARY KEY (task_id, attempt_number),
    UNIQUE (task_id, transaction_digest),
    UNIQUE (task_id, attempt_number, transaction_digest)
);

-- The sender needs exact local test bytes after restart. Surface only the digest in public reads.
CREATE TABLE target_effects (
    task_id text NOT NULL,
    attempt_number integer NOT NULL,
    effect_status text NOT NULL CHECK (effect_status IN ('SUCCESS','FAILURE')),
    transaction_digest text NOT NULL,
    confirmed_effect jsonb NOT NULL CHECK (jsonb_typeof(confirmed_effect) = 'object'),
    PRIMARY KEY (task_id, attempt_number),
    FOREIGN KEY (task_id, attempt_number, transaction_digest)
        REFERENCES submission_intents (task_id, attempt_number, transaction_digest)
);
CREATE UNIQUE INDEX one_confirmed_success_per_task ON target_effects (task_id)
    WHERE effect_status = 'SUCCESS';

CREATE TABLE receipts (
    task_id text PRIMARY KEY REFERENCES tasks,
    business_payload_digest text NOT NULL,
    route_revision_snapshot bigint NOT NULL,
    confirmed_receipt jsonb NOT NULL CHECK (jsonb_typeof(confirmed_receipt) = 'object'),
    created_at timestamptz NOT NULL DEFAULT clock_timestamp()
);

CREATE TABLE incidents (
    incident_id text PRIMARY KEY,
    task_id text NOT NULL REFERENCES tasks,
    kind text NOT NULL,
    evidence jsonb NOT NULL,
    created_at timestamptz NOT NULL DEFAULT clock_timestamp()
);

CREATE TABLE operator_actions (
    task_id text NOT NULL REFERENCES tasks,
    action_id text NOT NULL CHECK (action_id ~ '^[a-zA-Z0-9][a-zA-Z0-9._:-]{0,127}$'),
    request_sha256 text NOT NULL CHECK (request_sha256 ~ '^[0-9a-f]{64}$'),
    expected_task_revision bigint NOT NULL CHECK (expected_task_revision > 0),
    action text NOT NULL CHECK (action IN ('RECONCILE_ONLY','RESUME_PHASE','RETRY_CONFIRMED_FAILURE')),
    accepted_response jsonb NOT NULL CHECK (jsonb_typeof(accepted_response) = 'object'),
    work_status text NOT NULL DEFAULT 'PENDING' CHECK (work_status IN ('PENDING','CLAIMED','DONE','NEEDS_REVIEW')),
    available_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    created_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    PRIMARY KEY (task_id, action_id)
);

CREATE TABLE scanner_cursors (
    source_namespace text PRIMARY KEY CHECK (length(source_namespace) > 0),
    cursor jsonb NOT NULL CHECK (jsonb_typeof(cursor) = 'object'),
    revision bigint NOT NULL CHECK (revision > 0),
    updated_at timestamptz NOT NULL DEFAULT clock_timestamp()
);

CREATE FUNCTION preserve_task_identity_and_success() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF ROW(NEW.task_id, NEW.source_genesis_hash, NEW.source_contract, NEW.business_event_id,
           NEW.stable_route_id, NEW.route_revision_snapshot, NEW.route_snapshot, NEW.policy_snapshot,
           NEW.business_payload, NEW.business_payload_digest)
       IS DISTINCT FROM
       ROW(OLD.task_id, OLD.source_genesis_hash, OLD.source_contract, OLD.business_event_id,
           OLD.stable_route_id, OLD.route_revision_snapshot, OLD.route_snapshot, OLD.policy_snapshot,
           OLD.business_payload, OLD.business_payload_digest) THEN
        RAISE EXCEPTION 'immutable task identity, payload and route/policy snapshots';
    END IF;
    IF OLD.state = 'SUCCEEDED' AND NEW.state <> 'SUCCEEDED' THEN
        RAISE EXCEPTION 'successful task state is immutable';
    END IF;
    IF NEW.lease_fencing_token < OLD.lease_fencing_token OR NEW.task_revision <= OLD.task_revision THEN
        RAISE EXCEPTION 'stale task mutation';
    END IF;
    RETURN NEW;
END;
$$;
CREATE TRIGGER task_identity_immutable BEFORE UPDATE ON tasks
    FOR EACH ROW EXECUTE FUNCTION preserve_task_identity_and_success();

CREATE FUNCTION prevent_evidence_mutation() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    RAISE EXCEPTION 'recorded % evidence is immutable', TG_TABLE_NAME;
END;
$$;
CREATE TRIGGER target_effects_immutable BEFORE UPDATE OR DELETE ON target_effects
    FOR EACH ROW EXECUTE FUNCTION prevent_evidence_mutation();
CREATE TRIGGER receipts_immutable BEFORE UPDATE OR DELETE ON receipts
    FOR EACH ROW EXECUTE FUNCTION prevent_evidence_mutation();
CREATE TRIGGER incidents_immutable BEFORE UPDATE OR DELETE ON incidents
    FOR EACH ROW EXECUTE FUNCTION prevent_evidence_mutation();

CREATE FUNCTION admit_task(p_task jsonb) RETURNS jsonb
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, public, pg_temp AS $$
DECLARE
    candidate tasks := jsonb_populate_record(NULL::tasks, p_task);
    stored tasks;
BEGIN
    -- Lookup the immutable event and route first, including when the current revision has changed.
    SELECT * INTO stored FROM tasks
    WHERE source_genesis_hash = candidate.source_genesis_hash
      AND source_contract = candidate.source_contract
      AND business_event_id = candidate.business_event_id
      AND stable_route_id = candidate.stable_route_id
    FOR UPDATE;

    IF NOT FOUND THEN
        IF COALESCE(candidate.state, 'OBSERVED') NOT IN
           ('OBSERVED','WAITING_SOURCE','SOURCE_CONFIRMED','AUTHENTICATED','READY','SOURCE_INVALIDATED','REJECTED') THEN
            RAISE EXCEPTION 'invalid initial task state' USING ERRCODE = '22023';
        END IF;
        INSERT INTO tasks (
            task_id, source_genesis_hash, source_contract, business_event_id, stable_route_id,
            route_revision_snapshot, route_snapshot, policy_snapshot, business_payload,
            business_payload_digest, state
        ) VALUES (
            candidate.task_id, candidate.source_genesis_hash, candidate.source_contract,
            candidate.business_event_id, candidate.stable_route_id, candidate.route_revision_snapshot,
            candidate.route_snapshot, candidate.policy_snapshot, candidate.business_payload,
            candidate.business_payload_digest, COALESCE(candidate.state, 'OBSERVED')
        ) ON CONFLICT DO NOTHING RETURNING * INTO stored;
        IF FOUND THEN
            RETURN jsonb_build_object('disposition', 'CREATED', 'task', to_jsonb(stored));
        END IF;

        -- A competing insert can win while the insert statement is waiting on a unique key.
        SELECT * INTO STRICT stored FROM tasks
        WHERE task_id = candidate.task_id OR
              (source_genesis_hash, source_contract, business_event_id, stable_route_id) =
              (candidate.source_genesis_hash, candidate.source_contract, candidate.business_event_id, candidate.stable_route_id)
        ORDER BY (task_id = candidate.task_id)
        LIMIT 1
        FOR UPDATE;
    END IF;

    RETURN jsonb_build_object(
        'disposition', CASE WHEN
            ROW(stored.task_id, stored.source_genesis_hash, stored.source_contract,
                stored.business_event_id, stored.stable_route_id, stored.business_payload_digest, stored.business_payload)
            IS NOT DISTINCT FROM
            ROW(candidate.task_id, candidate.source_genesis_hash, candidate.source_contract,
                candidate.business_event_id, candidate.stable_route_id, candidate.business_payload_digest, candidate.business_payload)
            THEN 'REPLAY' ELSE 'CONFLICT' END,
        'task', to_jsonb(stored)
    );
END;
$$;

CREATE FUNCTION commit_source_batch(
    p_namespace text, p_expected_revision bigint, p_cursor jsonb, p_entries jsonb
) RETURNS bigint LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, public, pg_temp AS $$
DECLARE
    committed_revision bigint;
    entry jsonb;
    admitted jsonb;
    observation source_observations;
BEGIN
    IF p_expected_revision IS NULL OR p_expected_revision < 0
       OR jsonb_typeof(p_entries) IS DISTINCT FROM 'array' THEN
        RAISE EXCEPTION 'invalid scanner batch' USING ERRCODE = '22023';
    END IF;
    IF p_expected_revision = 0 THEN
        INSERT INTO scanner_cursors (source_namespace, cursor, revision)
        VALUES (p_namespace, p_cursor, 1)
        ON CONFLICT (source_namespace) DO NOTHING RETURNING revision INTO committed_revision;
    ELSE
        UPDATE scanner_cursors
        SET cursor = p_cursor, revision = revision + 1, updated_at = clock_timestamp()
        WHERE source_namespace = p_namespace AND revision = p_expected_revision
        RETURNING revision INTO committed_revision;
    END IF;
    IF committed_revision IS NULL THEN
        RETURN 0;
    END IF;

    FOR entry IN SELECT value FROM jsonb_array_elements(p_entries)
        ORDER BY value #>> '{task,task_id}', value #>> '{observation,observed_at}'
    LOOP
        admitted := admit_task(entry -> 'task');
        IF admitted ->> 'disposition' = 'CONFLICT' THEN
            RAISE EXCEPTION 'scanner observation conflicts with immutable task' USING ERRCODE = '23505';
        END IF;
        observation := jsonb_populate_record(NULL::source_observations, entry -> 'observation');
        INSERT INTO source_observations (
            task_id, tx_hash, block_hash, source_height, log_index, canonical_status, evidence, observed_at
        ) VALUES (
            admitted #>> '{task,task_id}', observation.tx_hash, observation.block_hash,
            observation.source_height, observation.log_index, observation.canonical_status,
            observation.evidence, observation.observed_at
        ) ON CONFLICT (task_id, tx_hash, block_hash, log_index) DO UPDATE
        SET canonical_status = EXCLUDED.canonical_status, evidence = EXCLUDED.evidence
        WHERE source_observations.source_height = EXCLUDED.source_height;
        IF NOT FOUND THEN
            RAISE EXCEPTION 'scanner observation height changed' USING ERRCODE = '22023';
        END IF;
    END LOOP;
    RETURN committed_revision;
END;
$$;

CREATE FUNCTION claim_task(p_task_id text, p_owner text, p_lease_seconds integer)
RETURNS SETOF tasks LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, public, pg_temp AS $$
BEGIN
    IF p_owner IS NULL OR length(p_owner) = 0 OR p_lease_seconds IS NULL OR p_lease_seconds < 1 THEN
        RAISE EXCEPTION 'invalid lease owner or duration' USING ERRCODE = '22023';
    END IF;
    RETURN QUERY UPDATE tasks
    SET lease_owner = p_owner, lease_expires_at = clock_timestamp() + make_interval(secs => p_lease_seconds),
        lease_fencing_token = lease_fencing_token + 1, task_revision = task_revision + 1,
        updated_at = clock_timestamp()
    WHERE task_id = p_task_id AND state NOT IN ('SUCCEEDED','REJECTED','MANUAL_REVIEW')
      AND (lease_owner IS NULL OR lease_expires_at <= clock_timestamp())
    RETURNING *;
END;
$$;

-- Recheck the expiry after acquiring the row lock so a blocked worker cannot use an expired lease.
CREATE FUNCTION lock_fenced_task(p_task_id text, p_owner text, p_fence bigint, p_revision bigint)
RETURNS tasks LANGUAGE plpgsql AS $$
DECLARE
    stored tasks;
BEGIN
    SELECT * INTO stored FROM tasks
    WHERE task_id = p_task_id AND lease_owner = p_owner
      AND lease_fencing_token = p_fence AND task_revision = p_revision
    FOR UPDATE;
    IF NOT FOUND OR stored.lease_expires_at <= clock_timestamp() THEN
        RETURN NULL;
    END IF;
    RETURN stored;
END;
$$;

CREATE FUNCTION release_task(p_task_id text, p_owner text, p_fence bigint, p_revision bigint)
RETURNS SETOF tasks LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, public, pg_temp AS $$
BEGIN
    IF (lock_fenced_task(p_task_id, p_owner, p_fence, p_revision)).task_id IS NULL THEN
        RETURN;
    END IF;
    RETURN QUERY UPDATE tasks
    SET lease_owner = NULL, lease_expires_at = NULL, task_revision = task_revision + 1,
        updated_at = clock_timestamp()
    WHERE task_id = p_task_id
    RETURNING *;
END;
$$;

CREATE FUNCTION advance_task(
    p_task_id text, p_owner text, p_fence bigint, p_revision bigint, p_state task_state,
    p_resume_phase task_state DEFAULT NULL, p_error text DEFAULT NULL
) RETURNS SETOF tasks LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, public, pg_temp AS $$
DECLARE
    stored tasks := lock_fenced_task(p_task_id, p_owner, p_fence, p_revision);
    previous_outcome text;
    allowed boolean;
BEGIN
    IF stored.task_id IS NULL THEN
        RETURN;
    END IF;
    SELECT outcome INTO previous_outcome FROM submission_intents
    WHERE task_id = p_task_id AND attempt_number = stored.attempt_count;

    allowed := CASE p_state
        WHEN 'WAITING_SOURCE' THEN stored.state IN ('OBSERVED','SOURCE_INVALIDATED')
        WHEN 'SOURCE_CONFIRMED' THEN stored.state = 'WAITING_SOURCE'
        WHEN 'AUTHENTICATED' THEN stored.state = 'SOURCE_CONFIRMED'
        WHEN 'READY' THEN stored.state = 'AUTHENTICATED'
        WHEN 'TARGET_SUBMITTED' THEN stored.state = 'READY'
        WHEN 'TARGET_OUTCOME_UNKNOWN' THEN stored.state IN ('READY','TARGET_SUBMITTED','TARGET_OUTCOME_UNKNOWN')
        WHEN 'SOURCE_INVALIDATED' THEN stored.state IN ('OBSERVED','WAITING_SOURCE')
        WHEN 'REJECTED' THEN stored.state IN ('OBSERVED','WAITING_SOURCE','SOURCE_CONFIRMED','AUTHENTICATED','READY')
        WHEN 'RETRY_WAIT' THEN stored.state IN ('WAITING_SOURCE','SOURCE_CONFIRMED','AUTHENTICATED','READY')
                               AND p_resume_phase = stored.state
        WHEN 'MANUAL_REVIEW' THEN stored.state IN ('WAITING_SOURCE','SOURCE_CONFIRMED','AUTHENTICATED',
                                      'READY','TARGET_SUBMITTED','TARGET_OUTCOME_UNKNOWN','RETRY_WAIT')
        ELSE false
    END;
    IF stored.state = 'RETRY_WAIT' AND p_state = stored.resume_phase THEN
        allowed := true;
    END IF;
    IF p_state IN ('TARGET_SUBMITTED','TARGET_OUTCOME_UNKNOWN') THEN
        allowed := allowed AND previous_outcome IN ('INTENT_PERSISTED','SUBMITTED','BROADCAST_UNKNOWN');
    ELSIF p_state IN ('RETRY_WAIT','REJECTED') THEN
        allowed := allowed AND (previous_outcome IS NULL OR previous_outcome = 'CONFIRMED_FAILURE');
    END IF;
    IF allowed IS NOT TRUE THEN
        RETURN;
    END IF;

    IF p_state IN ('TARGET_SUBMITTED','TARGET_OUTCOME_UNKNOWN') THEN
        UPDATE submission_intents
        SET outcome = CASE p_state WHEN 'TARGET_SUBMITTED' THEN 'SUBMITTED' ELSE 'BROADCAST_UNKNOWN' END
        WHERE task_id = p_task_id AND attempt_number = stored.attempt_count;
    END IF;
    RETURN QUERY UPDATE tasks
    SET state = p_state, resume_phase = p_resume_phase, last_error_code = p_error,
        review_required = review_required OR p_state = 'MANUAL_REVIEW',
        task_revision = task_revision + 1, updated_at = clock_timestamp()
    WHERE task_id = p_task_id
    RETURNING *;
END;
$$;

CREATE FUNCTION record_submission_intent(
    p_task_id text, p_owner text, p_fence bigint, p_revision bigint,
    p_target_genesis text, p_target_package text, p_digest text, p_signed_bytes bytea, p_signed_sha text
) RETURNS SETOF tasks LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, public, pg_temp AS $$
DECLARE
    stored tasks := lock_fenced_task(p_task_id, p_owner, p_fence, p_revision);
BEGIN
    IF stored.task_id IS NULL OR stored.state <> 'READY'
       OR (stored.attempt_count > 0 AND NOT EXISTS (
           SELECT 1 FROM submission_intents
           WHERE task_id = p_task_id AND attempt_number = stored.attempt_count AND outcome = 'CONFIRMED_FAILURE'
       )) THEN
        RETURN;
    END IF;
    IF p_target_genesis IS DISTINCT FROM stored.route_snapshot ->> 'target_genesis_digest'
       OR p_target_package IS DISTINCT FROM stored.route_snapshot ->> 'target_package_id' THEN
        RAISE EXCEPTION 'submission domain differs from pinned route' USING ERRCODE = '22023';
    END IF;
    IF p_signed_sha IS DISTINCT FROM encode(sha256(p_signed_bytes), 'hex') THEN
        RAISE EXCEPTION 'submission bytes digest differs from supplied digest' USING ERRCODE = '22023';
    END IF;
    INSERT INTO submission_intents (
        task_id, attempt_number, fencing_token, route_snapshot, target_genesis_digest,
        target_package_id, transaction_digest, signed_transaction_bytes, signed_bytes_sha256, outcome
    ) VALUES (
        p_task_id, stored.attempt_count + 1, p_fence, stored.route_snapshot, p_target_genesis,
        p_target_package, p_digest, p_signed_bytes, p_signed_sha, 'INTENT_PERSISTED'
    );
    RETURN QUERY UPDATE tasks
    SET attempt_count = attempt_count + 1, task_revision = task_revision + 1, updated_at = clock_timestamp()
    WHERE task_id = p_task_id
    RETURNING *;
END;
$$;

CREATE FUNCTION complete_task(
    p_task_id text, p_owner text, p_fence bigint, p_revision bigint, p_attempt integer,
    p_transaction_digest text, p_effect jsonb, p_receipt jsonb
) RETURNS SETOF tasks LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, public, pg_temp AS $$
DECLARE
    stored tasks := lock_fenced_task(p_task_id, p_owner, p_fence, p_revision);
BEGIN
    IF stored.task_id IS NULL OR stored.state NOT IN ('TARGET_SUBMITTED','TARGET_OUTCOME_UNKNOWN')
       OR stored.attempt_count IS DISTINCT FROM p_attempt OR NOT EXISTS (
           SELECT 1 FROM submission_intents WHERE task_id = p_task_id AND attempt_number = p_attempt
           AND transaction_digest = p_transaction_digest AND outcome IN ('SUBMITTED','BROADCAST_UNKNOWN')
       ) THEN
        RETURN;
    END IF;
    IF p_receipt ->> 'task_id' IS DISTINCT FROM p_task_id
       OR p_receipt ->> 'business_payload_digest' IS DISTINCT FROM stored.business_payload_digest
       OR p_receipt -> 'route_revision_snapshot' IS DISTINCT FROM to_jsonb(stored.route_revision_snapshot)
       OR p_receipt ->> 'state' IS DISTINCT FROM 'SUCCEEDED' THEN
        RAISE EXCEPTION 'receipt differs from immutable task binding' USING ERRCODE = '22023';
    END IF;

    INSERT INTO target_effects (task_id, attempt_number, effect_status, transaction_digest, confirmed_effect)
    VALUES (p_task_id, p_attempt, 'SUCCESS', p_transaction_digest, p_effect);
    INSERT INTO receipts (task_id, business_payload_digest, route_revision_snapshot, confirmed_receipt)
    VALUES (p_task_id, stored.business_payload_digest, stored.route_revision_snapshot, p_receipt);
    UPDATE submission_intents SET outcome = 'CONFIRMED_SUCCESS'
    WHERE task_id = p_task_id AND attempt_number = p_attempt;

    RETURN QUERY UPDATE tasks
    SET state = 'SUCCEEDED', last_error_code = NULL, lease_owner = NULL, lease_expires_at = NULL,
        task_revision = task_revision + 1, updated_at = clock_timestamp()
    WHERE task_id = p_task_id
    RETURNING *;
END;
$$;

CREATE FUNCTION record_target_failure(
    p_task_id text, p_owner text, p_fence bigint, p_revision bigint, p_attempt integer,
    p_transaction_digest text, p_effect jsonb
) RETURNS SETOF tasks LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, public, pg_temp AS $$
DECLARE
    stored tasks := lock_fenced_task(p_task_id, p_owner, p_fence, p_revision);
BEGIN
    IF stored.task_id IS NULL OR stored.state NOT IN ('TARGET_SUBMITTED','TARGET_OUTCOME_UNKNOWN')
       OR stored.attempt_count IS DISTINCT FROM p_attempt OR NOT EXISTS (
           SELECT 1 FROM submission_intents WHERE task_id = p_task_id AND attempt_number = p_attempt
           AND transaction_digest = p_transaction_digest AND outcome IN ('SUBMITTED','BROADCAST_UNKNOWN')
       ) THEN
        RETURN;
    END IF;
    INSERT INTO target_effects (task_id, attempt_number, effect_status, transaction_digest, confirmed_effect)
    VALUES (p_task_id, p_attempt, 'FAILURE', p_transaction_digest, p_effect);
    UPDATE submission_intents SET outcome = 'CONFIRMED_FAILURE'
    WHERE task_id = p_task_id AND attempt_number = p_attempt;

    RETURN QUERY UPDATE tasks
    SET state = 'RETRY_WAIT', resume_phase = 'READY', last_error_code = 'TARGET_CONFIRMED_FAILURE',
        task_revision = task_revision + 1, updated_at = clock_timestamp()
    WHERE task_id = p_task_id
    RETURNING *;
END;
$$;

CREATE FUNCTION record_operator_action(
    p_task_id text, p_action_id text, p_request_sha text, p_expected_revision bigint,
    p_action text, p_accepted_response jsonb
) RETURNS jsonb LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, public, pg_temp AS $$
DECLARE
    stored tasks;
    existing operator_actions;
    ack jsonb;
BEGIN
    SELECT * INTO stored FROM tasks WHERE task_id = p_task_id FOR UPDATE;
    IF NOT FOUND THEN
        RETURN jsonb_build_object('disposition', 'NOT_FOUND');
    END IF;
    SELECT * INTO existing FROM operator_actions WHERE task_id = p_task_id AND action_id = p_action_id;
    IF FOUND THEN
        IF existing.request_sha256 = p_request_sha THEN
            RETURN jsonb_build_object('disposition', 'REPLAY', 'response', existing.accepted_response);
        END IF;
        RETURN jsonb_build_object('disposition', 'CONFLICT', 'code', 'ACTION_ID_CONFLICT');
    END IF;
    IF stored.task_revision IS DISTINCT FROM p_expected_revision THEN
        RETURN jsonb_build_object('disposition', 'CONFLICT', 'code', 'TASK_REVISION_CONFLICT');
    END IF;
    IF (CASE p_action
        WHEN 'RECONCILE_ONLY' THEN stored.state IN ('TARGET_SUBMITTED','TARGET_OUTCOME_UNKNOWN')
        WHEN 'RESUME_PHASE' THEN stored.state = 'RETRY_WAIT' AND stored.last_error_code IS DISTINCT FROM 'TARGET_CONFIRMED_FAILURE'
        WHEN 'RETRY_CONFIRMED_FAILURE' THEN stored.state = 'RETRY_WAIT' AND stored.last_error_code = 'TARGET_CONFIRMED_FAILURE'
        ELSE false
    END) IS NOT TRUE THEN
        RETURN jsonb_build_object('disposition', 'CONFLICT', 'code', 'RETRY_NOT_PERMITTED');
    END IF;
    ack := COALESCE(p_accepted_response, '{}'::jsonb) || jsonb_build_object(
        'action_id', p_action_id, 'task_id', p_task_id, 'accepted_task_revision', stored.task_revision + 1,
        'action', p_action, 'task_identity_unchanged', true, 'creates_target_attempt_now', false
    );
    INSERT INTO operator_actions (
        task_id, action_id, request_sha256, expected_task_revision, action, accepted_response
    ) VALUES (p_task_id, p_action_id, p_request_sha, p_expected_revision, p_action, ack);
    UPDATE tasks SET task_revision = task_revision + 1, updated_at = clock_timestamp()
    WHERE task_id = p_task_id;
    RETURN jsonb_build_object('disposition', 'ACCEPTED', 'response', ack);
END;
$$;

-- Cluster roles carry capabilities only. Provision distinct login identities in the deployment.
DO $roles$
BEGIN
    IF NOT EXISTS (SELECT FROM pg_roles WHERE rolname = 'mango_taskkit_reader') THEN
        CREATE ROLE mango_taskkit_reader NOLOGIN;
    END IF;
    IF NOT EXISTS (SELECT FROM pg_roles WHERE rolname = 'mango_taskkit_api') THEN
        CREATE ROLE mango_taskkit_api NOLOGIN;
    END IF;
    IF NOT EXISTS (SELECT FROM pg_roles WHERE rolname = 'mango_taskkit_worker') THEN
        CREATE ROLE mango_taskkit_worker NOLOGIN;
    END IF;
END;
$roles$;

REVOKE CREATE ON SCHEMA public FROM PUBLIC, mango_taskkit_reader, mango_taskkit_api, mango_taskkit_worker;
GRANT USAGE ON SCHEMA public TO mango_taskkit_reader;
GRANT mango_taskkit_reader TO mango_taskkit_api, mango_taskkit_worker;
GRANT SELECT ON tasks, source_observations, incidents, scanner_cursors, receipts TO mango_taskkit_reader;
GRANT SELECT (task_id, attempt_number, transaction_digest) ON submission_intents TO mango_taskkit_reader;
GRANT SELECT ON submission_intents, target_effects, operator_actions TO mango_taskkit_worker;

REVOKE EXECUTE ON FUNCTION
    preserve_task_identity_and_success(),
    prevent_evidence_mutation(),
    admit_task(jsonb),
    commit_source_batch(text, bigint, jsonb, jsonb),
    claim_task(text, text, integer),
    lock_fenced_task(text, text, bigint, bigint),
    release_task(text, text, bigint, bigint),
    advance_task(text, text, bigint, bigint, task_state, task_state, text),
    record_submission_intent(text, text, bigint, bigint, text, text, text, bytea, text),
    complete_task(text, text, bigint, bigint, integer, text, jsonb, jsonb),
    record_target_failure(text, text, bigint, bigint, integer, text, jsonb),
    record_operator_action(text, text, text, bigint, text, jsonb)
FROM PUBLIC;

GRANT EXECUTE ON FUNCTION admit_task(jsonb), record_operator_action(text, text, text, bigint, text, jsonb)
TO mango_taskkit_api;
GRANT EXECUTE ON FUNCTION
    commit_source_batch(text, bigint, jsonb, jsonb),
    claim_task(text, text, integer),
    release_task(text, text, bigint, bigint),
    advance_task(text, text, bigint, bigint, task_state, task_state, text),
    record_submission_intent(text, text, bigint, bigint, text, text, text, bytea, text),
    complete_task(text, text, bigint, bigint, integer, text, jsonb, jsonb),
    record_target_failure(text, text, bigint, bigint, integer, text, jsonb)
TO mango_taskkit_worker;

COMMIT;
