BEGIN;

CREATE TABLE IF NOT EXISTS public.task_kit_dependency_health (
    dependency text NOT NULL CHECK (dependency IN ('source_fixture', 'mango_localnet')),
    network_identity text NOT NULL CHECK (network_identity ~ '^0x[0-9a-f]{64}$'),
    status text NOT NULL CHECK (status IN ('UP', 'DOWN')),
    observed_at_unix_ms bigint NOT NULL CHECK (observed_at_unix_ms BETWEEN 0 AND 9007199254740991),
    last_successful_poll_unix_ms bigint CHECK (last_successful_poll_unix_ms BETWEEN 0 AND observed_at_unix_ms),
    PRIMARY KEY (dependency, network_identity),
    CHECK (status <> 'UP' OR last_successful_poll_unix_ms IS NOT DISTINCT FROM observed_at_unix_ms)
);

CREATE OR REPLACE FUNCTION public.record_dependency_health(
    p_dependency text, p_network_identity text, p_status text, p_observed_at_unix_ms bigint
) RETURNS jsonb LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, public, pg_temp AS $$
DECLARE
    saved public.task_kit_dependency_health;
BEGIN
    INSERT INTO public.task_kit_dependency_health AS health (
        dependency, network_identity, status, observed_at_unix_ms, last_successful_poll_unix_ms
    ) VALUES (
        p_dependency, p_network_identity, p_status, p_observed_at_unix_ms,
        CASE WHEN p_status = 'UP' THEN p_observed_at_unix_ms END
    ) ON CONFLICT (dependency, network_identity) DO UPDATE SET
        status = CASE
            WHEN EXCLUDED.observed_at_unix_ms > health.observed_at_unix_ms THEN EXCLUDED.status
            WHEN EXCLUDED.observed_at_unix_ms = health.observed_at_unix_ms AND EXCLUDED.status = 'DOWN' THEN 'DOWN'
            ELSE health.status
        END,
        observed_at_unix_ms = GREATEST(health.observed_at_unix_ms, EXCLUDED.observed_at_unix_ms),
        last_successful_poll_unix_ms = GREATEST(health.last_successful_poll_unix_ms, EXCLUDED.last_successful_poll_unix_ms)
    RETURNING * INTO STRICT saved;

    RETURN to_jsonb(saved);
END;
$$;

REVOKE ALL ON TABLE public.task_kit_dependency_health FROM PUBLIC;
GRANT SELECT ON TABLE public.task_kit_dependency_health TO mango_taskkit_reader;
REVOKE EXECUTE ON FUNCTION public.record_dependency_health(text, text, text, bigint) FROM PUBLIC;
GRANT EXECUTE ON FUNCTION public.record_dependency_health(text, text, text, bigint) TO mango_taskkit_worker;

COMMIT;
