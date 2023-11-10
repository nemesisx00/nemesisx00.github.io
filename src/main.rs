#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

mod components;
mod data;
mod util;

use crate::components::App;

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
