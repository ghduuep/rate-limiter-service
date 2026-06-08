use super::rate_limit::RateLimitResult;

pub trait RateLimiter {
    fn check(&mut self) -> RateLimitResult;
}