#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use ::dioxus::prelude::*;
use ::dioxus_router::prelude::*;
use crate::components::route::Route;
use crate::data::{ContactEmail, GithubPageBaseUrl, TitleContent, SubtitleContent};

pub fn Header(cx: Scope) -> Element
{
	let name = TitleContent.to_owned();
	let job = SubtitleContent.to_owned();
	let email = ContactEmail.to_owned();
	let href = format!("mailto:{}", ContactEmail);
	let urlLabel = GithubPageBaseUrl[8..].to_string();
	
	return cx.render(rsx!
	{
		header
		{
			h1 { "{name}" }
			h3 { "{job}" }
			h4
			{
				Link
				{
					to: Route::Home {},
					"{urlLabel}"
				}
			}
			h4
			{
				a
				{
					href: "{href}",
					"{email}"
				}
			}
		}
	});
}
