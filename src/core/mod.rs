#[cfg(all(target_os = "linux", target_arch = "arm"))]
pub mod display_arm;
#[cfg(any(windows, target_os = "macos"))]
pub mod display_mock;
pub mod display_trait;
pub mod display_style;