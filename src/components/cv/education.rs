use ::dioxus::prelude::*;
use super::data::{UniversityDegree, UniversityGraduated, UniversityLocation, UniversityMajor, UniversityMinor, UniversityName};

#[component]
pub fn Education() -> Element
{
	return rsx!
	{
		section
		{
			class: "education",
			
			h1 { "Education" }
			
			div
			{
				h3 { "{UniversityDegree} in {UniversityMajor}" }
				h3 { "{UniversityName}" }
				h4 { "{UniversityLocation}" }
				h4 { "Minor in {UniversityMinor}" }
				h4 { "Graduated {UniversityGraduated}" }
			}
		}
	};
}
