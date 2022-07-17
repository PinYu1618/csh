mod app;
mod components;
mod pages;
mod route;
pub mod style;

pub use self::style::Theme;

use crate::app::App;

pub fn start() {
    yew::start_app::<App>();
}
