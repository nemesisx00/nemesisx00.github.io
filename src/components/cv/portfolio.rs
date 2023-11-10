#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use ::dioxus::prelude::*;
use crate::data::{ProjectData, collectProjectData, pageUrl};


pub fn Portfolio(cx: Scope) -> Element
{
	let projects = collectProjectData(true);
	
	return cx.render(rsx!
	{
		section
		{
			class: "portfolio",
			
			h1 { "Portfolio" }
			
			for project in projects
			{
				Project { project: project.to_owned() }
			}
		}
	});
}

#[inline_props]
fn Project(cx: Scope, project: ProjectData) -> Element
{
	let label = match project.title.is_empty()
	{
		true => project.label.to_owned(),
		false => project.title.to_owned(),
	};
	
	let description = match project.extended.is_empty()
	{
		true => project.description.to_owned(),
		false => project.extended.to_owned(),
	};
	
	let url = match project.url.starts_with("./")
	{
		true => pageUrl(&project.url[1..]),
		false => project.url.to_owned(),
	};
	
	return cx.render(rsx!
	{
		div
		{
			class: "project",
			
			div
			{
				h3 { "{label}" }
				
				h4
				{
					a
					{
						href: "{project.url}",
						"{url}"
					}
				}
			}
			
			p { "{description}" }
		}
	});
}
