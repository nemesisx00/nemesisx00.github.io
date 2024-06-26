use ::dioxus::prelude::*;
use crate::data::{
	GithubProfileUrl, KofiProfileUrl, LiberapayProfileUrl,
	HeaderContent, SubtitleContent, TitleContent,
	collectProjectData
};
use super::route::Route;

#[component]
pub fn PageHeader() -> Element
{
	let projects = collectProjectData(false);
	
	return rsx!
	{
		header
		{
			class: "content",
			
			PageTitles {}
			
			nav
			{
				id: "mainNav",
				
				for (index, project) in projects.iter().enumerate()
				{
					{index > 0}.then(|| rsx!(div { class: "navSpacer" }))
					
					a
					{
						key: "{index}",
						href: "{project.url}",
						target: "{project.target}",
						"{project.label}"
					}
				}
			}
			
			{!HeaderContent.is_empty()}.then(|| rsx!(p { class: "headerContent", "{HeaderContent}" }))
		}
	};
}

// --------------------------------------------------

#[component]
pub fn PageTitles() -> Element
{
	let githubUrl = GithubProfileUrl.to_owned();
	let kofiUrl = KofiProfileUrl.to_owned();
	let liberapayUrl = LiberapayProfileUrl.to_owned();
	
	return rsx!
	{
		h1 { "{TitleContent}" }
		h4 { "{SubtitleContent}" }
		h5
		{
			Link
			{
				class: "cv",
				to: Route::Cv {},
				"View Resume"
			}
		}
		h4
		{
			id: "socialLinks",
			
			a
			{
				href: "{githubUrl}",
				target: "_blank",
				title: "Github Profile",
				
				img
				{
					alt: "Github",
					src: "./images/third-party/github-mark/github-mark-white.svg"
				}
			}
			
			a
			{
				href: "{kofiUrl}",
				target: "_blank",
				title: "Support me on Ko-fi!",
				
				img
				{
					alt: "Ko-fi",
					src: "./images/third-party/kofi-logo-smol.png"
				}
			}
			
			a
			{
				class: "liberapay",
				href: "{liberapayUrl}",
				target: "_blank",
				title: "Donate on Liberapay!",
				
				i
				{
					class: "fa fa-liberapay",
					aria_hidden: "true"
				}
			}
		}
	};
}
