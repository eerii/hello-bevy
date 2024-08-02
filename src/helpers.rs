//! Global helper functions and macros.

/// Gets a single `Entity` from a `Query` or returns gracefully (no panic).
#[macro_export]
macro_rules! single {
    ($q:expr, $r:expr) => {
        match $q.get_single() {
            Ok(m) => m,
            _ => {
                debug!("get single failed for ${}", stringify!($e));
                $r
            },
        }
    };
    ($q:expr) => {
        single!($q, return)
    };
}
