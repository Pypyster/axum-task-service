use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Pending,
    InProcess,
    Done,
    Cancel,
}

impl From<TaskStatus> for String {
    fn from(value: TaskStatus) -> Self {
        match value {
            TaskStatus::Pending => "pending".to_string(),
            TaskStatus::InProcess => "in_process".to_string(),
            TaskStatus::Done => "done".to_string(),
            TaskStatus::Cancel => "cancel".to_string(),
        }
    }
}

impl TryFrom<String> for TaskStatus {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "pending" => Ok(TaskStatus::Pending),
            "in_process" => Ok(TaskStatus::InProcess),
            "done" => Ok(TaskStatus::Done),
            "cancel" => Ok(TaskStatus::Cancel),
            other => Err(format!("Unknown task status: {}", other)),
        }
    }
}
