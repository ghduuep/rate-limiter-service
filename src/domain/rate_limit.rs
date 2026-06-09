#[derive(Debug)]
pub struct RateLimitResult {
    pub allowed: bool,
    pub remaining: f64,
}