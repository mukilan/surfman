// surfman/surfman/src/platform/unix/generic/mod.rs
//
//! The Mesa "surfaceless" backend, which only supports off-screen surfaces and cannot directly
//! display surfaces on a screen.

pub mod connection;
pub mod context;
pub mod device;
pub mod surface;

crate::implement_interfaces!();

#[cfg(test)]
#[path = "../../../tests.rs"]
mod tests;
