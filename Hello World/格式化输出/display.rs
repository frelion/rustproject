/*
fmt::Debug 通常看起来不太简介，因此自定义输出的外观经常是可取的。这需要通过手动实现
fmt::Display来做到。fmt::Display采用{}标记。实现方式看起来像这样：
*/


// 使用 use 导入fmt亩哦块使 `fmt::Display` 可用
use std::fmt;

// 定义一个结构体，咱们会为他实现 `fmt::Display`。一下诗歌简单的元组结构体
// `Structure`, 包含一个 `i32`元素
struct Structure(i32);

// 为了使用`{}`标记，必须手动为类型实现`fmt::Displayg`trait
impl fmt::Display for Structure{
    // 这个 trait 要求 `fmt` 使用与下面的函数完全一致的函数签名
    fn fmt(&self, f: &mut fmt::formatter) -> fmt::Result{
        // 仅将 self 的第一个元素写入到给定的输出流 `f`。返回 `fmmt:Result`，此
        // 结果表明操作成功或失败。注意 ``write!`的用法和`println!`很相似。
        write!(f, "{}", self.0)
    }
}

/*
fmt::Display 的效果可能比 fmt::Debug 简介，但对于std库来说，这就有一个问题。模棱两可的类型该如何显示呢？
据个例子，假设标准库对所有的Vec<T>都实现了同一种输出样式，那么它应该是哪种样式？下面两种的一种吗？
    - Vec<path>：/:/etc:/home/username:/bin （使用:分割）
    - Vec<number>：1,2,3（使用,分割）
rust并没有这样做，因为没有一种合适的样式适用于所有类型。标准库也并不擅自规定一种样式。对于Vec<T>或其他任意
泛型容器，fmt::Display都没有实现。因此在这些泛型的情况下药用fmt::Debug。

这并不是一个问题，因为对于任何非泛型的容器类型， fmt::Display 都能够实现。
*/


