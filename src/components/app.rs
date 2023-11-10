#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use ::dioxus::prelude::*;
use ::dioxus_router::prelude::*;
use super::route::Route;

pub fn App(cx: Scope) -> Element
{
	return cx.render(rsx!
	{
		Router::<Route> {}
	});
}
