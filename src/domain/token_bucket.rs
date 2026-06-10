use std::time::Instant;

use crate::domain::rate_limit_policy::RateLimitPolicy;
use crate::domain::rate_limit_request::RateLimitRequest;

use super::rate_limit::RateLimitResult;
use super::rate_limiter::RateLimiter;

pub struct TokenBucket {
    tokens: f64,
    last_refill: Instant,
}

impl TokenBucket {
    pub fn new(
        policy: &RateLimitPolicy
    ) -> Self {
        Self {
            tokens: policy.capacity,
            last_refill: Instant::now(),
        }
    }
    fn refill(&mut self, policy: &RateLimitPolicy) {
        let elapsed = self.last_refill.elapsed().as_secs_f64();

        let tokens_to_add = elapsed * policy.refill_rate;

        self.tokens = (self.tokens + tokens_to_add).min(policy.capacity);

        self.last_refill = Instant::now();
    }
}

impl RateLimiter for TokenBucket {
    fn check(&mut self, request: &RateLimitRequest, policy: &RateLimitPolicy) -> RateLimitResult {
        self.refill(policy);

        if self.tokens < request.cost as f64{
            return RateLimitResult { allowed: false, remaining: self.tokens };
        }

        self.tokens -= request.cost as f64;

        RateLimitResult { allowed: true, remaining: self.tokens }
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

use super::*;
    use crate::domain::rate_limit_policy::RateLimitPolicy;

    #[test]
    fn test_token_bucket_initialization() {
        let policy = RateLimitPolicy { capacity: 10.0, refill_rate: 1.0};
        let bucket = TokenBucket::new(&policy);

        assert_eq!(bucket.tokens, policy.capacity);
    }

    #[test]
    fn test_token_bucket_refill_and_consume() {
        let policy = RateLimitPolicy { capacity: 10.0, refill_rate: 1.0};
        let mut bucket = TokenBucket::new(&policy);
        let request = RateLimitRequest { key: "user:123".to_string(), cost: 8.0 };
        
        bucket.check(&request, &policy);

        assert_eq!(bucket.tokens, 2.0);

        thread::sleep(Duration::from_secs(1));
        bucket.refill(&policy);

        let epsilon = 0.001;
        assert!((bucket.tokens - 3.0).abs() < epsilon,
            "Expected tokens to be ~3.0, but got {}", bucket.tokens);
    }
}