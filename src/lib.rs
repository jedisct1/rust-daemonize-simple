#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub use unix::*;

#[cfg(not(unix))]
mod not_unix;
#[cfg(not(unix))]
pub use not_unix::*;
