mod api;
mod application;
mod domain;
mod infrastructure;

use std::sync::{Arc, Mutex};
use std::thread;

use application::rate_limiter_service::RateLimiterService;

fn main() {
  let service = Arc::new(Mutex::new(RateLimiterService::new(5,1)));

  let mut handles = Vec::new();
  
  for i in 0..10 {
      let service_clone = Arc::clone(&service);

      let handle = thread::spawn(move || {
          let mut service = service_clone.lock().unwrap();

          let result = service.check("user:123");

          println!("Thread {} -> allowed={}, remaining={}", i, result.allowed, result.remaining);
      });

      handles.push(handle);
  }

  for handle in handles {
      handle.join().unwrap();
  }
}
