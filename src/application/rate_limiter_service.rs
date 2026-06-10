use dashmap::DashMap;

use crate::domain::rate_limit::RateLimitResult;
use crate::domain::rate_limit_policy::RateLimitPolicy;
use crate::domain::rate_limit_request::RateLimitRequest;
use crate::domain::rate_limiter::RateLimiter;
use crate::domain::token_bucket::TokenBucket;

pub struct RateLimiterService {
    buckets: DashMap<String, TokenBucket>,
    policies: DashMap<String, RateLimitPolicy>
}

impl RateLimiterService {
    pub fn new() -> Self {
        Self {
            buckets: DashMap::new(),
            policies: DashMap::new(),
        }
    }
}

impl RateLimiterService {
    pub fn check(
        &self,
        request: &RateLimitRequest,
    ) -> RateLimitResult {

        let policy = self.policies.get(&request.key).expect("Policy not found for key");
        
        let mut bucket = self.buckets.entry(request.key.clone()).or_insert_with(|| {
            TokenBucket::new(&policy)
        });

        bucket.value_mut().check(request, policy.value())
    }

    pub fn add_policy(&self, key: String, policy: RateLimitPolicy) {
        self.policies.insert(key, policy);
    }
}
