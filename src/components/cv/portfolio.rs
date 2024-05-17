use ::dioxus::prelude::*;
use crate::data::{ProjectData, collectProjectData, pageUrl};

#[component]
pub fn Portfolio() -> Element
{
	let projects = collectProjectData(true);
	
	return rsx!
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
	};
}

#[component]
fn Project(project: ProjectData) -> Element
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
	
	return rsx!
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
	};
}
