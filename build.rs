use std::{
	fs::{
		self,
		File,
	},
	io::{
		self,
		prelude::*,
	},
	path::PathBuf,
};

fn main() {
	let fns_vec = fs::read_dir("./src").expect("couldn't check the source directory")
		.map(|res| res.map(|e| e.path()))
		.collect::<Result<Vec<_>,io::Error>>().expect("couldn't check all files");

	let mut fns_iter = fns_vec.iter();

	loop {
		let item: PathBuf = match fns_iter.next() {
			Some(item) => item.to_path_buf(),
			None => break,
		};
		if item.extension().unwrap() == "rs" {
			let mut file = File::open(item).expect("couldn't open file");
			let mut contents = String::new();
			file.read_to_string(&mut contents).unwrap();
			contents = contents.escape_debug().to_string();
			if contents.contains("let estradiol") || contents.contains("let _estradiol") || contents.contains("let mut _estradiol") || contents.contains("let mut estradiol") {
			    println!("cargo:warning={}", "it seems you're wanting estrogen... here, have some:");
			    println!("cargo:warning={}", "           ..',;:clllooooooollc:;;,'..            ");
			    println!("cargo:warning={}", "      .';:lodddddddddxxxxxdxxxddddddolc;,..       ");
			    println!("cargo:warning={}", "   ';coddxxxxxxxxxxxxdxxdddxxxxxxxxxxxxxddl:,.    ");
			    println!("cargo:warning={}", " .codxxxxxxxdddddddxdodddddddddddddxddxxxxxxdo:'  ");
			    println!("cargo:warning={}", "'ldxxxxxdddddddddddxdodxdddddddddddddddddxxxxxdo:.");
			    println!("cargo:warning={}", "ldxkxxdddddddddddddxdoddoddddddddddddddddddxxxxdd:");
			    println!("cargo:warning={}", "odxxxxdddddddddddddxdoodddooodddddddddddddddxxxdoc");
			    println!("cargo:warning={}", "lodxxddddddddddddddxdodxdddooddddddddddddddddxxdoc");
			    println!("cargo:warning={}", ",loddddddddddddddddddooddddooddddddddddddddddddoo;");
			    println!("cargo:warning={}", " 'cooooodddddddddddddooooooodddddddddddddddddool;.");
			    println!("cargo:warning={}", "  .':looooooooddddddddddddddddddddddoooooooool:'  ");
			    println!("cargo:warning={}", "     ..,:clllllooooooooooooooooooooooollll:;'.    ");
			    println!("cargo:warning={}", "         ..',;:clllllllllllllllllllc::,'..        ");
			    println!("cargo:warning={}", "              ..',;:ccccllllccc:;,'..             ");
			    println!("cargo:warning={}", "");
			    println!("cargo:warning={}", "good girl");
			} else if contents.contains("let testosterone") || contents.contains("let _testosterone") || contents.contains("let mut _testosterone") || contents.contains("let mut testosterone") {
			    compile_error!("no testosterone for you!");
			}
		}		
	}
}
