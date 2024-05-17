use ::dioxus::prelude::*;
use super::data::{JobData, jobList};

#[component]
pub fn Experience() -> Element
{
	let jobs = jobList();
	
	return rsx!
	{
		section
		{
			class: "workExperience",
			
			h1 { "Work Experience" }
			
			for (i, job) in jobs.iter().enumerate()
			{
				Job { key: "{i}", id: i, job: job.clone() }
			}
		}
	};
}

#[component]
fn Job(id: usize, job: JobData) -> Element
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
			lines.push(rsx!(p { {line} }));
		}
	}
	
	return rsx!
	{
		div
		{
			class: "job",
			
			div
			{
				div
				{
					class: "left",
					
					h3 { {job.company} }
					{!department.is_empty()}.then(|| rsx!(h4 { {department} }))
					h5 { {job.location} }
					h4 { {job.job} }
				}
				
				div
				{
					class: "right",
					
					h3 { {time} }
				}
			}
			
			div
			{
				class: "technologies",
				
				for tech in job.technologies
				{
					p { {tech} }
				}
			}
			
			for line in lines
			{
				{line}
			}
		}
	};
}
