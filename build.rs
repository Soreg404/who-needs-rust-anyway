use std::process::{ Command, exit };

fn main() {
	println!("starting build");
	
    /* 
     * compile raw-main.rs to object file (.obj)
     */
	let r = Command::new("rustc")
		.arg("--edition=2024")
		.arg("--emit=obj")
		.arg("-C")
		.arg("panic=abort")
		.arg("raw-main.rs")
		.status()
		.unwrap();

    r.success() || exit(r.code().unwrap_or(-1));

    /*
     * link raw-main.obj with rust std libraries
     */
	let r = Command::new("rustc")
		.arg("--print")
		.arg("target-libdir")
		.output()
		.unwrap();
    r.status.success() || exit(r.code().unwrap_or(-1));
    let libdir = PathBuf::from(r.stdout);
	println!("rustc --print target-libdir");
    println!("{libdir:?}");
	
	let s = Command::new("clang")
		// .arg("--verbose")
		.arg("raw-main.o")
		.arg(format!("{sysroot}/libcompiler_builtins-0cdac2e568b3eee7.rlib"))
		.arg(format!("{sysroot}/libcore-d378bf52e32e9198.rlib"))
		.arg(format!("{sysroot}/libpanic_abort-80cad03f5519e354.rlib"))
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
