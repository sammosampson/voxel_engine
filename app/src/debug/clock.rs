#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
compile_error!(
    "The TSC crate only supports the \"x86\" and \"x86_64\" architectures"
);

use core::ops;

#[cfg(target_arch = "x86")]
use core::arch::x86 as arch;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64 as arch;

pub struct Start(u64);

impl Start {
    pub fn now() -> Self {
        unsafe {
            let _ = arch::__cpuid(0);
            Start(core::mem::transmute(arch::_rdtsc()))
        }
    }
}

pub struct Stop(u64);

impl Stop {
    pub fn now() -> Self {
        unsafe {
            let mut core: u32 = 0;
            let r = arch::__rdtscp(&mut core as *mut _) as u64;
            let _ = arch::__cpuid(0);
            Stop(r)
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Duration(u64);

impl Duration {
    pub fn cycles(self) -> u64 {
        self.0
    }
}

impl ops::Sub<Start> for Stop {
    type Output = Duration;
    fn sub(self, start: Start) -> Duration {
        debug_assert!(
            self.0 > start.0,
            "stop time instant happened after start time instant"
        );
        Duration(self.0 - start.0)
    }
}

impl ops::Sub<Duration> for Duration {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        debug_assert!(self.0 > other.0, "subtracting durations overflows");
        Duration(self.0 - other.0)
    }
}