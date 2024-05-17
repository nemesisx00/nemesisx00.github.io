use ::dioxus::prelude::*;
use crate::data::{Copywrite, Name};
use super::cv::CvPage;
use super::head::PageHeader;
use super::body::BodyContent;
use super::projects::ProjectList;

#[derive(Clone, Routable)]
pub enum Route
{
	#[route("/")]
	Home {},
	#[route("/cv")]
	Cv {},
}

#[component]
pub fn Home() -> Element
{
	let footerText = format!("{} {}", Copywrite, Name);
	
	return rsx!
	{
		PageHeader {}
		ProjectList {}
		BodyContent {}
		
		footer { {footerText} }
	};
}

#[component]
pub fn Cv() -> Element
{
	let footerText = format!("{} {}", Copywrite, Name);
	
	return rsx!
	{
		CvPage {}
		
		footer { {footerText} }
	};
}
