mod components;
mod controllers;
mod fragments;
mod i18n;
mod models;
mod templates;

use i18n::I18n;
pub use controllers::index;
pub use controllers::insert;
pub use controllers::salaries;
pub use controllers::sort;
pub use controllers::validate;