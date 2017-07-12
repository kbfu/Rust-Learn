fn main() {
    println!("Hello World!");
    println!("I'm a Rustacean!");
    // println! 会检查使用到的参数数量是否正确。
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // 改正 ^ 补上漏掉的参数： "James"

    // 创建一个包含` I32 `类型结构体(structure)。命名为 `Structure`。
    #[allow(dead_code)]
    struct Structure(i32);

    // 但是像结构体这样自定义类型需要更复杂的方式来处理。
    // 下面语句无法运行。
    // println!("This struct `{}` won't print...", Structure(3));
}