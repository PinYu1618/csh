mod home;
mod not_found;

pub use self::home::Home;
pub use self::not_found::NotFound;

// TODO: Remove this
mod demo;
pub use self::demo::Demo;