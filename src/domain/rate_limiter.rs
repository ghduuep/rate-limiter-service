use crate::domain::{rate_limit_policy::RateLimitPolicy, rate_limit_request::RateLimitRequest};

use super::rate_limit::RateLimitResult;

pub trait RateLimiter {
    fn check(&mut self, request: &RateLimitRequest, policy: &RateLimitPolicy) -> RateLimitResult;
}