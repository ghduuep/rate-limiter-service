use std::collections::HashMap;

use crate::domain::rate_limit::RateLimitResult;
use crate::domain::rate_limiter::RateLimiter;
use crate::domain::token_bucket::TokenBucket;

pub struct RateLimiterService {
    buckets: HashMap<String, TokenBucket>,
    capacity: u64,
    refill_rate: u64,
}

impl RateLimiterService {
    pub fn new(
        capacity: u64,
        refill_rate: u64,
    ) -> Self {
        Self {
            buckets:: HashMap::new(),
            capacity,
            refill_rate,
        }
    }
}

impl RateLimiterService {
    pub fn check(
        &mut self,
        key: &str,
    ) -> RateLimitResult {
        let bucket = self.buckets.entry(key.to_string()).or_insert_with(|| {
            TokenBucket::new(self.capacity, self.refill_rate)
        });

        bucket.check()
    }
}