#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

mod components;
mod data;
mod util;

use ::dioxus::prelude::*;
use ::tracing::Level;
use crate::components::App;

fn main()
{
	::dioxus_logger::init(Level::INFO).expect("failed to init logger");
	launch(App);
}
