mod components;
mod controllers;
mod fragments;
mod i18n;
mod models;
mod pages;

pub use controllers::graphs;
pub use controllers::index;
pub use controllers::insert;
pub use controllers::maintenance;
pub use controllers::no_insert;
pub use controllers::not_found;
pub use controllers::notification;
pub use controllers::sort;
pub use controllers::validate;
use i18n::I18n;
