#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

mod components;
mod data;
mod util;

use ::dioxus::prelude::*;
use crate::components::{BodyContent, PageHeader, ProjectList};
use crate::data::collectProjectData;

fn main()
{
	if cfg!(debug_assertions)
	{
		// init debug tool for WebAssembly
		::wasm_logger::init(wasm_logger::Config::default());
		::console_error_panic_hook::set_once();
	}
	
	::dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element
{
	return cx.render(rsx!
	{
		PageHeader {}
		ProjectList { projects: collectProjectData() }
		BodyContent {}
		
		footer { "© 2023 Peter Lunneberg" }
	});
}
