/// A marker trait specifying what `access` means.
pub trait Access {}

/// Shared (&T) access to a pointed to type.
#[derive(Copy, Clone)]
pub enum Shared {}

/// Unique (&mut T) access to a pointed to type.
// -- NO #[derive(Copy, Clone)] FOR UNIQUE ACCESS!! --
pub enum Unique {}

impl Access for Shared {}
impl Access for Unique {}
