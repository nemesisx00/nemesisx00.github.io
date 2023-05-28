#![allow(non_snake_case)]

use std::process::Command;

fn main()
{
	let mut program = "cmd";
	let mut firstArg = "/C";
	
	if !cfg!(target_os = "windows")
	{
		program = "sh";
		firstArg = "-c";
	}
	
	Command::new(program)
		.args(&[firstArg, "npm run stylus"])
		.output()
		.expect("Failed to execute Stylus script");
	
	
	Command::new(program)
		.args(&[firstArg, "npm run build --prefix ./subprojects/novas-run"])
		.output()
		.expect("Failed to build Nova's Run");
	
	//Just always re-run this script
	println!("cargo:rerun-if-changed=stylus/**/*.styl");
	
	println!("cargo:rerun-if-changed=subprojects/novas-run/public/**/*.*");
	println!("cargo:rerun-if-changed=subprojects/novas-run/src/**/*.*");
	println!("cargo:rerun-if-changed=subprojects/novas-run/next.config.js");
	println!("cargo:rerun-if-changed=subprojects/novas-run/package.json");
	println!("cargo:rerun-if-changed=subprojects/novas-run/tsconfig.json");
}
