#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use ::dioxus::prelude::*;
use ::dioxus_router::prelude::Link;
use crate::data::ContactEmail;
use crate::components::route::Route;

pub fn BodyContent(cx: Scope) -> Element
{
	return cx.render(rsx!
	{
		div
		{
			id: "bodyContent",
			
			WhoAmI {}
			hr {}
			Credentials {}
			hr {}
			Collaborate { lookingForWork: true }
		}
	});
}

// --------------------------------------------------

/**
Call to action.
*/
#[inline_props]
fn Collaborate(cx: Scope, lookingForWork: bool) -> Element
{
	let emailHref = format!("mailto:{}", ContactEmail);
	
	return cx.render(rsx!
	{
		section
		{
			h1 { "WANT TO COLLABORATE?" }
			
			p
			{
				r#"Are you stuck on a task? Is there an algorithm or software
				design pattern that you would like to understand better? Or
				maybe you are looking to put together a team for a new project.
				I'm here to help! I can provide many software development
				services, including but not limited to:"#
			}
			
			ul
			{
				li { "Code Review" }
				li { "Design Consultation" }
				li { "Development Commissions" }
			}
			
			p
			{
				r#"Let's have a conversation about your particular needs."#
			}
			
			lookingForWork.then(move || rsx!
			{
				div
				{
					class: "lfw",
					
					p
					{
						r#"I am also currently looking for full time employment. If you
						could use a seasoned, highly skilled, and adaptable software
						developer, send me a description of the position you are looking
						to fill."#
					}
					
					//p { "I am only considering fully remote positions at this time." }
					
					p
					{
						class: "center",
						
						Link
						{
							class: "button cv",
							to: Route::Cv {},
							"View Resume"
						}
					}
				}
			})
			
			p
			{
				r#"In any case, if you could use my help, feel free to reach out!"#
			}
			
			a
			{
				class: "button",
				href: "{emailHref}",
				"Reach Out"
			}
		}
	});
}

/**
Why should you listen to me?
*/
fn Credentials(cx: Scope) -> Element
{
	return cx.render(rsx!
	{
		section
		{
			h1 { "WHERE CAN YOU SEE MY PORTFOLIO?" }
			
			p
			{
				r#"Honestly, you're already looking at it. This site you're
				viewing right now is an application which I built from scratch.
				I also created the videos and images for the scrolling panels
				above (which is why a couple of the projects don't have any
				unique images yet). If you haven't tried building a frontend
				with Dioxus yet, I highly recommend it!"#
			}
			
			p
			{
				r#"At the top of this page, I've listed my most prominent
				or relevant projects. When it comes to software development, my
				primary motivation is to help people, not making a profit. This
				is why I prefer creating free, open source applications."#
			}
			
			p
			{
				r#"You can find the rest on my Github profile. Many of the
				projects there were exploratory learning opportunities for me or
				simple demonstrations of my ability to work with a wide variety
				of technologies. A few them are very old projects that I built
				and actually used in my professional work back in the day. But
				taken together they represent my journey on the path of the
				software developer."#
			}
		}
	});
}

/**
Personal description: education/work history/interests/etc.
*/
fn WhoAmI(cx: Scope) -> Element
{
	return cx.render(rsx!
	{
		section
		{
			h1 { "WHO AM I?" }
			
			p
			{
				r#"I'm a gamer, software developer, and language enthusiast. Or,
				in a word, a nerd."#
			}
			
			p
			{
				r#"I started working as a professional software developer after
				graduating from university in 2008. Over the course of my career,
				I've designed, implemented, and maintained software applications
				using a myriad of frontend, backend, and database technologies."#
			}
			
			p
			{
				r#"The breadth of my experience has taught me an important lesson:
				It is much more useful to know how to find the answer than it is
				to already know the answer. Instead of trying to keep all the
				documentation for every existing language, library, and tool in
				my head all the time, I focus on knowing where to go looking
				when I need information. This is why I say with confidence that
				I can develop software with any technology, whether I've used it
				before or not."#
			}
			
			p
			{
				r#"In my free time, I tend to indulge in my passion for stories
				and storytelling. In addition to watching movies and shows, or
				playing narrative-driven video games, I enjoy participating in
				collaborative storytelling games, also known as tabletop
				roleplaying games (TTRPGs)."#
			}
			
			p
			{
				r#"Game development has always been a mountain I've wanted to
				climb but, in the past, I found it to be too intimidating. That
				is, until I had time to take a breath and reflect on my career,
				realizing just how far I had come. At the same time, I stumbled
				upon Godot, a completely free and open source game engine, which
				removed the final roadblock. I've been exploring game development
				for a few years now and plan to release at least one retail title
				in the near future."#
			}
		}
	});
}
