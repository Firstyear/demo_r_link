
extern "C" {
    fn reverse_call(a: f64) -> f64;
}


#[no_mangle]
pub extern fn f64sum(a: f64, b: f64) -> f64 {
    a + b
}

#[no_mangle]
pub extern fn f64mul(a: f64) -> f64 {
    unsafe { reverse_call(a) }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
