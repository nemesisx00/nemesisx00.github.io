#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use ::dioxus::prelude::*;
use ::dioxus_router::prelude::*;
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

pub fn Home(cx: Scope) -> Element
{
	return cx.render(rsx!
	{
		PageHeader {}
		ProjectList {}
		BodyContent {}
		
		footer { format!("{} {}", Copywrite, Name) }
	});
}

#[inline_props]
pub fn Cv(cx: Scope) -> Element
{
	return cx.render(rsx!
	{
		CvPage {}
		
		footer { format!("{} {}", Copywrite, Name) }
	});
}
