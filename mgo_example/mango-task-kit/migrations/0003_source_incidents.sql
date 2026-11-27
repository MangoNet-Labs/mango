BEGIN;

-- Keep source history and the effect/receipt visible when an observation is reversed.
CREATE FUNCTION public.record_source_observation_transition() RETURNS trigger
LANGUAGE plpgsql SET search_path = pg_catalog, public, pg_temp AS $$
DECLARE
    incident_key text;
    created_incident text;
BEGIN
    IF NEW.canonical_status IN ('CANONICAL', 'UNKNOWN') THEN
        UPDATE public.tasks
           SET state = 'WAITING_SOURCE', last_error_code = NULL,
               updated_at = clock_timestamp(), task_revision = task_revision + 1
         WHERE task_id = NEW.task_id AND state = 'SOURCE_INVALIDATED'
           AND attempt_count = 0;
        RETURN NEW;
    END IF;

    IF TG_OP <> 'UPDATE' OR OLD.canonical_status = 'INVALIDATED' THEN
        RETURN NEW;
    END IF;

    IF OLD.canonical_status = 'CANONICAL' AND EXISTS (
        SELECT 1 FROM public.tasks WHERE task_id = NEW.task_id AND state = 'SUCCEEDED'
    ) THEN
        incident_key := 'source-invalidated:' || encode(sha256(convert_to(
            jsonb_build_array(NEW.task_id, NEW.tx_hash, NEW.block_hash, NEW.log_index)::text, 'UTF8'
        )), 'hex');
        INSERT INTO public.incidents (incident_id, task_id, kind, evidence)
        VALUES (incident_key, NEW.task_id, 'SOURCE_OBSERVATION_INVALIDATED_AFTER_SUCCESS',
            jsonb_build_object('source_transaction_hash', NEW.tx_hash, 'source_block_hash', NEW.block_hash,
                               'source_height', NEW.source_height, 'source_log_index', NEW.log_index,
                               'prior_status', OLD.canonical_status, 'current_status', NEW.canonical_status,
                               'source_observation', NEW.evidence))
        ON CONFLICT (incident_id) DO NOTHING
        RETURNING incident_id INTO created_incident;
        IF created_incident IS NOT NULL THEN
            UPDATE public.tasks
               SET review_required = TRUE, updated_at = clock_timestamp(), task_revision = task_revision + 1
             WHERE task_id = NEW.task_id AND state = 'SUCCEEDED';
        END IF;
    END IF;

    UPDATE public.tasks t
       SET state = 'SOURCE_INVALIDATED', last_error_code = 'SOURCE_OBSERVATION_INVALIDATED',
           updated_at = clock_timestamp(), task_revision = task_revision + 1
     WHERE t.task_id = NEW.task_id AND t.attempt_count = 0
       AND t.state IN ('OBSERVED', 'WAITING_SOURCE')
       AND NOT EXISTS (
           SELECT 1 FROM public.source_observations o
            WHERE o.task_id = t.task_id AND o.canonical_status IN ('CANONICAL', 'UNKNOWN')
       );
    RETURN NEW;
END;
$$;

REVOKE EXECUTE ON FUNCTION public.record_source_observation_transition() FROM PUBLIC;
CREATE TRIGGER source_observation_transition
AFTER INSERT OR UPDATE OF canonical_status ON public.source_observations
FOR EACH ROW EXECUTE FUNCTION public.record_source_observation_transition();

COMMIT;
