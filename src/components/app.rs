use ::dioxus::prelude::*;
use super::route::Route;

#[component]
pub fn App() -> Element
{
	return rsx!
	{
		Router::<Route> {}
	};
}
