#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use dioxus::prelude::*;

pub fn BodyContent(cx: Scope) -> Element
{
	return cx.render(rsx!
	{
		div
		{
			id: "bodyContent",
			
			WhoAmI {}
			hr {}
			GeneralistPolyglot {}
			hr {}
			Collaborate { lookingForWork: true }
		}
	});
}

// --------------------------------------------------

#[inline_props]
fn Collaborate(cx: Scope, lookingForWork: bool) -> Element
{
	return cx.render(rsx!
	{
		section
		{
			h1 { "Want To Collaborate With Me?" }
			
			p
			{
				r#"Are you stuck on a task? Is there an algorithm or software
				design pattern that you would like to understand better? Or
				maybe you are looking to put together a team for a new project."#
			}
			
			p
			{
				r#"Let me help! I am available to provide code review and tutoring
				services, as well as contribute to open source projects that can
				capture my interest. Let's have a conversation about the details
				of your particular needs."#
			}
			
			(lookingForWork).then(move || rsx!(p
			{
				r#"I am also currently looking for employment. If you could use
				a seasoned, highly skilled, and adaptable full stack software
				developer, send me a description of the position you are looking
				to fill. Note: I am only considering fully remote positions at
				this time."#
			}))
			
			p
			{
				r#"In any case, if you're interested in working with me, feel
				free to reach out!"#
			}
			
			a
			{
				class: "button",
				href: "mailto:plunn.collab@gmail.com",
				"Reach Out"
			}
		}
	});
}

// --------------------------------------------------

fn GeneralistPolyglot(cx: Scope) -> Element
{
	return cx.render(rsx!
	{
		section
		{
			h1 { "What Do I Mean By Generalist Polyglot Developer?" }
			
			p
			{
				r#"I consider myself to be a polyglot. I'm not referring to my
				conversational fluency in Norwegian, my having dabbled in 10 or
				more other spoken languages, nor my ability to read several
				non-Latin writing systems phonetically."#
			}
			
			p
			{
				r#"A programming language is just that: a language. It is a set
				of grammar and syntax which provides a means of communicating
				with a computer. As with spoken language, when I learn a new
				programming language I intuitively bring over understanding from
				all the other languages with which I am familiar, dramatically
				reducing the time needed to attain fluency."#
			}
			
			p
			{
				r#"Similarly to learning a language, gaining proficiency with
				any new technology is a process which becomes easier the more
				you do it. Over the course of my career as a software developer
				I have become exceedingly efficient at this process, to the
				point of being able to start working with a completely new-to-me
				technology almost immediately. This is why I began referring to
				myself as an intentional generalist."#
			}
			
			p
			{
				r#"As a result of all of this, I know I can work with any software
				development technology regardless of whether or not I've used it
				before."#
			}
		}
	});
}

// --------------------------------------------------

fn WhoAmI(cx: Scope) -> Element
{
	return cx.render(rsx!
	{
		section
		{
			h1 { "Who Am I?" }
			
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
				r#"On my own time, I prefer to explore areas of software development
				that are outside the domains of my professional work. Most notably,
				I have a passion for game development. I also enjoy trying out
				new, or new-to-me, technologies. After all, there is always more
				to learn."#
			}
			
			p
			{
				r#"However, everyone needs to take a step back from the grind
				and relax from time to time. I prefer immersing myself in engaging
				stories and interactive storytelling, whether in the form of
				movies or TV shows, video games, or collaborative storytelling
				sessions (i.e. TTRPGs)."#
			}
		}
	});
}
