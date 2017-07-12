fn foo(x: u32, y: u32) -> u32 {
    x + y
}

fn main() {
    let a = 12;
    let b = 42;
    let z: u32 = {
        foo(b, a)
    };
    print!("{}", z);
}