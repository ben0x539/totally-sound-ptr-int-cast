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
}
