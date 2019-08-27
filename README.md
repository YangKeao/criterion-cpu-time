# criterion-cpu-time

As criterion supports [custom measurement](https://bheisler.github.io/criterion.rs/book/user_guide/custom_measurements.html) now, we can use POSIX CPU time rather than wall time now!

## Implementation

Use `getrusage` to get currently used time.
