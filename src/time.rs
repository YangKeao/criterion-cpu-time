use crate::PosixTime;
use libc::{rusage, timespec, c_long, time_t};
use std::time::Duration;
use std::mem::MaybeUninit;

fn get_r_usage() -> Result<Box<rusage>, libc::c_int> {
    let mut r_usage = MaybeUninit::<rusage>::uninit();

    let errno = unsafe {
       libc::getrusage(libc::RUSAGE_SELF, r_usage.as_mut_ptr())
    };

    if errno != 0 {
        Err(errno)
    } else {
        Ok(Box::new(unsafe { r_usage.assume_init() }))
    }
}

fn clock_gettime() -> Result<Box<timespec>, libc::c_int> {
    let mut time_spec = timespec {
        tv_nsec: 0 as c_long,
        tv_sec: 0 as time_t,
    };
    let errno = unsafe {
        libc::clock_gettime(libc::CLOCK_PROCESS_CPUTIME_ID, &mut time_spec as *mut timespec)
    };
    if errno != 0 {
        Err(errno)
    } else {
        Ok(Box::new(time_spec))
    }
}

impl PosixTime {
    pub(crate) fn get_time(&self) -> Duration {
        match self {
            PosixTime::UserTime => {
                let r_usage = get_r_usage();
                match r_usage {
                    Ok(r_usage) => Duration::from_micros(r_usage.ru_utime.tv_usec as u64)
                        + Duration::from_secs(r_usage.ru_utime.tv_sec as u64),
                    Err(errno) => panic!("getrusage() error: {}", errno),
                }
            }
            PosixTime::UserAndSystemTime => {
                let time_spec = clock_gettime();
                match time_spec {
                    Ok(time_spec) =>  Duration::from_secs(time_spec.tv_sec as u64)
                        + Duration::from_nanos(time_spec.tv_nsec as u64),
                    Err(errno) => panic!("clock_gettime() error: {}", errno),
                }
            }
        }

    }
}
