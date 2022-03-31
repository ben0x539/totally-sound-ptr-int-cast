//! ## why is this sound
//!
//! To withstand this future I'm raving about in the readme, we
//! return to the old ways.
//!
//! C defines the `printf` format specifier `p` something like
//!
//! > The argument shall be a pointer to void. The value of the pointer
//! > is converted to a sequence of printing characters, in an
//! > implementation-defined manner.
//!
//! and the `scanf` format specifier `p` like
//!
//! > Matches an implementation-defined set of sequences, which should
//! > be the same as the set of sequences that may be produced by the %p
//! > conversion of the fprintf function. The corresponding argument
//! > shall be a pointer to a pointer to void. The input item is
//! > converted to a pointer value in an implementation-defined manner.
//! > If the input item is a value converted earlier during the same
//! > program execution, the pointer that results shall compare equal to
//! > that value; otherwise the behavior of the %p conversion is
//! > undefined.
//!
//! Once you're at "shall compare equal", you pretty much can't get out
//! of the pointers being usable interchangeably, or the wording for
//! casting to `void*` and back doesn't work.
//!
//! We additionally note that glibc documents that non-null pointers are
//! printed as hex integers. I'm pretty sure musl does about the same
//! thing but I haven't tested with musl.
//!
//! So, we `%p`-print a pointer and parse the result as an integer and
//! there's our very defensible integer representation of a pointer. For
//! the inverse, we print the integer in that exact hexadecimal form,
//! and then `%p`-parse it back into a pointer, and we have the great,
//! very defensible argument that we can actually use this pointer
//! because that's what the `scanf` docs say. The rust memory model
//! can hardly say that C stops working as defined, right?

use std::{mem, ptr};

pub fn ptr2int<T>(p: *const T) -> u64 {
    if p.is_null() { return 0; }

    unsafe {
        let mut buf = [0i8; 32];

        let r = libc::snprintf(
            buf.as_mut_ptr(),
            32,
            "%p\0".as_ptr() as *const _,
            p as *const libc::c_void);
        if r >= 32 {
            panic!("too long");
        }

        let mut n = mem::MaybeUninit::new(0u64);

        let r = libc::sscanf(
            buf.as_ptr(),
            "%lx\0".as_ptr() as *const i8,
            ptr::addr_of_mut!(n) as *mut _);
        if r != 1 {
            panic!("yr platform isnt supported");
        }

        n.assume_init()
    }
}

pub fn int2ptr<T>(n: u64) -> *const T {
    int2ptr_mut(n)
}

pub fn int2ptr_mut<T>(n: u64) -> *mut T {
    if n == 0 { return ptr::null_mut(); }

    unsafe {
        let mut buf = [0i8; 32];

        let r = libc::snprintf(
            buf.as_mut_ptr(),
            32,
            "0x%lx\0".as_ptr() as *const _,
            n);
        if r >= 32 {
            panic!("too long");
        }

        let mut p = mem::MaybeUninit::new(ptr::null_mut());

        let r = libc::sscanf(
            buf.as_ptr(),
            "%p\0".as_ptr() as *const i8,
            ptr::addr_of_mut!(p) as *mut *mut libc::c_void);
        if r != 1 {
            panic!("yr platform isnt supported");
        }

        p.assume_init()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let stuff = "it definitely works".to_string();
        let n = ptr2int(ptr::addr_of!(stuff));
        let p: *const String = int2ptr(n);
        let stuff = unsafe { &*p };
        assert_eq!(stuff, "it definitely works");
    }

    #[test]
    fn the_thing_from_the_readme() {
        use crate as totally_sound_ptr_int_cast;
        use std::thread;

        let your_boxed_stuff = Box::new(42.0);

        let n = totally_sound_ptr_int_cast::ptr2int(
            Box::into_raw(your_boxed_stuff));

        // lets do the "any later point" in a silly way
        thread::spawn(move || {
            let p = totally_sound_ptr_int_cast::int2ptr_mut(n);
            let s1 = unsafe { format!("{:?}", *p) };

            let my_boxed_stuff: Box<f64> = unsafe { Box::from_raw(p) };
            let s2 = format!("{:?}", my_boxed_stuff);
            assert_eq!(s1, s2);
        }).join().unwrap();
    }
}
