#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

pub const TitleContent: &'static str = "Peter Lunneberg";
pub const SubtitleContent: &'static str = "Software Developer";
pub const HeaderContent: &'static str = r#""#;

pub const ContactEmail: &'static str = "plunn.collab@gmail.com";

pub const GithubPageBaseUrl: &'static str = "https://nemesisx00.github.io";
pub const GithubProfileUrl: &'static str = "https://github.com/nemesisx00";
pub const KofiProfileUrl: &'static str = "https://ko-fi.com/peterlunneberg";
pub const LiberapayProfileUrl: &'static str = "https://liberapay.com/plunn.dev/";

pub const RainDropDensityMax: usize = 500;
pub const RainDropDensityMin: usize = 50;

pub fn pageUrl(fragment: &str) -> String { return format!("{}{fragment}", GithubPageBaseUrl.to_owned()); }
pub fn projectUrl(fragment: &str) -> String { return format!("{}{fragment}", GithubProfileUrl.to_owned()); }

pub fn collectProjectData(cv: bool) -> Vec<ProjectData>
{
	let mut projects = vec![
		ProjectData
		{
			backgroundIsVideo: false,
			backgroundPath: "./images/ocsm-promo-zoom.png".into(),
			description: "Open Character Sheet Manager is an open source cross-platform desktop application for conveniently managing TTRPG character sheets.".into(),
			extended: r#"Open Character Sheet Manager is a free, open source,
cross-platform desktop application for conveniently managing TTRPG character
sheets. It is built using Godot Engine with scripts written in C#."#.into(),
			id: "1".into(),
			label: "OCSM".into(),
			target: "_blank".into(),
			title: "Open Character Sheet Manager".into(),
			url: projectUrl("/ocsm"),
			..Default::default()
		},
		
		ProjectData
		{
			description: "A library for parsing Infinity Engine game files written in Rust.".into(),
			extended: r#"Infinity Engine Parser is a free, open source library
for parsing Infinity Engine game files written in Rust. It is configured to
provide a Foreign Function Interface (FFI), via the safer-ffi crate, enabling
cross-language integration with other applications and libraries."#.into(),
			id: "2".into(),
			label: "Infinity Engine Parser".into(),
			target: "_blank".into(),
			url: projectUrl("/infinity-engine-parser"),
			..Default::default()
		},
		
		ProjectData
		{
			description: "A desktop GUI frontend for yt-dlp written in Rust using Dioxus.".into(),
			id: "3".into(),
			label: "Rust VDL".into(),
			target: "_blank".into(),
			url: projectUrl("/rust-vdl"),
			..Default::default()
		},
		
		ProjectData
		{
			backgroundIsVideo: true,
			backgroundPath: "./video/novas-run-teaser.webm".into(),
			backgroundPoster: "./images/novas-run-teaser-poster.png".into(),
			description: "A web-based 2D platformer written with React/Next.js.".into(),
			extended: r#"A web-based 2D platformer written in Typescript using
React and Next.js. The entirety of the game mechanics are custom built, using
HTML, CSS, and Typescript, rather than relying on an existing game engine or
library."#.into(),
			id: "4".into(),
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
			extended: r#"Dice Roller (Godot) is a free, open source, literal
dice rolling application I created in order to explore 3D physics in Godot
Engine 4. It was also an opportunity to explore writing Godot scripts using C++
via the official godot-cpp bindings. The project builds using CMake and Ninja.
Its Github Pages page was created using Rust and Dioxus."#.into(),
			id: "5".into(),
			label: "Dice Roller (Godot)".into(),
			target: "_blank".into(),
			url: pageUrl("/dice-roller-godot/"),
			..Default::default()
		},
	];
	
	if cv
	{
		projects.remove(2);
	}
	
	return projects;
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ProjectData
{
	pub backgroundIsVideo: bool,
	pub backgroundPath: String,
	pub backgroundPoster: String,
	pub description: String,
	pub extended: String,
	pub id: String,
	pub label: String,
	pub target: String,
	pub title: String,
	pub url: String,
}
