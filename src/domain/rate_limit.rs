#[derive(Debug)]
pub struct RateLimitResult {
    pub allowed: bool,
    pub remaining: u64,
}