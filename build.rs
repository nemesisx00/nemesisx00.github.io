#![allow(non_snake_case)]

use std::process::Command;

fn main()
{
	let mut program = "cmd";
	let mut firstArg = "/C";
	
	if !cfg!(windows)
	{
		program = "sh";
		firstArg = "-c";
	}
	
	Command::new(program)
		.args(&[firstArg, "npm run build"])
		.output()
		.expect("Failed to execute Stylus script");
	
	Command::new(program)
		.args(&[firstArg, "npm run build --prefix ./subprojects/novas-run"])
		.output()
		.expect("Failed to build Nova's Run");
	
	//Rebuild on Stylus update
	println!("cargo:rerun-if-changed=stylus/**/*.styl");
	
	//Rebuild on Nova's Run update
	_ = ["public/**/*.*", "src/**/*.*", "next.config.js", "package.json", "tsconfig.json"]
		.map(|frag| println!("cargo:rerun-if-changed=subprojects/novas-run/{}", frag));
}
