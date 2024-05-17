use ::dioxus::prelude::*;
use ::rand::random;
use crate::data::{RainDropDensityMax, RainDropDensityMin};
use crate::util::rangeRandom;

#[component]
pub fn RainBackground() -> Element
{
	let density = rangeRandom(RainDropDensityMin as f64, RainDropDensityMax as f64) as usize;
	
	return rsx!
	{
		div
		{
			class: "rain",
			
			for i in 0..density
			{
				RainDrop { key: "{i}" }
			}
		}
	};
}

const RainDropPath: &str = "M 2.5,0 C 2.6949458,3.5392017 3.344765,20.524571 4.4494577,30.9559 5.7551357,42.666753 4.5915685,50 2.5,50 0.40843152,50 -0.75513565,42.666753 0.55054234,30.9559 1.655235,20.524571 2.3050542,3.5392017 2.5,0 Z";

#[component]
fn RainDrop() -> Element
{
	let data = RainDropData::random();
	let styles = data.toCss();
	
	return rsx!
	{
		svg
		{
			view_box: "0 0 5 50",
			style: "{styles}",
			
			path
			{
				d: "{RainDropPath}",
			}
		}
	};
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct RainDropData
{
	pub delay: f64,
	pub duration: f64,
	pub opacity: f64,
	pub scale: f64,
	pub x: f64,
	pub y: f64,
}

impl RainDropData
{
	pub fn random() -> Self
	{
		let mut instance = Self::default();
		instance.randomize();
		return instance;
	}
	
	pub fn randomize(&mut self)
	{
		self.delay = (random::<f64>() * 2.0) - 1.0;
		self.duration = random::<f64>() + 0.5;
		self.opacity = random();
		self.scale = random();
		self.x = random::<f64>() * 100.0;
		self.y = random::<f64>() * 100.0;
	}
	
	pub fn toCss(&self) -> String
	{
		let out = format!("--delay: {}; --duration: {}; --opacity: {}; --scale: {}; --x: {}; --y: {};",
			self.delay,
			self.duration,
			self.opacity,
			self.scale,
			self.x,
			self.y
		);
		return out;
	}
}
