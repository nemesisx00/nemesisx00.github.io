#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use ::dioxus::prelude::*;
use super::data::{SummaryText, skillList};

pub fn Summary(cx: Scope) -> Element
{
	return cx.render(rsx!
	{
		section
		{
			class: "summary",
			
			p { "{SummaryText}" }
			Skills {}
		}
	});
}

fn Skills(cx: Scope) -> Element
{
	let skills = skillList();
	
	return cx.render(rsx!
	{
		div
		{
			class: "skillList",
			
			for skill in skills
			{
				p { "{skill}" }
			}
		}
	});
}
