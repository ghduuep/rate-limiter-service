#[derive(Debug)]
pub struct RateLimitRequest {
    pub key: String,
    pub cost: f64,
}