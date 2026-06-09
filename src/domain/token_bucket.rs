use std::time::Instant;

use crate::domain::rate_limit_policy::RateLimitPolicy;
use crate::domain::rate_limit_request::RateLimitRequest;

use super::rate_limit::RateLimitResult;
use super::rate_limiter::RateLimiter;

pub struct TokenBucket {
    tokens: u64,
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
        let elapsed = self.last_refill.elapsed().as_secs();

        let tokens_to_add = elapsed * policy.refill_rate;

        self.tokens = std::cmp::min(policy.capacity, self.tokens + tokens_to_add);

        self.last_refill = Instant::now();
    }
}

impl RateLimiter for TokenBucket {
    fn check(&mut self, request: &RateLimitRequest, policy: &RateLimitPolicy) -> RateLimitResult {
        self.refill(policy);

        if self.tokens < request.cost {
            return RateLimitResult { allowed: false, remaining: 0 };
        }

        self.tokens -= request.cost;

        RateLimitResult { allowed: true, remaining: self.tokens }
    }
}