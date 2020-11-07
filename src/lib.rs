#[no_mangle]
pub extern fn f64sum(a: f64, b: f64) -> f64 {
    a + b
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // crate::do_thang();
    }
}
