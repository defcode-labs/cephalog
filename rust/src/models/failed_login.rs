use std::collections::HashMap;
use std::net::IpAddr;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct FailedLogins {
    per_minute: HashMap<IpAddr, HashMap<u64, usize>>,
    per_10_seconds: HashMap<IpAddr, HashMap<u64, usize>>,
    min_threshold: usize,
    sec_threshold: usize,
    window_mins: u64,
    window_secs: u64,
    last_cleanup: u64, // Tracks when last cleanup happened
    cleanup_interval: u64, // How often to run cleanup (seconds)
}

impl FailedLogins {
    pub fn new(min_threshold: usize, sec_threshold: usize, window_mins: u64, window_secs: u64, cleanup_interval: u64) -> Self {
        Self {
            per_minute: HashMap::new(),
            per_10_seconds: HashMap::new(),
            min_threshold,
            sec_threshold,
            window_mins,
            window_secs,
            last_cleanup: Self::current_time(),
            cleanup_interval,
        }
    }

    fn current_time() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    pub fn register_attempt(&mut self, ip: IpAddr) -> bool {
        let now_seconds = Self::current_time();
        let now_minute = now_seconds / 60;
        let now_10s = now_seconds / 10;

        if now_seconds - self.last_cleanup > self.cleanup_interval {
            self.cleanup_old_attempts();
            self.last_cleanup = now_seconds;
        }

        let min_buckets = self.per_minute.entry(ip).or_insert_with(HashMap::new);
        let sec_buckets = self.per_10_seconds.entry(ip).or_insert_with(HashMap::new);

        *min_buckets.entry(now_minute).or_insert(0) += 1;
        *sec_buckets.entry(now_10s).or_insert(0) += 1;

        let min_failures: usize = min_buckets.values().sum();
        let sec_failures: usize = sec_buckets.values().sum();

        min_failures >= self.min_threshold || sec_failures >= self.sec_threshold
    }

    fn cleanup_old_attempts(&mut self) {
        let now_minute = Self::current_time() / 60;
        let now_10s = Self::current_time() / 10;

        self.per_minute.retain(|_, buckets| {
            buckets.retain(|&min, _| now_minute.saturating_sub(min) < self.window_mins);
            !buckets.is_empty()
        });

        self.per_10_seconds.retain(|_, buckets| {
            buckets.retain(|&sec, _| now_10s.saturating_sub(sec) < self.window_secs);
            !buckets.is_empty()
        });

        println!("[Cleanup] Removed stale login attempts.");
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};

    /// Wrapper to simulate time changes without modifying production code
    struct MockTimeTracker {
        failed_logins: FailedLogins,
        now: u64,
    }

    impl MockTimeTracker {
        fn new(min_threshold: usize, sec_threshold: usize, window_mins: u64, window_secs: u64) -> Self {
            Self {
                failed_logins: FailedLogins::new(min_threshold, sec_threshold, window_mins, window_secs, 60),
                now: 1700000000, // Fixed mock start time
            }
        }

        fn register_attempt(&mut self, ip: IpAddr) -> bool {
            // Override `current_time()` with controlled mock time
            let now_minute = self.now / 60;
            let now_10s = self.now / 10;

            let min_buckets = self.failed_logins.per_minute.entry(ip).or_insert_with(HashMap::new);
            let sec_buckets = self.failed_logins.per_10_seconds.entry(ip).or_insert_with(HashMap::new);

            *min_buckets.entry(now_minute).or_insert(0) += 1;
            *sec_buckets.entry(now_10s).or_insert(0) += 1;

            min_buckets.retain(|&min, _| now_minute.saturating_sub(min) < self.failed_logins.window_mins);
            sec_buckets.retain(|&sec, _| now_10s.saturating_sub(sec) < self.failed_logins.window_secs);

            let min_failures: usize = min_buckets.values().sum();
            let sec_failures: usize = sec_buckets.values().sum();

            min_failures >= self.failed_logins.min_threshold || sec_failures >= self.failed_logins.sec_threshold
        }

        fn advance_time(&mut self, secs: u64) {
            self.now += secs;
        }
    }

    fn mock_ip() -> IpAddr {
        IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))
    }

    #[test]
    fn test_ban_after_three_attempts_in_a_minute() {
        let mut tracker = MockTimeTracker::new(3, 5, 1, 1);
        let ip = mock_ip();

        assert_eq!(tracker.register_attempt(ip), false);
        assert_eq!(tracker.register_attempt(ip), false);
        assert_eq!(tracker.register_attempt(ip), true);
    }

    #[test]
    fn test_ban_after_five_attempts_in_ten_seconds() {
        let mut tracker = MockTimeTracker::new(10, 5, 1, 1);
        let ip = mock_ip();

        for _ in 0..4 {
            assert_eq!(tracker.register_attempt(ip), false);
        }
        assert_eq!(tracker.register_attempt(ip), true); 
    }

    #[test]
    fn test_no_ban_if_attempts_are_spread_out() {
        let mut tracker = MockTimeTracker::new(5, 10, 1, 1);
        let ip = mock_ip();

        for _ in 0..4 {
            tracker.register_attempt(ip);
            tracker.advance_time(20);
        }

        tracker.advance_time(120);
        assert_eq!(tracker.register_attempt(ip), false); 
    }

    #[test]
    fn test_expired_attempts_dont_contribute_to_ban() {
        let mut tracker = MockTimeTracker::new(5, 10, 1, 1);
        let ip = mock_ip();

        for _ in 0..4 {
            tracker.register_attempt(ip);
        }

        tracker.advance_time(61);
        assert_eq!(tracker.register_attempt(ip), false); 
    }

    #[test]
    fn test_burst_attack_gets_caught() {
        let mut tracker = MockTimeTracker::new(10, 5, 1, 1);
        let ip = mock_ip();

        for _ in 0..4 {
            tracker.register_attempt(ip);
            tracker.advance_time(2);
        }

        assert_eq!(tracker.register_attempt(ip), true);
    }
}
