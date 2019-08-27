use crate::PosixTime;
use libc::rusage;
use libc::timespec;
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

fn clock_gettime() -> Result<Box<timespec>, libc::c_int> {
    unsafe {
        let time_spec = libc::malloc(size_of::<timespec>()) as *mut libc::timespec;
        let errno = libc::clock_gettime(libc::CLOCK_PROCESS_CPUTIME_ID, time_spec);
        if errno != 0 {
            Err(errno)
        } else {
            Ok(Box::from_raw(time_spec))
        }
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
