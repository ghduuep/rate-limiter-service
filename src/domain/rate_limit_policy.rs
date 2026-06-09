#[derive(Clone)]
pub struct RateLimitPolicy {
    pub capacity: f64,
    pub refill_rate: f64,
}