use libc::rusage;
use crate::PosixTime;
use std::mem::size_of;
use std::time::Duration;

fn get_r_usage() -> Result<Box<rusage>, libc::c_int> {
    unsafe {
        let r_usage = libc::malloc(size_of::<rusage>()) as *mut libc::rusage;
        let errno = libc::getrusage(libc::RUSAGE_SELF, r_usage);
        if errno != 0 {
            Err(errno)
        } else {
            Ok(Box::from_raw(r_usage))
        }
    }
}

impl PosixTime {
    pub(crate) fn get_time(&self) -> Duration {
        let r_usage = get_r_usage();
        match r_usage {
            Ok(r_usage) => {
                match self {
                    PosixTime::UserTime => {
                        Duration::from_micros(r_usage.ru_utime.tv_usec as u64)
                    }
                    PosixTime::UserAndSystemTime => {
                        Duration::from_micros(r_usage.ru_utime.tv_usec as u64 + r_usage.ru_stime.tv_usec as u64)
                    }
                }
            }
            Err(errno) => {
                panic!("getrusage() error: {}", errno)
            }
        }
    }
}