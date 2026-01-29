/// Parser module for processing history.jsonl files
pub struct SessionRecord {
    pub display: String,
    pub timestamp: i64,
    pub project: String,
    pub session_id: String,
}
