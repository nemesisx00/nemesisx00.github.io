use ::dioxus::prelude::*;
use crate::components::route::Route;
use crate::data::{ContactEmail, GithubPageBaseUrl, SubtitleContent, TitleContent};

#[component]
pub fn Header() -> Element
{
	let name = TitleContent.to_owned();
	let job = SubtitleContent.to_owned();
	let email = ContactEmail.to_owned();
	let href = format!("mailto:{}", ContactEmail);
	let urlLabel = GithubPageBaseUrl[8..].to_string();
	
	return rsx!
	{
		header
		{
			h1 { "{name}" }
			h3 { "{job}" }
			
			h4
			{
				Link
				{
					to: Route::Home {},
					{urlLabel}
				}
			}
			
			h4
			{
				a
				{
					href: href,
					{email}
				}
			}
		}
	};
}
