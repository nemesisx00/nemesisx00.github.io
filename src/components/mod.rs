#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

mod body;
mod head;
mod projects;

pub use body::BodyContent;
pub use head::{PageHeader, PageTitles};
pub use projects::ProjectList;
