use std::collections::HashMap;

pub const UniversityName: &'static str = "Concordia University Wisconsin";
pub const UniversityLocation: &'static str = "Mequon, Wisconsin, USA";
pub const UniversityGraduated: usize = 2008;
pub const UniversityDegree: &'static str = "Bachelor of Science";
pub const UniversityMajor: &'static str = "Computer Science";
pub const UniversityMinor: &'static str = "Mathematics";

pub const SummaryText: &'static str = r#"I am a professional software developer with over a decade of experience designing, developing, and maintaining applications across a variety of technologies.
In addition, I excel at client interaction, learning new techniques, and problem solving."#;

pub const SummaryTextNorsk: &'static str = r#"Jeg er en profesjonell programvareutvikler med mer enn 10 års erfaring på tvers av en rekke teknologier.
Dessuten utmerker jeg meg på klientinteraksjon, læring av nye teknologier og problemløsning."#;

pub fn jobList() -> Vec<JobData>
{
	let list = vec![
		JobData
		{
			company: "Michigan State University".into(),
			department: Some("Biomedical Research Informatics Core".into()),
			location: "East Lansing, Michigan, USA".into(),
			job: "Information Technologist II".into(),
			start: "March 2012".into(),
			end: Some("December 2018".into()),
			technologies: vec![
				"C#".into(),
				"CSS".into(),
				"Git".into(),
				"HTML".into(),
				"Java".into(),
				"Javascript".into(),
				"MySQL".into(),
				"Node.js".into(),
				"PHP".into(),
				"PostgreSQL".into(),
				"REDCap".into(),
				"SQL".into(),
				"SQL Server".into(),
				"Subversion".into(),
			],
			text: r#"• Designed, implemented, and maintained functionality for multiple Electronic Data Capture web applications.
• Interfaced directly with clients to discuss project status and new feature requests.
• Implemented database driven data visualization, dynamically generating inline SVG via Javascript.
• Collaborated with data managers to ensure project requirements were being met.
• Maintained data security in accordance with HIPAA.
• Worked across the full stack."#.into(),
			..Default::default()
		},
		
		JobData
		{
			company: "Nerdwerx".into(),
			end: Some("March 2012".into()),
			job: "Software Developer".into(),
			location: "Davenport, Iowa, USA".into(),
			start: "August 2011".into(),
			technologies: vec![
				"CSS".into(),
				"HTML".into(),
				"MySQL".into(),
				"PHP".into(),
				"SQL".into(),
				"Wordpress".into(),
			],
			text: r#"• Designed, implemented, and maintained custom web applications.
• Worked across the full stack.
• Interfaced directly with clients.
• Mentored junior developer."#.into(),
			..Default::default()
		},
		
		JobData
		{
			company: "Smart Software Solutions, Inc".into(),
			end: Some("February 2011".into()),
			job: "Senior Software Developer".into(),
			location: "Pierre, South Dakota, USA".into(),
			start: "August 2008".into(),
			technologies: vec![
				"C#".into(),
				"CSS".into(),
				"HTML".into(),
				"Java".into(),
				"Javascript".into(),
				"Linux".into(),
				"MySQL".into(),
				"PHP".into(),
				"SQL".into(),
				"SQL Server".into(),
				"Subversion".into(),
			],
			text: r#"• Implemented and maintained new and existing web applications.
• Contributed to embedded system application.
• Contributed to API for managing SIP phone interaction.
• Participated in client interactions.
• Worked across the full stack."#.into(),
			..Default::default()
		},
	];
	
	return list;
}

pub fn languageSkillList() -> HashMap<String, Option<String>>
{
	return HashMap::from([
		("English".into(), Some("Mother Tongue".into())),
		("Norwegian".into(), Some("Basic".into())),
	]);
}

pub fn softSkillList() -> HashMap<String, Option<String>>
{
	return HashMap::from([
		("API Design".into(), None),
		("Client Interaction".into(), None),
		("Code Review".into(), None),
		("Coding Style".into(), None),
		("Database Design".into(), None),
		("Problem Solving".into(), None),
		("Project Estimation".into(), None),
		("Version Control".into(), None),
		("Team Collaboration".into(), None),
	]);
}

pub fn techSkillList() -> HashMap<String, Option<String>>
{
	let expert = Some("Expert".to_string());
	let intermediate = Some("Intermediate".to_string());
	
	return HashMap::from([
		("C#".into(), expert.clone()),
		("CSS".into(), expert.clone()),
		("Git".into(), expert.clone()),
		("Github CI".into(), intermediate.clone()),
		("Godot".into(), intermediate.clone()),
		("HTML".into(), expert.clone()),
		("Java".into(), intermediate.clone()),
		("Javascript".into(), expert.clone()),
		("MySQL / MariaDB".into(), expert.clone()),
		("PHP".into(), expert.clone()),
		("PostgreSQL".into(), expert.clone()),
		("Python".into(), intermediate.clone()),
		("Rust".into(), intermediate.clone()),
		("SQL".into(), expert.clone()),
		("SQL Server".into(), expert.clone()),
	]);
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct JobData
{
	pub company: String,
	pub department: Option<String>,
	pub end: Option<String>,
	pub job: String,
	pub location: String,
	pub start: String,
	pub technologies: Vec<String>,
	pub text: String,
}
