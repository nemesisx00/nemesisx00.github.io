#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use ::dioxus::prelude::*;
use super::head::Header;
use super::summary::Summary;
use super::education::Education;
use super::experience::Experience;
use super::portfolio::Portfolio;

pub fn CvPage(cx: Scope) -> Element
{
	return cx.render(rsx!
	{
		div
		{
			class: "cv",
			
			Header {}
			hr {}
			Summary {}
			hr {}
			Portfolio {}
			hr {}
			Experience {}
			hr {}
			Education {}
		}
	});
}
