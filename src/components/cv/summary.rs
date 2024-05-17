use ::dioxus::prelude::*;
use super::data::{SummaryText, SummaryTextNorsk};

#[component]
pub fn Summary() -> Element
{
	return rsx!
	{
		section
		{
			class: "summary",
			
			p { "{SummaryText}" }
			p { "{SummaryTextNorsk}" }
		}
	};
}
