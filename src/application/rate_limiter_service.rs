use std::collections::HashMap;

use crate::domain::rate_limit::RateLimitResult;
use crate::domain::rate_limit_policy::RateLimitPolicy;
use crate::domain::rate_limit_request::RateLimitRequest;
use crate::domain::rate_limiter::RateLimiter;
use crate::domain::token_bucket::TokenBucket;

pub struct RateLimiterService {
    buckets: HashMap<String, TokenBucket>,
    policies: HashMap<String, RateLimitPolicy>
}

impl RateLimiterService {
    pub fn new() -> Self {
        Self {
            buckets: HashMap::new(),
            policies: HashMap::new(),
        }
    }
}

impl RateLimiterService {
    pub fn check(
        &mut self,
        request: &RateLimitRequest,
        policy: &RateLimitPolicy,
    ) -> RateLimitResult {
        
        let bucket = self.buckets.entry(request.key.clone()).or_insert_with(|| {
            TokenBucket::new(policy)
        });

        bucket.check(request, policy)
    }
}
