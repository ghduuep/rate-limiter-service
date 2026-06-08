mod api;
mod application;
mod domain;
mod infrastructure;

use domain::rate_limit::RateLimitResult;
use domain::token_bucket::TokenBucket;

use crate::domain::rate_limiter::RateLimiter;

fn main() {
   let mut limiter = TokenBucket::new(5, 1);

   for i in 1..=10 {
       let result = limiter.check();

       println!("Request {} -> allowed={}, remaining={}", i, result.allowed, result.remaining);
   }
}
