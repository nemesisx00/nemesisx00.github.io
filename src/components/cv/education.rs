#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use ::dioxus::prelude::*;
use super::data::{UniversityDegree, UniversityGraduated, UniversityName};

pub fn Education(cx: Scope) -> Element
{
	return cx.render(rsx!
	{
		section
		{
			class: "education",
			
			h2 { "{UniversityDegree}" }
			h3 { "{UniversityName}" }
			h4 { "Graduated {UniversityGraduated}" }
		}
	});
}
