#[derive(Debug)]
pub enum Errors {
    InsertFailed,
    QueryFailed,
    Unauthorized,
    InvalidId,
    InvalidDate,
    Unknown,
}
