#![no_std] // requires explicit #[panic_handler]
#![no_main] // fn main requires rust std

unsafe extern "C" {	
	// return type of "!" means "never returns"
	// panic!(), todo!() and unreachable!() macros also return this "type"
	fn exit(_: i32) -> !;
	
	fn printf(fmt: *const core::ffi::c_char, ...) -> i32;
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
	unsafe {
		exit(-1)
	}
}

// normal, C-like main
#[unsafe(no_mangle)]
fn main(argc: i32, argv: *const *const u8) {
	unsafe {
		printf(c"hello world!\n".as_ptr());
		printf(c"num args: %i\n".as_ptr(), argc);
		
		let mut arg_i = 0;
		while arg_i < argc {
			printf(c"arg %i: %s\n".as_ptr(), arg_i, *(argv.wrapping_add(arg_i as usize)));
			arg_i += 1;
		}
		exit(420);
	}
}
