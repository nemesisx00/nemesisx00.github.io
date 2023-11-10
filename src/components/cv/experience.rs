#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use ::dioxus::prelude::*;
use super::data::jobList;

pub fn Experience(cx: Scope) -> Element
{
	let jobs = jobList();
	
	return cx.render(rsx!
	{
		section
		{
			class: "workExperience",
			
			h1 { "Work Experience" }
			
			for (i, job) in jobs.iter().enumerate()
			{
				Job { key: "{i}", job: job.clone() }
			}
		}
	});
}

#[inline_props]
fn Job(cx: Scope, job: JobData) -> Element
{
	let end = match &job.end
	{
		None => "Present".into(),
		Some(s) => s.to_owned(),
	};
	
	let time = format!("{} - {}", job.start, end);
	let department = match &job.department
	{
		None => String::new(),
		Some(d) => d.to_owned(),
	};
	
	let mut lines = vec![];
	for line in job.text.split("\n")
	{
		if !line.is_empty()
		{
			match line.contains("•")
			{
				true => {
					lines.push(rsx!(p
					{
						span {}
						"{line}"
					}));
				},
				false => lines.push(rsx!(p { "{line}" })),
			}
		}
	}
	
	return cx.render(rsx!
	{
		div
		{
			class: "job",
			
			div
			{
				div
				{
					class: "left",
					
					h3 { "{job.company}" }
					(!department.is_empty()).then(|| rsx!(h4 { "{department}" }))
					h5 { "{job.location}" }
					h4 { "{job.job}" }
				}
				
				div
				{
					class: "right",
					
					h3 { "{time}" }
				}
			}
			
			for line in lines
			{
				line
			}
		}
	});
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct JobData
{
	pub company: String,
	pub department: Option<String>,
	pub location: String,
	pub job: String,
	pub start: String,
	pub end: Option<String>,
	pub text: String,
}
