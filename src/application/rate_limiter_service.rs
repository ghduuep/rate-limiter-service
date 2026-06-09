use std::collections::HashMap;

use dashmap::DashMap;

use crate::domain::rate_limit::RateLimitResult;
use crate::domain::rate_limit_policy::RateLimitPolicy;
use crate::domain::rate_limit_request::RateLimitRequest;
use crate::domain::rate_limiter::RateLimiter;
use crate::domain::token_bucket::TokenBucket;

pub struct RateLimiterService {
    buckets: DashMap<String, TokenBucket>,
    policies: HashMap<String, RateLimitPolicy>
}

impl RateLimiterService {
    pub fn new() -> Self {
        Self {
            buckets: DashMap::new(),
            policies: HashMap::new(),
        }
    }
}

impl RateLimiterService {
    pub fn check(
        &self,
        request: &RateLimitRequest,
        policy: &RateLimitPolicy,
    ) -> RateLimitResult {
        
        let mut bucket = self.buckets.entry(request.key.clone()).or_insert_with(|| {
            TokenBucket::new(policy)
        });

        bucket.check(request, policy)
    }
}
