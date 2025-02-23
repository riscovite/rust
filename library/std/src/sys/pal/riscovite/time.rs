use crate::time::Duration;
use super::syscall::{syscall, defs::core::SYS_GET_CURRENT_TIMESTAMP};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Instant(u64);

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct SystemTime(Duration);

pub const UNIX_EPOCH: SystemTime = SystemTime(Duration::from_secs(0));

impl Instant {
    pub fn now() -> Instant {
        // This system function should never fail unless the OS itself
        // is malfunctioning.
        let result = unsafe { syscall!(SYS_GET_CURRENT_TIMESTAMP) };
        Instant(result.value)
    }

    pub fn checked_sub_instant(&self, other: &Instant) -> Option<Duration> {
        let ns = self.0.checked_sub(other.0)?;
        Some(Duration::from_nanos(ns))
    }

    pub fn checked_add_duration(&self, other: &Duration) -> Option<Instant> {
        let other_ns: u64 = other.as_nanos().try_into().ok()?;
        let new_ns = self.0.checked_add(other_ns)?;
        Some(Instant(new_ns))
    }

    pub fn checked_sub_duration(&self, other: &Duration) -> Option<Instant> {
        let other_ns: u64 = other.as_nanos().try_into().ok()?;
        let new_ns = self.0.checked_sub(other_ns)?;
        Some(Instant(new_ns))
    }
}

impl SystemTime {
    pub fn now() -> SystemTime {
        panic!("time not implemented on this platform")
    }

    pub fn sub_time(&self, other: &SystemTime) -> Result<Duration, Duration> {
        self.0.checked_sub(other.0).ok_or_else(|| other.0 - self.0)
    }

    pub fn checked_add_duration(&self, other: &Duration) -> Option<SystemTime> {
        Some(SystemTime(self.0.checked_add(*other)?))
    }

    pub fn checked_sub_duration(&self, other: &Duration) -> Option<SystemTime> {
        Some(SystemTime(self.0.checked_sub(*other)?))
    }
}
