#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use super::experience::JobData;

pub const UniversityName: &'static str = "Concordia University Wisconsin";
pub const UniversityGraduated: usize = 2008;
pub const UniversityDegree: &'static str = "B.S. in Computer Science";

pub const SummaryText: &'static str = r#"Professional software developer with over a decade of experience designing, developing, and maintaining applications across a variety of technologies.
Excels at client interaction, learning new techniques, and problem solving."#;

pub fn jobList() -> Vec<JobData>
{
	let list = vec![
		JobData
		{
			company: "Michigan State University".into(),
			department: Some("Biomedical Research Informatics Core".into()),
			location: "East Lansing, MI".into(),
			job: "Information Technologist II".into(),
			start: "March 2012".into(),
			end: Some("December 2018".into()),
			text: r#"As a part of the Biomedical Research Informatics Core (BRIC), designed, implemented, and maintained functionality for multiple Electronic Data Capture (EDC) web applications across multiple technology stacks:
	• PHP/MySQL
	• Java/PostgreSQL
	• C#/SQL Server

Interfaced directly with clients to discuss project status and new feature requests.
Implemented database driven data visualization by generating inline SVG via Javascript.
Obtained HIPAA certification and ensured the applications were HIPAA-compliant."#.into(),
			..Default::default()
		},
		
		JobData
		{
			company: "Nerdwerx".into(),
			location: "Davenport, IA".into(),
			job: "Software Developer".into(),
			start: "August 2011".into(),
			end: Some("March 2012".into()),
			text: r#"Full stack web development at a web consulting agency, primarily worked with PHP/MySQL and WordPress.
Interfaced directly with clients."#.into(),
			..Default::default()
		},
		
		JobData
		{
			company: "Smart Software Solutions, Inc".into(),
			location: "Pierre, SD".into(),
			job: "Senior Software Developer".into(),
			start: "August 2008".into(),
			end: Some("February 2011".into()),
			text: r#"Full stack web development at a web consulting company.
Worked across multiple technology stacks:
	• Java/MySQL
	• PHP/MySQL
	• C#/SQL Server

Some embedded systems and hardware interaction experience.
Involved in client interactions."#.into(),
			..Default::default()
		},
	];
	
	return list;
}

pub fn skillList() -> Vec<String>
{
	let list = vec![
		"Frontend Development".into(),
		"Backend Development".into(),
		"Database Design".into(),
		"Client Interactions".into(),
		"Code Review".into(),
		"Data Visualization".into(),
		"C#".into(),
		"Javascript".into(),
		"Rust".into(),
		"MySQL".into(),
		"PostgreSQL".into(),
		"SQL Server".into(),
	];
	
	return list;
}
