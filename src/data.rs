#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

pub const TitleContent: &'static str = "Peter Lunneberg";
pub const SubtitleContent: &'static str = "Full Stack Software Developer";
pub const HeaderContent: &'static str = r#""#;

pub const GithubPageBaseUrl: &'static str = "https://nemesisx00.github.io";
pub const GithubProfileUrl: &'static str = "https://github.com/nemesisx00";
pub const KofiProfileUrl: &'static str = "https://ko-fi.com/U7U1HEKZ9";

fn pageUrl(fragment: &str) -> String
{
	return format!("{}{fragment}", GithubPageBaseUrl.to_owned());
}

fn projectUrl(fragment: &str) -> String
{
	return format!("{}{fragment}", GithubProfileUrl.to_owned());
}

pub fn collectProjectData() -> Vec<ProjectData>
{
	let ocsm = ProjectData
	{
		description: "Open Character Sheet Manager".into(),
		id: "1".into(),
		label: "OCSM".into(),
		url: projectUrl("/ocsm"),
	};
	
	let diceRollerGodot = ProjectData
	{
		id: "2".into(),
		label: "Dice Roller (Godot)".into(),
		url: pageUrl("/dice-roller-godot/"),
		..Default::default()
	};
	
	let rustVdl = ProjectData
	{
		id: "3".into(),
		label: "rust-vdl".into(),
		url: projectUrl("/rust-vdl"),
		..Default::default()
	};
	
	let github = ProjectData
	{
		id: "4".into(),
		label: "Github Profile".into(),
		url: GithubProfileUrl.to_owned(),
		..Default::default()
	};
	
	let kofi = ProjectData
	{
		description: "Support me on Ko-fi!".into(),
		id: "5".into(),
		label: "Ko-fi".into(),
		url: KofiProfileUrl.to_owned(),
	};
	
	let projects = vec![ocsm, diceRollerGodot, rustVdl, github, kofi];
	return projects;
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ProjectData
{
	pub description: String,
	pub id: String,
	pub label: String,
	pub url: String,
}
