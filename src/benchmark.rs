#[no_mangle]
pub extern "C" fn benchmark() {
    let mut a = 1;
    for _ in 0..99999 {
        a += 1;
    }
    println!("{}", a);
}
