#![feature(lang_items, core_intrinsics, rustc_private)]
#![allow(internal_features)]
#![no_std]
#![no_main]
#![feature(panic_unwind)]

#[cfg(not(test))]
extern crate unwind;
#[cfg(not(test))]
extern crate libc;

#[cfg(test)]
use core::ffi::c_char;
#[cfg(test)]
use core::ffi::c_int;
#[cfg(not(test))]
use core::ffi::{c_char, c_int};
#[cfg(not(test))]
use core::panic::PanicInfo;
use cuddly_computing_machine::m_1628;

#[no_mangle]
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

#[cfg(not(test))]
#[lang = "eh_personality"]
fn rust_eh_personality() {}

#[cfg(not(test))]
#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    core::intrinsics::abort()
}
