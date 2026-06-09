mod api;
mod application;
mod domain;
mod infrastructure;

use std::sync::{Arc};

use application::rate_limiter_service::RateLimiterService;
use domain::rate_limit_policy::RateLimitPolicy;
use domain::rate_limit_request::RateLimitRequest;

#[tokio::main] async fn main() {
    let policy = RateLimitPolicy {
        capacity: 5.0,
        refill_rate: 1.0,
    };

    let service = Arc::new(RateLimiterService::new());

    let mut handles = Vec::new();

    for i in 0..10 {
        let service_clone = Arc::clone(&service);
        let policy = policy.clone();

        let handle = tokio::spawn(async move {

            let request = RateLimitRequest {
                key: "user:123".to_string(),
                cost: 1,
            };

            let result = service_clone.check(&request, &policy);

            println!(
                "Thread {} -> allowed={}, remaining={}",
                i, result.allowed, result.remaining
            );
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}
