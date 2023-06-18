#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

pub const TitleContent: &'static str = "Peter Lunneberg";
pub const SubtitleContent: &'static str = "Full Stack Software Developer";
pub const HeaderContent: &'static str = r#"🚧 Under Construction 🚧"#;

pub const GithubPageBaseUrl: &'static str = "https://nemesisx00.github.io";
pub const GithubProfileUrl: &'static str = "https://github.com/nemesisx00";
pub const KofiProfileUrl: &'static str = "https://ko-fi.com/peterlunneberg";
pub const LiberapayProfileUrl: &'static str = "https://liberapay.com/plunn.dev/";

fn pageUrl(fragment: &str) -> String { return format!("{}{fragment}", GithubPageBaseUrl.to_owned()); }
fn projectUrl(fragment: &str) -> String { return format!("{}{fragment}", GithubProfileUrl.to_owned()); }

pub fn collectProjectData() -> Vec<ProjectData>
{
	let projects = vec![
		ProjectData
		{
			backgroundIsVideo: false,
			backgroundPath: "./images/ocsm-promo-zoom.png".into(),
			description: "Open Character Sheet Manager is an open source cross-platform desktop application for conveniently managing TableTop RolePlaying Game character sheets for a wide variety of game systems.".into(),
			id: "1".into(),
			label: "OCSM".into(),
			url: projectUrl("/ocsm"),
			..Default::default()
		},
		
		ProjectData
		{
			description: "A desktop GUI frontend for yt-dlp written in Rust using Dioxus.".into(),
			id: "2".into(),
			label: "rust-vdl".into(),
			url: projectUrl("/rust-vdl"),
			..Default::default()
		},
		
		ProjectData
		{
			backgroundIsVideo: true,
			backgroundPath: "./video/novas-run-teaser.webm".into(),
			description: "A web-based 2D platformer written with React/Next.js.".into(),
			id: "3".into(),
			label: "Nova's Run".into(),
			target: "_self".into(),
			url: "./novas-run".into(),
			..Default::default()
		},
		
		ProjectData
		{
			backgroundIsVideo: true,
			backgroundPath: "./video/dice-roller-godot-20sec-loop.webm".into(),
			backgroundPoster: "./images/dice-roller-godot-poster.png".into(),
			description: "Exploring 3D physics with C++ and Godot 4 in a literal dice roller application.".into(),
			id: "4".into(),
			label: "Dice Roller (Godot)".into(),
			url: pageUrl("/dice-roller-godot/"),
			..Default::default()
		},
		/*
		ProjectData
		{
			description: "See what I'm up to on Github!".into(),
			id: "4".into(),
			label: "Github Profile".into(),
			url: GithubProfileUrl.to_owned(),
			..Default::default()
		},
		
		ProjectData
		{
			description: "Support me on Ko-fi!".into(),
			id: "5".into(),
			label: "Ko-fi".into(),
			url: KofiProfileUrl.to_owned(),
			..Default::default()
		},
		*/
	];
	
	return projects;
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ProjectData
{
	pub backgroundIsVideo: bool,
	pub backgroundPath: String,
	pub backgroundPoster: String,
	pub description: String,
	pub id: String,
	pub label: String,
	pub target: String,
	pub url: String,
}
