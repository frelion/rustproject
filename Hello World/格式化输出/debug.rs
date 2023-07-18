/*
所有的类型，如果想要std:fmt的格式化打印，都要求实现至少一个可打印的 `traits` （这点似乎和python有点类似）
仅有一些类型提供了自动实现，比如std库中的类型。其他的类型都必须手动实现！

fmt::Debug 这个 trait 使这项工作变得相当简单。所有类型都能推倒 (derive, 即自动创建)
fmt::Debug 的实现。但是fmt::Display需要手动实现。
*/

// 这个结构体不能使用 `fmt::Display` 或 `fmt::Debug`来进行打印
#[allow(dead_code)]
struct UnPrintable(i32);

// `derive` 属性会自动创建所需的实现，使这个 `struct`能使用 `fmt::Debug` 打印
#[allow(dead_code)]
#[derive(Debug)]
struct DebugPrintable(i32);

// 推导`Structure`的`fmt::Debug`实现
// `Structure` 是一个包含但个`i32`的结构体
#[derive(Debug)]
struct Structure(i32);

// 将 `Strcuture` 放入结构体Deep中，然后使Deep也能够被打印
#[derive(Debug)]
struct Deep(Structure);

#[allow(dead_code)]
#[derive(Debug)]
struct Persion<'a> {
    name: &'a str,
    age: u8
}


fn main(){
    // 使用``{:?}`打印和使用 `{}`类似
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?}  is the {actor:?} name.",
            "Slater",
            "Christian",
            actor="actor's");
    
    // `Structure` 也可以打印
    println!("Now {:?} will print!", Structure(3));

    // 使用 `derive` 的一个问题是不能控制输出的形式
    // 加入我只想展示一个7该怎么办？
    println!("Now {:?} will print!", Deep(Structure(3)));

    // 173 2736 7583
    let name = "Peter";
    let age = 27;
    let peter = Persion{
        name,
        age
    };

    // 美化打印
    println!("{:#?}", peter);
}