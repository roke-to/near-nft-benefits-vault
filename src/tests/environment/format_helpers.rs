use workspaces::result::{ExecutionFinalResult, ExecutionOutcome};

// Simple helper to customize print of the ExecutionFinalResult.
pub fn format_execution_result(res: &ExecutionFinalResult) -> String {
    if res.is_success() {
        let receipts = if res.outcomes().is_empty() {
            String::new()
        } else {
            format!(
                "\nreceipts: {}",
                format_receipt_outcomes(res.receipt_outcomes())
            )
        };
        format!(
            "\ntransaction: {}{}
is success: {:#?}",
            format_execution_outcome(res.outcome()).unwrap_or_else(|| "None".to_owned()),
            receipts,
            res.is_success()
        )
    } else {
        format!("{res:#?}")
    }
}

// Simple helper to customize print of multiple `ExecutionOutcome`s.
fn format_receipt_outcomes(outcomes: &[ExecutionOutcome]) -> String {
    outcomes
        .iter()
        .filter_map(|outcome| format_execution_outcome(outcome).map(|s| s + "\n"))
        .collect()
}

// Simple helper to customize print of the ExecutionOutcome.
fn format_execution_outcome(outcome: &ExecutionOutcome) -> Option<String> {
    if outcome.logs.is_empty() {
        None
    } else {
        let logs = format!(",\nlogs: {:#?}", outcome.logs);
        let logs: String = logs.lines().map(|l| "    ".to_owned() + l + "\n").collect();
        let msg = format!(
            "
    executor_id: {}
    gas_burnt: {}{logs}",
            outcome.executor_id, outcome.gas_burnt,
        );
        Some(msg)
    }
}
