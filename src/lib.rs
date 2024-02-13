pub mod enums;
pub mod counter;

pub fn name() -> &'static str {
    "timetrack.rs"
}

use std::time::{Instant, Duration};
use std::ops::Sub;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct ActiveDurationState {
    pub duration: Duration,
    pub paused: bool,
}

#[derive(Clone, Debug)]
pub struct ActiveDuration {
    start_from: Instant,
    state: ActiveDurationState,
}

impl ActiveDuration {
    pub fn from_str(s: &str) -> Result<ActiveDuration, &'static str> {
        let paused = !s.ends_with('*');
        let numstring = match paused {
            true => s,
            false => &s[..s.len() - 1],
        };
        let seconds = numstring.parse::<u64>().unwrap_or(0);
        Ok(ActiveDuration {
            start_from: Instant::now().sub(Duration::from_secs(seconds)),
            state: ActiveDurationState {
                duration: Duration::from_secs(seconds),
                paused,
            },
        })
    }

    pub fn new() -> ActiveDuration {
        ActiveDuration {
            start_from: Instant::now(),
            state: ActiveDurationState {
                duration: Duration::from_secs(0),
                paused: false,
            },
        }
    }

    pub fn to_string(&self) -> String {
        match self.paused() {
            true => format!("{}", self.elapsed().as_secs()),
            false => format!("{}*", self.elapsed().as_secs()),
        }
    }

    pub fn state(&self) -> ActiveDurationState {
        match self.state.paused {
            true => self.state.clone(),
            false => ActiveDurationState {
                duration: Instant::now().duration_since(self.start_from),
                paused: false,
            }
        }
    }

    pub fn elapsed(&self) -> Duration {
        match self.state.paused {
            true => self.state.duration.clone(),
            false => Instant::now().duration_since(self.start_from),
        }
    }

    pub fn pause(&mut self) {
        if !self.paused() {
            self.state.paused = true;
            self.state.duration = Instant::now().duration_since(self.start_from);
        }
    }

    pub fn resume(&mut self) {
        self.start_from = Instant::now().sub(self.state.duration);
        self.state.paused = false;
    }

    pub fn paused(&self) -> bool {
        self.state.paused
    }
}

#[cfg(test)]
mod active_duration {
    use std::thread::sleep;

    use super::*;

    #[test]
    fn elapsed_returns_duration_when_paused() {
        let mut ad = ActiveDuration::new();
        assert!(!ad.paused());
        ad.pause();
        assert!(ad.paused());
        let start_duration = ad.elapsed();
        sleep(Duration::from_millis(100));
        assert_eq!(ad.elapsed(), start_duration);
    }

    #[test]
    fn elapsed_returns_larger_duration_when_not_paused() {
        let ad = ActiveDuration::new();
        let start_duration = ad.elapsed();
        sleep(Duration::from_millis(100));
        assert!(ad.elapsed() > start_duration);
    }

    #[test]
    fn test_pause_and_unpase() {
        let mut ad = ActiveDuration::new();
        let start = ad.elapsed();

        // assert that an active ActiveDuration object increments elapsed time after a sleep
        sleep(Duration::from_millis(100));
        assert!(ad.elapsed() > start);
        ad.pause();
        let start = ad.elapsed();

        // assert that a paused ActiveDuration object does not increment elapsed time after a sleep
        sleep(Duration::from_millis(100));
        let elapsed = ad.elapsed();
        assert_eq!(elapsed, start);

        // sleep for half a second while paused; elapsed time should not increase significantly when we resume
        sleep(Duration::from_millis(500));
        ad.resume();
        // assert that the resumed time after pause/resume makes sense
        assert!(ad.elapsed() - elapsed < Duration::from_millis(500));
    }

    #[test]
    fn test_paused_from_str() {
        let ad = ActiveDuration::from_str("100").unwrap();
        assert!(ad.paused());
        assert_eq!(ad.elapsed(), Duration::from_secs(100));
    }

    #[test]
    fn test_unpaused_from_str() {
        let ad = ActiveDuration::from_str("500*").unwrap();
        assert!(!ad.paused());
        assert!(ad.elapsed() > Duration::from_secs(500));
    }

    #[test]
    fn test_paused_to_string() {
        assert_eq!(ActiveDuration::from_str("1000").unwrap().to_string(), "1000");
    }

    #[test]
    fn test_unpaused_to_string() {
        let ad = ActiveDuration::from_str("1000*").unwrap();
        sleep(Duration::from_secs(1));
        assert_eq!(ad.to_string(), "1001*");
    }

    #[test]
    fn test_pause_is_idempotent() {
        let mut ad = ActiveDuration::from_str("1000*").unwrap();
        ad.pause();
        assert_eq!(ad.elapsed().as_secs(), 1000);
        sleep(Duration::from_secs(1));
        ad.pause();
        assert_eq!(ad.elapsed().as_secs(), 1000);
    }
}
