pub const TitleContent: &'static str = "Peter Lunneberg";
pub const SubtitleContent: &'static str = "Software Developer";
pub const HeaderContent: &'static str = r#""#;

pub const Name: &'static str = "Peter Lunneberg";
pub const ContactEmail: &'static str = "plunn.collab@gmail.com";
pub const Copywrite: &'static str = "© 2023";

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
			description: "An open source cross-platform desktop application for conveniently managing TTRPG character sheets.".into(),
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
			description: "A desktop application for archiving and viewing game achievements data.".into(),
			extended: r#"Local Achievements is a free, open source, cross-platform
			desktop application for archiving game achievements data from multiple
			different platforms and provides the ability to view the data offline.
			It is written entirely in Rust and utilizes the Freya GUI library."#.into(),
			id: "2".into(),
			label: "Local Achievements".into(),
			target: "_blank".into(),
			url: projectUrl("/local-achievements"),
			..Default::default()
		},
		
		ProjectData
		{
			description: "A library for parsing Infinity Engine game files written in Rust.".into(),
			extended: r#"Infinity Engine Parser is a free, open source library
written in Rust for parsing Infinity Engine game files. It is configured to
provide a Foreign Function Interface (FFI), via the safer-ffi crate, enabling
cross-language integration with other applications and libraries."#.into(),
			id: "3".into(),
			label: "Infinity Engine Parser".into(),
			target: "_blank".into(),
			url: projectUrl("/infinity-engine-parser"),
			..Default::default()
		},
		
		ProjectData
		{
			description: "A 3D chess game built in Godot.".into(),
			extended: r#"Godot Chess is a free, open source 3D chess game built
using Godot Engine with scripts written in C#. This project is a simple foundation
from which to explore Game AI implementation."#.into(),
			id: "4".into(),
			label: "Godot Chess".into(),
			target: "_blank".into(),
			url: projectUrl("/godot-chess"),
			..Default::default()
		},
		
		ProjectData
		{
			backgroundIsVideo: true,
			backgroundPath: "./video/novas-run-teaser.webm".into(),
			backgroundPoster: "./images/novas-run-teaser-poster.png".into(),
			description: "A web-based 2D platformer written with React/Next.js.".into(),
			extended: r#"Nova's Run is a web-based 2D platformer written in
Typescript using React and Next.js. The entirety of the game mechanics are custom
built, using HTML, CSS, and Typescript, rather than relying on an existing game
engine or library."#.into(),
			id: "5".into(),
			label: "Nova's Run".into(),
			target: "_self".into(),
			url: pageUrl("/novas-run"),
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
			id: "6".into(),
			label: "Dice Roller (Godot)".into(),
			target: "_blank".into(),
			url: pageUrl("/dice-roller-godot"),
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
