"""Persist a measured outcome for one local dependency and network identity."""

import json
import re
from time import time_ns


def record_dependency_health(db, dependency, network_identity, *, up, observed_at_unix_ms=None):
    """Record an actual probe result using the task-kit worker database role.

    An explicit timestamp lets a caller preserve the measurement time when
    delivering or retrying a recorded probe outcome.
    """
    if dependency not in ("source_fixture", "mango_localnet"):
        raise ValueError("Unknown local dependency")
    if not isinstance(network_identity, str) or not re.fullmatch(r"0x[0-9a-f]{64}", network_identity):
        raise ValueError("Invalid local network identity")
    if type(up) is not bool:
        raise ValueError("An actual successful or failed probe outcome is required")
    if observed_at_unix_ms is None:
        observed_at_unix_ms = time_ns() // 1_000_000
    if type(observed_at_unix_ms) is not int or not 0 <= observed_at_unix_ms <= 9_007_199_254_740_991:
        raise ValueError("Invalid dependency observation timestamp")

    status = "UP" if up else "DOWN"
    stored = db.query(f"SELECT public.record_dependency_health("
                      f"'{dependency}', '{network_identity}', '{status}', {observed_at_unix_ms});")
    return json.loads(stored)
