#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use ::rand::random;

pub fn rangeRandom(min: f64, max: f64) -> f64
{
	return transformRange(random(), 0.0, 1.0, min, max);
}

pub fn transformRange(x: f64, min1: f64, max1: f64, min2: f64, max2: f64) -> f64
{
	return min2 + ((max2 - min2) / (max1 - min1)) * (x - min1);
}

mod test
{
	#[allow(unused_imports)]
	use super::*;
	
	#[test]
	fn testRangeRandom()
	{
		let min = 25.0;
		let max = 75.0;
		
		let result = rangeRandom(min, max);
		assert!(result >= 25.0);
		assert!(result <= 75.0);
	}
	
	#[test]
	fn testTransformRange()
	{
		let x = 0.5;
		let min1 = 0.0;
		let max1 = 1.0;
		let min2 = 25.0;
		let max2 = 75.0;
		
		let expected = 50.0;
		let result = transformRange(x, min1, max1, min2, max2);
		
		assert_eq!(expected, result);
	}
}
