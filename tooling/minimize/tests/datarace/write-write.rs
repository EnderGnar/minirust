extern crate intrinsics;
use intrinsics::*;

static GLOBAL: usize = 0;

fn thread() {
    let ptr = &GLOBAL as *const usize as *mut usize;
    
    let mut i = 0;
    while i < 256 {
        unsafe { atomic_write(ptr, 1) };
        i += 1;
    }
}

fn main() {
    let ptr = &GLOBAL as *const usize as *mut usize;
    let id = spawn(thread as fn());

    let mut i = 0;
    while i < 256 {
        unsafe { *ptr = i };
        i += 1;
    }

    join(id);
}
