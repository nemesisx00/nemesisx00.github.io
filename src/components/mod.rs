#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

mod app;
mod body;
mod cv;
mod head;
mod projects;
mod rain;
mod route;

pub use app::App;

pub use body::BodyContent;
pub use cv::CvPage;
pub use head::{PageHeader, PageTitles};
pub use projects::ProjectList;
