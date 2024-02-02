#![feature(lang_items, core_intrinsics, rustc_private)]
#![allow(internal_features)]
#![no_std]
#![no_main]
// Necessary for `panic = "unwind"` builds on some platforms.
#![feature(panic_unwind)]
extern crate unwind;

// Pull in the system libc library for what crt0.o likely requires.
extern crate libc;

use core::ffi::{c_char, c_int};
use core::panic::PanicInfo;
use cuddly_computing_machine::m_1628;

// Entry point for this program.
#[no_mangle] // ensure that this symbol is included in the output as `main`
extern "C" fn main(_argc: c_int, _argv: *const *const c_char) -> c_int {
    //let m = &mut [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    // TODO: Reinterpret nested array above as contiguous. If possible.
    let m = &mut [1, 2, 3, 4, 5, 6, 7, 8, 9];

    m_1628::rotate_3x3(m);

    if m == &[7, 4, 1, 8, 5, 2, 9, 6, 3] {
        0
    } else {
        1
    }
}

// These functions are used by the compiler, but not for an empty program like this.
// They are normally provided by `std`.
#[lang = "eh_personality"]
fn rust_eh_personality() {}
#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    core::intrinsics::abort()
}
