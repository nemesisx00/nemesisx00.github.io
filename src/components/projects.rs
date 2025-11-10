use std::collections::HashMap;
use ::dioxus::prelude::*;
use crate::data::{ProjectData, collectProjectData};
use super::rain::RainBackground;

#[component]
pub fn ProjectList() -> Element
{
	let projects = collectProjectData(false);
	let classes = buildInitialClassMap(projects.clone());
	
	return rsx!
	{
		div
		{
			id: "projectListContainer",
			
			for (index, project) in projects.iter().enumerate()
			{
				Project
				{
					key: "{project.id}",
					data: project.to_owned(),
					class: classes[&index].to_owned()
				}
			}
		}
	};
}

fn buildInitialClassMap(projects: Vec<ProjectData>) -> HashMap<usize, Option<String>>
{
	let mut map = HashMap::<usize, Option<String>>::default();
	for (index, _) in projects.iter().enumerate()
	{
		match index
		{
			0 => map.insert(index, Some("show".to_string())),
			1 => map.insert(index, Some("next show".to_string())),
			_ => match index == projects.len() - 1
				{
					true => map.insert(index, Some("previous show".to_string())),
					false => map.insert(index, None),
				},
		};
	}
	
	return map;
}

// --------------------------------------------------

#[component]
fn Project(data: ProjectData, #[props(!optional)] class: Option<String>) -> Element
{
	let mut cn = "project".to_string();
	if let Some(s) = class
	{
		cn = format!("{cn} {s}");
	}
	
	if data.background.is_none()
	{
		cn = format!("{cn} noBackground");
	}
	
	let target = match data.target.is_empty()
	{
		true => "_blank",
		false => data.target.as_str(),
	};
	
	return rsx!
	{
		a
		{
			class: "{cn}",
			href: "{data.url}",
			target: "{target}",
			title: "{data.label}",
			
			h1 { "{data.label}" }
			p { "{data.description}" }
			
			if data.background.is_some()
			{
				if data.backgroundIsVideo
				{
					video
					{
						autoplay: true,
						controls: false,
						r#loop: true,
						muted: true,
						playsinline: true,
						preload: true,
						poster: data.backgroundPoster.unwrap(),
						src: data.background.unwrap()
					}
				}
				else
				{
					img { src: data.background.unwrap() }
				}
			}
			else
			{
				RainBackground {}
			}
		}
	};
}
