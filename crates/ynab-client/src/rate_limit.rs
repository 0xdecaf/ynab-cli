use std::collections::VecDeque;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// Client-side rate limiter for the YNAB API (200 requests per hour).
pub struct RateLimiter {
    window: Duration,
    max_requests: usize,
    timestamps: Mutex<VecDeque<Instant>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            window: Duration::from_secs(3600), // 1 hour
            max_requests: 200,
            timestamps: Mutex::new(VecDeque::new()),
        }
    }

    /// Check if a request can be made. Returns Ok(remaining) or Err(wait_duration).
    pub fn check(&self) -> Result<usize, Duration> {
        let mut timestamps = self.timestamps.lock().unwrap();
        let now = Instant::now();
        let cutoff = now - self.window;

        // Remove expired timestamps
        while timestamps.front().is_some_and(|&t| t < cutoff) {
            timestamps.pop_front();
        }

        if timestamps.len() >= self.max_requests {
            let oldest = timestamps.front().unwrap();
            let wait = self.window - (now - *oldest);
            Err(wait)
        } else {
            Ok(self.max_requests - timestamps.len())
        }
    }

    /// Record that a request was made.
    pub fn record(&self) {
        let mut timestamps = self.timestamps.lock().unwrap();
        timestamps.push_back(Instant::now());
    }

    /// Get the number of remaining requests in the current window.
    pub fn remaining(&self) -> usize {
        self.check().unwrap_or(0)
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}
