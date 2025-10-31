static mut COUNTER: i32 = 0;

#[unsafe(no_mangle)]
pub extern "C" fn increment(x: i32) -> i32 {
    unsafe {
        COUNTER += x;
        COUNTER
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn get() -> i32 {
    unsafe { COUNTER }
}
