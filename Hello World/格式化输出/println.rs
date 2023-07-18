fn main(){
    // 通常情况下 {} 被任何变量内容所替换
    // 变量内容会转化成字符串
    println!("{} days", 31);
    
    // 不加后缀的话， 31就被转化成i32类型
    // 我们可以添加后缀来改变31的类型，如31i64（这样就声明31为i64类型了）

    // 使用变量替换字符串有多种写法
    // 比如可以使用位置参数
    println!("{0}, this is {1}, {1}, this is {0}", "Alice", "Bob");

    // 也可以使用命名参数
    println!("{subject} {verb} {object}",
        object="the lazy dog",
        subject="the quick brown fox",
        verb="jumps over");
    
    // 可以在 `:` 后面指定特殊的格式
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // 你可以按指定宽度来右对齐文本
    // 下面语句输出 "      1"， 5个空格后面连着1
    println!("{number:>width$}", number=1, width=6);

    // 也可以在数字左边补零
    println!("{number:>0width$}", number=1, width=6);

    //println! 会检查使用到的参数数量是否正确
    /* 
    println!("My name is {0}, {1} {0}", "Bond");
        error: invalid reference to positional argument 1 (there is 1 argument)
        --> format.rs:30:32
        |
        30 |     println!("My name is {0}, {1} {0}", "Bond");
        |                                ^
        |
        = note: positional arguments are zero-based

        error: aborting due to previous error
    */
    // 改正，补上漏掉的参数，"James"
    println!("My name is {0}, {1} {0}", "Bond", "James");


    // 创建一个包含但个 `i32` 的结构体 （structure）。命名为 `Structure`
    #[allow(dead_code)]
    struct Structure(i32);

    // 但是像结构体这样的自定义类型需要更复杂的方式来处理。
    // 下面语句无法运行。
    /*
    println!("This struct `{}` won't print...", Structure(3));
        error[E0277]: `Structure` doesn't implement `std::fmt::Display`
        --> format.rs:51:49
        |
        51 |     println!("This struct `{}` won't print...", Structure(3));
        |                                                 ^^^^^^^^^^^^ `Structure` cannot be formatted with the default formatter
        |
        = help: the trait `std::fmt::Display` is not implemented for `Structure`
        = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
        = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

        error: aborting due to previous error

        For more information about this error, try `rustc --explain E0277`.
    */

    // 打印pi，精确到小数点后3位
    let pi = 3.1415926;
    println!("Pi is roughly {:.3}", pi);
}