//! Implements some custom traits for `Stdin` to make reading console input simpler.

pub mod input;
pub mod read_hidden;

pub use input::Input;
pub use read_hidden::ReadHidden;
