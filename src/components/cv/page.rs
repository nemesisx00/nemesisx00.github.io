use ::dioxus::prelude::*;
use super::head::Header;
use super::summary::Summary;
use super::education::Education;
use super::experience::Experience;
use super::portfolio::Portfolio;
use super::skill::Skills;

#[component]
pub fn CvPage() -> Element
{
	return rsx!
	{
		div
		{
			class: "cv",
			
			Header {}
			hr {}
			Summary {}
			hr {}
			Portfolio {}
			hr {}
			Experience {}
			hr {}
			Skills {}
			hr {}
			Education {}
		}
	};
}
