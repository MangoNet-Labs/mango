"use strict";

const byId = (id) => document.getElementById(id);
const notice = byId("notice");
const lookup = byId("lookup");
const operator = byId("operator");
const record = byId("record");
const states = {
  OBSERVED: "Observed", WAITING_SOURCE: "Waiting for source confirmation", SOURCE_CONFIRMED: "Source confirmed",
  AUTHENTICATED: "Locally authorized", READY: "Ready to submit", TARGET_SUBMITTED: "Submitted, awaiting reconciliation",
  TARGET_OUTCOME_UNKNOWN: "Target outcome pending reconciliation", RETRY_WAIT: "Waiting to retry", SUCCEEDED: "Entitlement issued",
  SOURCE_INVALIDATED: "Source event invalidated", REJECTED: "Task rejected", MANUAL_REVIEW: "Awaiting operator review",
};
const actions = {
  WAIT_SOURCE: "Waiting for source confirmation", AUTHORIZE_FIXTURE: "Verify local authorization", VALIDATE_RECIPIENT: "Verify recipient",
  SUBMIT_IF_ACTIVE: "Waiting for local execution", RECONCILE: "Reconcile the original transaction", WAIT_RETRY: "Waiting to retry",
  OPERATOR_REVIEW: "Contact an operator", NONE: "Processing complete",
};
let selected = null;
let lookupSequence = 0;

function call(path, token, options = {}) {
  const headers = { Authorization: "Bearer " + token };
  if (options.body) headers["Content-Type"] = "application/json";
  return fetch(path, { ...options, headers, cache: "no-store" });
}

async function loadTask(id, token) {
  const sequence = ++lookupSequence;
  const path = "/v1/tasks/" + encodeURIComponent(id);
  selected = null;
  record.hidden = true;
  notice.textContent = "Looking up task";
  const response = await call(path, token);
  const data = await response.json();
  if (sequence !== lookupSequence) return;
  if (!response.ok) {
    notice.textContent = data.message || "Task lookup failed";
    return;
  }
  const receiptResponse = await call(path + "/receipt", token);
  const receipt = await receiptResponse.json();
  if (sequence !== lookupSequence) return;

  selected = data;
  byId("state").dataset.state = data.state;
  byId("state").textContent = states[data.state] || data.state;
  byId("reason").textContent = [actions[data.next_action], data.reason_code].filter(Boolean).join(" | ");
  byId("revision").textContent = data.task_revision;
  byId("digest").textContent = data.target_transaction_digest || "Unavailable";
  byId("source-poll").textContent = data.source_last_successful_poll_unix_ms === null ?
    "Unavailable" : new Date(data.source_last_successful_poll_unix_ms).toLocaleString("en-US");
  byId("incidents").textContent = data.incident_refs.join(" | ") || "Unavailable";
  byId("receipt").textContent = receiptResponse.status === 200 ?
    JSON.stringify(receipt, null, 2) : receipt.message || "Receipt pending confirmation";
  operator.hidden = response.headers.get("x-task-kit-role") !== "operator" ||
    !["TARGET_OUTCOME_UNKNOWN", "TARGET_SUBMITTED", "RETRY_WAIT"].includes(data.state);
  byId("action").textContent = data.state === "RETRY_WAIT" ? "Request recovery" : "Request reconciliation";
  record.hidden = false;
  notice.textContent = "";
}

lookup.addEventListener("submit", async (event) => {
  event.preventDefault();
  try {
    await loadTask(byId("task-id").value.trim(), byId("token").value);
  } catch {
    notice.textContent = "Unable to connect to the local service";
  }
});

operator.addEventListener("submit", async (event) => {
  event.preventDefault();
  if (!selected) return;
  const current = selected;
  byId("action").disabled = true;
  try {
    const token = byId("token").value;
    const response = await call("/v1/tasks/" + encodeURIComponent(current.task_id) + "/retry", token, {
      method: "POST",
      body: JSON.stringify({ action_id: byId("action-id").value, expected_task_revision: current.task_revision }),
    });
    const result = await response.json();
    if (!response.ok) {
      notice.textContent = result.message || "Action request rejected";
      return;
    }
    await loadTask(current.task_id, token);
    notice.textContent = "Action request accepted";
  } catch {
    notice.textContent = "Unable to connect to the local service";
  } finally {
    byId("action").disabled = false;
  }
});
