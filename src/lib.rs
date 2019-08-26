use criterion::measurement::{Measurement, ValueFormatter};
use std::time::Duration;

mod formatter;
mod time;

use formatter::DurationFormatter;

pub enum PosixTime {
    UserTime,
    UserAndSystemTime,
}
impl Measurement for PosixTime {
    type Intermediate = Duration;
    type Value = Duration;

    fn start(&self) -> Self::Intermediate {
        self.get_time()
    }

    fn end(&self, i: Self::Intermediate) -> Self::Value {
        self.get_time() - i
    }

    fn add(&self, v1: &Self::Value, v2: &Self::Value) -> Self::Value {
        *v1 + *v2
    }

    fn zero(&self) -> Self::Value {
        Duration::from_secs(0)
    }

    fn to_f64(&self, value: &Self::Value) -> f64 {
        value.as_nanos() as f64
    }

    fn formatter(&self) -> &dyn ValueFormatter {
        &DurationFormatter
    }
}
