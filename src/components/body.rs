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
			h1 { "Want to Collaborate?" }
			
			p
			{
				r#"I am available to provide code review and tutoring services, likely
				for a modest fee. I would also love to contribute to your open source
				project."#
			}
			
			(lookingForWork).then(move || rsx!(p
			{
				r#"I am currently available for employment. If you're looking for a highly
				skilled and adaptable full stack software developer, send me a description
				of your company or project and the position you want to fill. I am only
				considering fully remote positions at this time."#
			}))
			
			p
			{
				r#"I'm also open to collaboration on things other than software development.
				For example, if you're putting together a TTRPG Actual Play series, I
				don't think you would be surprised to learn that, like most enthusiasts,
				I have created many more character concepts than I have had the opportunity
				to play."#
			}
			
			p { "In any case, if you're interested in working with me, feel free to reach out!" }
			a { class: "button", href: "mailto:plunn.collab@gmail.com", "Contact Me" }
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
			h1 { "What is a Generalist Polyglot?" }
			
			p
			{
				r#"I refer to myself as an intentional generalist. By this I mean I am
				practiced in rapidly assimilating new information and learning to use new
				technologies. In general, I can get up to speed on a completely new-to-me
				technology, be it a library, framework, or whole programming language,
				in a day or two at most. And, to be clear, I simply learn to use it,
				leveraging my more than 15 years of experience learning new things rather
				than requiring explicit training."#
			}
			
			p
			{
				r#"I also consider myself to be a polyglot. I'm not referring to my
				conversational fluency in Norwegian, my having dabbled in 10 or more other
				spoken languages, nor my ability to read several non-Latin writing systems
				(Greek, Korean Hangul, Japanese kana, Russian and Ukrainian Cyrillic) in
				spite of not always knowing the meaning behind the words."#
			}
			
			p
			{
				r#"A programming language is just that: a language. And I am fluent
				in all of them, even the ones I haven't used before."#
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
			
			p { "I'm a gamer, software developer, and language enthusiast. Or, in a word, a nerd." }
			
			p
			{
				r#"I started working as a professional software developer after graduating
				from university back in 2008. Over the course of my career, I've designed,
				implemented, and maintained software applications using a myriad of frontend,
				backend, and database technologies."#
			}
			
			p
			{
				r#"On my own time, I prefer to explore areas of software development
				that are outside the domains of my professional work. These have often
				skewed towards desktop GUI applications and game development. I also
				enjoy trying out new, or new-to-me, technologies. After all, there is
				always more to learn."#
			}
			
			p
			{
				r#"However, everyone needs to take a step back from the grind and relax
				from time to time. I prefer immersing myself in engaging stories and
				interactive storytelling. This can range from watching movies or TV
				shows to joining a weekly session playing collaborative storytelling
				games, more commonly known as TTRPGs."#
			}
		}
	});
}
