
#[derive(Debug, Copy, Clone)]
pub enum TaskImportance {
    Low = 0,
    Normal = 1,
    High = 2,
}

#[derive(Debug, Copy, Clone)]
pub enum TaskStatus {
    NotStarted = 0,
    Completed = 1,
}
