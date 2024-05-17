use std::collections::HashMap;
use ::dioxus::prelude::*;
use ::itertools::Itertools;
use super::data::{languageSkillList, softSkillList, techSkillList};

#[component]
pub fn Skills() -> Element
{
	let languages = languageSkillList();
	let softs = softSkillList();
	let techs = techSkillList();
	
	return rsx!
	{
		section
		{
			class: "skills",
			
			h1 { "Skills" }
			
			SkillGrid { title: "Languages".to_string(), skills: languages.to_owned(), four:false }
			SkillGrid { title: "Soft Skills".to_string(), skills: softs.to_owned(), four:false }
			SkillGrid { title: "Technical Skills".to_string(), skills: techs.to_owned(), four: false }
		}
	};
}

#[component]
fn SkillGrid(title: String, skills: HashMap<String, Option<String>>, four: bool) -> Element
{
	return rsx!
	{
		div
		{
			class: "skillGrid",
			
			h3 { "{title}" }
			div
			{
				for pair in skills.iter().sorted()
				{
					p
					{
						class: match four { true => "four", false => "", },
						
						h4 { "{pair.0}" }
						
						if let Some(val) = pair.1
						{
							h5 { "{val}" }
						}
					}
				}
			}
		}
	};
}
