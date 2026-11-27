BEGIN;

-- Old workers could send before updating the outcome. Their rows stay lookup-only.
ALTER TABLE public.submission_intents
    ADD COLUMN recoverable_before_dispatch boolean NOT NULL DEFAULT false;

CREATE FUNCTION public.record_recoverable_submission_intent(
    p_task_id text, p_owner text, p_fence bigint, p_revision bigint,
    p_target_genesis text, p_target_package text, p_digest text, p_signed_bytes bytea, p_signed_sha text
) RETURNS SETOF public.tasks LANGUAGE plpgsql
SECURITY DEFINER SET search_path = pg_catalog, public, pg_temp AS $$
DECLARE
    recorded public.tasks;
BEGIN
    SELECT * INTO recorded FROM public.record_submission_intent(
        p_task_id, p_owner, p_fence, p_revision, p_target_genesis, p_target_package,
        p_digest, p_signed_bytes, p_signed_sha
    );
    IF NOT FOUND THEN
        RETURN;
    END IF;
    UPDATE public.submission_intents
       SET recoverable_before_dispatch = true
     WHERE task_id = p_task_id AND attempt_number = recorded.attempt_count;
    RETURN NEXT recorded;
END;
$$;

-- Commit this marker before calling the target. Any later worker can only look up the digest.
CREATE FUNCTION public.begin_submission_dispatch(
    p_task_id text, p_owner text, p_fence bigint, p_revision bigint, p_digest text
) RETURNS jsonb LANGUAGE plpgsql
SECURITY DEFINER SET search_path = pg_catalog, public, pg_temp AS $$
DECLARE
    stored public.tasks := public.lock_fenced_task(p_task_id, p_owner, p_fence, p_revision);
    signed_bytes bytea;
BEGIN
    IF stored.task_id IS NULL OR stored.state <> 'READY' THEN
        RETURN NULL;
    END IF;
    UPDATE public.submission_intents
       SET outcome = 'BROADCAST_UNKNOWN', recoverable_before_dispatch = false
     WHERE task_id = p_task_id AND attempt_number = stored.attempt_count
       AND transaction_digest = p_digest AND outcome = 'INTENT_PERSISTED'
       AND recoverable_before_dispatch
       AND route_snapshot = stored.route_snapshot
       AND target_genesis_digest = stored.route_snapshot ->> 'target_genesis_digest'
       AND target_package_id = stored.route_snapshot ->> 'target_package_id'
       AND signed_bytes_sha256 = encode(sha256(signed_transaction_bytes), 'hex')
     RETURNING signed_transaction_bytes INTO signed_bytes;
    IF NOT FOUND THEN
        RETURN NULL;
    END IF;
    RETURN jsonb_build_object('digest', p_digest, 'signed_hex', encode(signed_bytes, 'hex'));
END;
$$;

REVOKE EXECUTE ON FUNCTION
    public.record_recoverable_submission_intent(text, text, bigint, bigint, text, text, text, bytea, text),
    public.begin_submission_dispatch(text, text, bigint, bigint, text)
FROM PUBLIC;
GRANT EXECUTE ON FUNCTION
    public.record_recoverable_submission_intent(text, text, bigint, bigint, text, text, text, bytea, text),
    public.begin_submission_dispatch(text, text, bigint, bigint, text)
TO mango_taskkit_worker;

COMMIT;
