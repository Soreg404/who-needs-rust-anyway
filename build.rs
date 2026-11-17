#![allow(unused_imports)]
use std::process::{ Command, exit };
use std::path::PathBuf;

fn main() {
	println!("# starting build");
	
    println!("# compile raw-main.rs to obj file");
	let r = Command::new("rustc")
		.arg("--edition=2024")
		.arg("--emit=obj")
		.arg("-C")
		.arg("panic=abort")
		.arg("raw-main.rs")
		.status()
		.unwrap();
    _ = r.success() || exit(r.code().unwrap_or(-1));

    println!("# link raw-main.obj with rust std libraries");
	println!("## run `rustc --print target-libdir`");
	let r = Command::new("rustc")
		.arg("--print")
		.arg("target-libdir")
		.output()
		.unwrap();
    _ = r.status.success() || exit(r.status.code().unwrap_or(-1));
    let libdir = str::from_utf8(&r.stdout.trim_ascii())
        .unwrap()
        .trim();
    let libdir = PathBuf::from(libdir);
    println!("## libdir={libdir:?}");

    println!("# clang config");
	let mut clang = Command::new("clang");
    clang
		// .arg("--verbose")
		.arg("raw-main.o")
		.arg("-o")
		.arg("raw-main.exe");

    println!("## crude find libs to link (from libdir)");
    for entry in libdir.read_dir().unwrap() {
        let entry = entry.unwrap();
        let entry = entry.path();
        let fname = entry
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();
        let starts_with =
            fname.starts_with("libcompiler_builtins") ||
            fname.starts_with("libcore") ||
            fname.starts_with("libpanic_abort");
        let ends_with = entry.extension().unwrap() == "rlib";
        if starts_with && ends_with {
            println!("### link lib: {:?}", fname);
            clang.arg(entry);
        }
    }

    println!("# finish clang");
    let r = clang
		.status()
		.unwrap();
    _ = r.success() || exit(r.code().unwrap_or(-1));
	
	println!("# build success");
	
}
