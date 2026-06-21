use std::process::{ Command, exit };

fn main() {
	println!("starting build");
	
	let s = Command::new("rustc")
		.arg("--edition=2024")
		.arg("--emit=obj")
		.arg("-C")
		.arg("panic=abort")
		.arg("raw-main.rs")
		.status()
		.unwrap();
	if !s.success() {
		exit(s.code().unwrap_or(-2));
	}
	
	let sysroot = Command::new("rustc")
		.arg("--print")
		.arg("sysroot")
		.output()
		.unwrap()
		.stdout;
	let sysroot = String::from_utf8_lossy(sysroot.trim_ascii());
	println!("rustc sysroot: {sysroot:?}");
	
	let s = Command::new("clang")
		// .arg("--verbose")
		.arg("raw-main.o")
		.arg(format!("{sysroot}/lib/rustlib/x86_64-pc-windows-msvc/lib/libcompiler_builtins-0cdac2e568b3eee7.rlib"))
		.arg(format!("{sysroot}/lib/rustlib/x86_64-pc-windows-msvc/lib/libcore-d378bf52e32e9198.rlib"))
		.arg(format!("{sysroot}/lib/rustlib/x86_64-pc-windows-msvc/lib/libpanic_abort-80cad03f5519e354.rlib"))
		.arg("-o")
		.arg("raw-main.exe")
		.spawn()
		.unwrap()
		.wait()
		.unwrap();
	if !s.success() {
		exit(s.code().unwrap_or(-2));
	}
	
	println!("build success");
	
}