# 前言

新 Rust 开发者通常会因三个原因而在字符串上遇到困难：Rust 倾向于暴露可能的错误、字符串作为一种数据结构比许多程序员认为的要复杂，以及 UTF-8。这些因素结合在一起，可能会让你在从其他编程语言转换过来时觉得困难。

我们将字符串作为集合来讨论，因为字符串被实现为一个字节集合，加上一些方法，以便在将这些字节解释为文本时提供有用的功能。在本节中，我们将讨论每个集合类型都有的 String 操作，例如创建、更新和读取。我们还将讨论 String 与其他集合的不同之处，即索引到 String 的复杂性，这是由于人和计算机解释 String 数据的方式不同。


# 内容

我们首先定义我们所说的字符串术语。Rust 在核心语言中只有一个字符串类型，即通常以借用形式 `&str `出现的字符串切片 `str`。在前面的章节中，我们讨论了字符串切片，它们是对存储在其他地方的某些`UTF-8` 编码字符串数据的引用。例如，字符串字面量存储在程序的二进制文件中，因此它们是字符串切片。

String 类型由 Rust 的标准库提供，而不是编码在核心语言中，是一个可增长的、可变的、拥有的 UTF-8 编码字符串类型。当 Rust 开发者提到“字符串”时，他们可能指的是 `String` 或 `&str` 类型，而不仅仅是其中一种类型。尽管本节主要讨论 `String`，但这两种类型在 Rust 的标准库中都被大量使用，并且两者都是 UTF-8 编码的。



## 创建字符串

许多与 Vec<T> 可用的操作也可用于 String，因为 String 实际上是包装在一个字节数组周围的，并具有一些额外的保证、限制和功能。一个与 Vec<T> 和 String 工作方式相同的函数示例是 new 函数，用于创建实例

`Vec<T>` 中可用的许多相同操作也可用于 `String`，因为 `String` 实际上是作为字节向量的包装器实现的，具有一些额外的保证、限制和功能。与 `Vec<T>` 和 `String` 一起工作方式相同的函数的一个例子是`new`函数，用于创建实例。

```rust
fn main() {
    let mut s = String::new();
}
```

这行代码创建了一个名为`s` 的新空字符串，然后我们可以将数据加载到其中。通常，我们会有一些初始数据来启动字符串。为此，我们使用 `to_string` 方法，该方法可用于实现 Display 特征的任何类型，字符串字面量也是如此。

```rust
fn main() {
    let data = "initial contents";

    let s = data.to_string();
    println!("sting s ===>: {s}");
    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    
    println!("s=====>: {s}");
}
```

我们还可以使用函数 `String::from` 从字符串字面量创建一个 String。

```rust
fn main() {
    let s = String::from("initial contents");
    println!("s=====>: {s}");
}
```

因为字符串被用于很多事物中，我们可以为字符串使用许多不同的通用API，这为我们提供了很多选择。其中一些可能看起来多余，但它们都有其存在的意义！在这种情况下，`String::from` 和`to_string` 执行相同的操作，因此您选择哪一个只是样式和可读性的问题。

请记住，字符串是 UTF-8 编码的，因此我们可以在其中包含任何正确编码的数据，如下所示。

```rust
fn main() {
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}
```

## 更新字符串

你可以将更多数据推送到 `String` 中，就像 `Vec<T>` 的内容一样。此外，您可以方便地使用 `+` 运算符或 `format!` 宏来连接 `String` 值。

### 使用push_str和push向string中追加内容

```rust
fn main() {
    let mut s = String::from("foo");
    s.push_str("bar");
}
```

在这两行之后，`s` 将包含 `foobar`。`push_str` 方法接收一个字符串切片（`&str`），因为我们不一定想要获取参数的所有权。例如，在下面的示例代码中，我们希望能够在将 s2 的内容附加到 `s1` 后使用`s2`。

```rust
fn main() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");
}
```

如果 `push_str` 方法获得了 `s2` 的所有权，我们将无法在最后一行打印其值。但是，此代码的工作方式符合我们的预期！

`push` 方法将单个字符作为参数，并将其添加到`String` 中。下面我们使用 `push`方法将字母`l`添加到 `String` 中。

```rust
fn main() {
    let mut s = String::from("lo");
    s.push('l');
    println!("s: {s}");
}
```

###  使用 `+` 运算符或 `format!`宏

通常，您需要合并两个现有字符串。一种方法是使用 `+` 运算符，如下所示：

```rust
fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("s3:{s3}")
}
```

字符串 `s3` 将包含 `Hello, world!`。添加后 `s1` 不再有效的原因，以及我们使用 `s2` 引用的原因，与使用 `+` 运算符时调用的方法的签名有关。`+` 运算符使用 `add` 方法，其签名如下所示：

```rust
fn add(self, s: &str) -> String {
```

在标准库中，你会看到`add`是用泛型和关联类型定义的。在这里，我们替换了具体的类型，这就是当我们用 `String` 值调用这个方法时发生的情况。

首先，`s2` 有一个 `&`，这意味着我们正在将第二个字符串的引用添加到第一个字符串中。这是因为 `add`函数中的 `s` 参数：我们只能向 `String` 添加 `&str`；我们不能将两个 `String`值相加。但是 `&s2` 的类型是 `&String`，而不是`add`函数的第二个参数所指定的`&str`，为什么编译会通过嘛？

我们能够在 `add` 调用中使用 `&s2` 的原因是编译器可以将 `&String` 参数强制转换为 `&str`。当我们调用 `add`方法时，Rust 将 `&s2` 强制转为了`&s2[..]`。由于 `add` 不获取 `s` 参数的所有权，因此在此操作后 `s2` 仍将是有效的 `String`。

其次，我们可以在签名中看到 `add` 获得了 `self`的所有权，因为 `self` *没有* `&`。这意味着 `s1` 将被移动到 `add` 调用中，之后将不再有效。因此，尽管`let s3 = s1 + &s2;` 看起来会复制两个字符串并创建一个新字符串，但此语句实际上获取了 `s1` 的所有权，追加了 `s2` 内容的副本，然后返回结果的所有权。换句话说，它看起来正在制作很多副本，但事实并非如此；实现比复制更有效。

如果我们需要连接多个字符串，`+` 运算符的行为会变得笨拙：

```rust
fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s:{s}")
}
```

此时，`s` 将是`tic-tac-toe`。对于所有的 `+` 和 `"`，很难看出发生了什么。对于以更复杂的方式组合字符串，我们可以改用 `format!`宏：

```rust
fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("s:{s}")
}
```

此代码还将 `s` 设置为 `tic-tac-toe`。`format!` 宏的工作方式与`println!` 类似，但它不是将输出打印到屏幕上，而是返回一个包含内容的`String`。使用 `format!` 的代码版本更易于阅读，并且由 `format!` 宏生成的代码使用引用，因此此调用不会占用其任何参数的所有权。

## 使用索引读取字符串

在许多其他编程语言中，通过索引访问字符串中的单个字符是有效且常见的操作，但是在Rust中如果你尝试这种操作，你会得到一个错误，现在让我们一起来尝试下。

```rust
fn main() {
    let s1 = String::from("hello");
    let h = s1[0];
}
```

错误如下：

```shell
error[E0277]: the type `str` cannot be indexed by `{integer}`
  --> src/main.rs:73:16
   |
73 |     let h = s1[0];
   |                ^ string indices are ranges of `usize`
   |
   = help: the trait `SliceIndex<str>` is not implemented for `{integer}`, which is required by `String: Index<_>`
   = note: you can use `.chars().nth()` or `.bytes().nth()`
           for more information, see chapter 8 in The Book: <https://doc.rust-lang.org/book/ch08-02-strings.html#indexing-into-strings>
   = help: the trait `SliceIndex<[_]>` is implemented for `usize`
   = help: for that trait implementation, expected `[_]`, found `str`
   = note: required for `String` to implement `Index<{integer}>`

```

错误和注释说明了问题：Rust 字符串不支持索引。但是为什么不支持呢？要回答这个问题，我们需要讨论 Rust 如何将字符串存储在内存中。

`String` 是 `Vec<u8>` 的包装器。让我们看看一些正确编码的 UTF-8 示例字符串。首先，这个：

```rust
fn main() {
    let hello = String::from("Hola");
}
```

在这种情况下，`len` 将为 `4`，这意味着存储字符串`"Hola"`的向量长度为 4 字节。这些字母在以 UTF-8 编码时每个字母都占用一个字节。但是，下一行可能会让您感到惊讶（请注意，此字符串以大写的西里尔字母 *Ze* 开头，而不是数字 3）

```rust
fn main() {
      let hello = String::from("Здравствуйте");
}
```

如果有人问你字符串有多长，你可能会说 12。事实上，Rust 的答案是 24：这是在 UTF-8 中编码 “Здравствуйте” 所需的字节数，因为该字符串中的每个 Unicode 标量值都需要 2 个字节的存储空间。因此，字符串字节的索引并不总是与有效的Unicode标量值相关联。为了证明这一点，考虑以下无效的Rust代码：

```rust
let hello = "Здравствуйте";
let answer = &hello[0];
```

你已经知道答案不会是 `З`，第一个字母。当用 UTF-8 编码时，`З` 的第一个字节是 `208`，第二个字节是 `151`，所以看起来答案实际上应该是 `208`，但 `208` 本身并不是一个有效的字符。如果用户要求此字符串的第一个字母，则返回 `208` 可能不是用户想要的；但是，这是 Rust 在字节索引 0 处的唯一数据。用户通常不希望返回字节值，即使字符串仅包含拉丁字母：如果 `&"hello"[0]` 是返回字节值的有效代码，它将返回 `104`，而不是 `h`。

那么，答案是，为了避免返回意外值并导致可能不会立即发现的错误，Rust 根本不编译这些代码，并防止在开发过程的早期产生误解。

关于 UTF-8 的另一点是，从 Rust 的角度来看，实际上有三种相关的方法可以查看字符串：字节、标量值和字形簇（最接近我们所说的字母）。

如果我们看一下用梵文书写的印地语单词“नमस्ते”，它被存储为 `u8` 值的向量，如下所示：

```rust
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]
```

这是 18 字节，是计算机最终存储这些数据的方式。如果我们将它们视为 Unicode 标量值，这就是 Rust 的 `char` 类型，这些字节看起来像这样：

```rust
['न', 'म', 'स', '्', 'त', 'े']
```

这里有六个字符值，但第四个和第六个不是字母：它们是变音符号，它们本身没有意义。最后，如果我们把它们看作是字素簇，我们就会得到一个人所说的构成印地语单词的四个字母：

```rust
["न", "म", "स्", "ते"]
```

Rust提供了不同的方式来解释计算机存储的原始字符串数据，以便每个程序可以选择它需要的解释，无论数据使用哪种人类语言。

Rust 不允许我们索引到 `String` 中来获取字符的最后一个原因是索引操作总是需要恒定的时间（O（1））。但是不能保证使用 `String` 的性能，因为 Rust 必须从头到索引遍历内容以确定有多少有效字符。



## 字符串切片

索引到字符串通常是一个不好的主意，因为字符串索引操作的返回类型应该是什么并不清楚：字节值、字符、字形簇还是字符串切片。因此，如果你真的需要使用索引来创建字符串切片，Rust会要求你更具体。

与其使用[]和一个数字进行索引，你可以使用[]和一个范围来创建一个包含特定字节的字符串切片：

```rust
#![allow(unused)]
fn main() {
  let hello = "Здравствуйте";
  let s = &hello[0..4];
  println!("s:{s}")
}
```

这里，s将是一个&str，包含字符串的前四个字节。之前，我们提到过这些字符每一个都是两个字节，这意味着s将是Зд。

如果我们尝试使用类似`&hello[0..1]`的方法来切割一个字符的部分字节，Rust会在运行时像访问向量中的无效索引一样发生错误：

```shell
/Users/wangyang/.cargo/bin/cargo run --color=always --package n08_string --bin n08_string --profile dev
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/n08_string`
thread 'main' panicked at src/main.rs:86:19:
byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`
stack backtrace:
   0: rust_begin_unwind
             at /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/std/src/panicking.rs:662:5
   1: core::panicking::panic_fmt
             at /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/core/src/panicking.rs:74:14
   2: core::str::slice_error_fail_rt
   3: core::str::slice_error_fail
             at /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/core/src/str/mod.rs:68:5
   4: core::str::traits::<impl core::slice::index::SliceIndex<str> for core::ops::range::Range<usize>>::index
             at /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/core/src/str/traits.rs:242:21
   5: core::str::traits::<impl core::ops::index::Index<I> for str>::index
             at /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/core/src/str/traits.rs:60:9
   6: n08_string::main
             at ./src/main.rs:86:19
   7: core::ops::function::FnOnce::call_once
             at /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

进程已结束，退出代码为 101
```

在通过范围创建字符串切片时，您应该谨慎行事，因为这样做可能会导致程序崩溃。

## 遍历字符串方法

操作字符串片段的最佳方式是明确您是要字符还是字节。对于单个Unicode标量值，请使用`chars`方法。在“Зд”上调用`chars`会分离并返回两个`char`类型的值，您可以迭代结果以访问每个元素：

```rust
#![allow(unused)]
fn main() {
  for c in "Зд".chars() {
      println!("{c}");
  }
}
```

此内容输出内容如下：

```shell
/Users/wangyang/.cargo/bin/cargo run --color=always --package n08_string --bin n08_string --profile dev
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/n08_string`
З
д
```

或者，bytes方法返回每个原始字节，这可能适用于您的领域：

```rust
#![allow(unused)]
fn main() {
  for b in "Зд".bytes() {
      println!("{b}");
  }
}
```

将输出组成这个字符串的四个字节：

```shell
/Users/wangyang/.cargo/bin/cargo run --color=always --package n08_string --bin n08_string --profile dev
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/n08_string`
208
151
208
180
```

但请务必记住，有效的Unicode标量值可能由多个字节组成。

从字符串中获取字形簇，如梵文脚本，是复杂的，所以标准库不提供这种功能。如果你需要这种功能，可以在crates.io上找到相应的库。



## 字符串并不简单

总而言之，字符串很复杂。不同的编程语言对如何向程序员呈现这种复杂性做出了不同的选择。Rust 选择将正确处理 `String` 数据作为所有 Rust 程序的默认行为，这意味着程序员必须提前花更多的心思来处理 UTF-8 数据。与其他编程语言相比，这种权衡暴露了更多的字符串复杂性，但它可以防止您在开发生命周期的后期处理涉及非 ASCII 字符的错误。

好消息是，标准库提供了许多基于 `String` 和 `&str` 类型构建的功能，以帮助正确处理这些复杂的情况。请务必查看文档以了解有用的方法，如contains（在字符串中搜索）和replace（用另一个字符串替换字符串的部分）。

让我们切换到稍微简单一点的东西：哈希映射！
