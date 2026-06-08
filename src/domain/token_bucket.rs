use std::time::Instant;

use super::rate_limit::RateLimitResult;
use super::rate_limiter::RateLimiter;

pub struct TokenBucket {
    capacity: u64,
    tokens: u64,
    refill_rate: u64,
    last_refill: Instant,
}

impl TokenBucket {
    pub fn new(
        capacity: u64,
        refill_rate: u64,
    ) -> Self {
        Self {
            capacity,
            tokens: capacity,
            refill_rate,
            last_refill: Instant::now(),
        }
    }
    fn refill(&mut self) {
        let elapsed = self.last_refill.elapsed().as_secs();

        let tokens_to_add = elapsed * self.refill_rate;

        self.tokens = std::cmp::min(self.capacity, self.tokens + tokens_to_add);

        self.last_refill = Instant::now();
    }
}

impl RateLimiter for TokenBucket {
    fn check(&mut self) -> RateLimitResult {
        self.refill();

        if self.tokens == 0 {
            return RateLimitResult { allowed: false, remaining: 0 };
        }

        self.tokens -= 1;

        RateLimitResult { allowed: true, remaining: self.tokens }
    }
}