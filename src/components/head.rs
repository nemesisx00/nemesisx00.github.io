#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use dioxus::prelude::*;
use crate::data::{
	GithubProfileUrl, KofiProfileUrl,
	HeaderContent, SubtitleContent, TitleContent
};

pub fn PageHeader(cx: Scope) -> Element
{
	return cx.render(rsx!
	{
		header
		{
			class: "content",
			"{HeaderContent}"
		}
	});
}

// --------------------------------------------------

pub fn PageTitles(cx: Scope) -> Element
{
	let githubUrl = GithubProfileUrl.to_owned();
	let kofiUrl = KofiProfileUrl.to_owned();
	
	return cx.render(rsx!
	{
		h1 { "{TitleContent}" }
		h4 { "{SubtitleContent}" }
		h4
		{
			id: "socialLinks",
			
			a
			{
				href: "{githubUrl}",
				target: "_blank",
				title: "Github Profile",
				
				img { alt: "Github", src: "./images/github-mark/github-mark-white.svg" }
			}
			
			a
			{
				href: "{kofiUrl}",
				target: "_blank",
				title: "Support me on Ko-fi!",
				
				img { alt: "Ko-fi", src: "" }
			}
		}
	});
}
