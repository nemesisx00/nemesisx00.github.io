use ::dioxus::prelude::*;
use super::route::Route;

#[component]
pub fn App() -> Element
{
	static Css: Asset = asset!("/assets/main.css");
	static Scroller: Asset = asset!("/assets/js/scroller.js");
	static FavIcon: Asset = asset!("/assets/favicon.ico");
	
	return rsx!
	{
		document::Stylesheet
		{
			rel: "stylesheet",
			href: "https://cdn.jsdelivr.net/npm/fork-awesome@1.2.0/css/fork-awesome.min.css",
			integrity: "sha256-XoaMnoYC5TH6/+ihMEnospgm0J1PM/nioxbOUdnM8HY=",
			crossorigin: "anonymous"
		}
		document::Stylesheet { href: Css }
		document::Script { src: Scroller }
		document::Link { rel: "icon", type: "image/x-icon", href: FavIcon }
		
		Router::<Route> {}
	};
}
