#[cfg(target_os = "unix")]
mod unix;
#[cfg(target_os = "unix")]
pub use unix::*;

#[cfg(not(target_os = "unix"))]
mod not_unix;
#[cfg(not(target_os = "unix"))]
pub use not_unix::*;
