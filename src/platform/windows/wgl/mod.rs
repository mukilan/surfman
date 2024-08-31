// surfman/src/platform/windows/wgl/mod.rs

//! A backend using the native Windows OpenGL WGL API.

use crate::implement_interfaces;

pub mod connection;
pub mod context;
pub mod device;
pub mod surface;

implement_interfaces!();

#[cfg(test)]
#[path = "../../../tests.rs"]
mod tests;
