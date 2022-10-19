
#[derive(Debug, Copy, Clone)]
pub enum TaskImportance {
    Low,
    Normal,
    High,
}

#[derive(Debug, Copy, Clone)]
pub enum TaskStatus {
    NotStarted,
    Completed,
}
