use workspaces::result::{ExecutionFinalResult, ExecutionOutcome};

pub fn format_execution_result(res: &ExecutionFinalResult) -> String {
    if res.is_success() {
        format!(
            "\ntransaction: {}
receipts: {}
is success: {:#?}",
            format_execution_outcome(res.outcome()),
            format_receipt_outcomes(res.receipt_outcomes()),
            res.is_success()
        )
    } else {
        format!("{:#?}", res)
    }
}

fn format_receipt_outcomes(outcomes: &[ExecutionOutcome]) -> String {
    outcomes
        .iter()
        .map(|outcome| format_execution_outcome(outcome) + "\n")
        .collect()
}

fn format_execution_outcome(outcome: &ExecutionOutcome) -> String {
    let logs = if outcome.logs.is_empty() {
        String::new()
    } else {
        let logs = format!(",\nlogs: {:#?}", outcome.logs);
        logs.lines().map(|l| "    ".to_owned() + l + "\n").collect()
    };
    format!(
        "
    executor_id: {}
    gas_burnt: {}{}",
        outcome.executor_id, outcome.gas_burnt, logs,
    )
}
