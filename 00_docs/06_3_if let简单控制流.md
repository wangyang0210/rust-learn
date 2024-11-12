# 前言

在这一章我们将开始`if let`的学习，`if let` 语法允许您将 `if` 和 `let` 组合成一种不太冗长的方式，以处理与一个模式匹配的值，同时忽略其余模式，现在让我们开始学习吧。



# 内容

现在让我们写一个代码，它与 `config_max` 变量中的 `Option<u8>` 值匹配，但只在值为 `Some` 成员时执行代码。

```rust
fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
}
```

如果值为 `Some`，我们通过将值绑定到模式中的变量 `max` 来打印出 `Some` 成员中的值。我们不想对 `None` 值执行任何操作。为了满足 `match` 表达式，我们必须在只处理一个成员后添加 `_ => （），`这是很烦人的样板代码。

相反，我们可以使用 `if let` 以更短的方式编写它。以下代码的行为与上面中的 `match` 相同：

```rust

#![allow(unused)]
fn main() {
  let some_u8_value = Some(0u8);
  if let Some(3) = some_u8_value {
      println!("three");
  }
}
```

语法 `if let` 采用模式和表达式，以等号分隔。它的工作方式与 `match` 相同，其中表达式给出了匹配，模式是它的第一个分支。在这种情况下，模式是 `Some（max）`，并且 `max` 绑定到 `Some` 中的值。然后，我们可以在 `if let` 块的主体中使用 `max`，就像我们在相应的 `match` 分支中使用 `max` 一样。如果值与模式不匹配，则` if let` 块中的代码不会运行。

使用 `if let` 意味着更少的键入、更少的缩进和更少的样板代码。但是，您将失去 `match` 强制要求的穷尽性检查。在 `match` 和 `if let` 之间进行选择取决于您在特定情况下执行的操作，以及获得简洁性和是否是失去穷尽性检查的权衡。

换句话说，您可以将 `if let` 视为`match`的语法糖，当值与一个模式匹配时运行代码，然后忽略所有其他值。

我们可以在 `if let` 中包含 `else`。与 `else` 一起使用的代码块与 `match` 表达式中与 `_` 写一起使用的代码块相同，等效于 `if let` 和 `else`。回想一下在match控制流示例中的 `Coin` 枚举定义，其中 `Quarter` 成员也包含一个 `UsState` 值。如果我们想计算我们看到的所有非 25 美分硬币，同时宣布 25 美分硬币的状态，我们可以使用 `match` 表达式来实现，如下所示：

```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin = Coin::Penny;
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}!"),
        _ => count += 1,
    }
}
```

或者我们可以使用 `if let` 和 `else` 表达式，如下所示：

```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin = Coin::Penny;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }
}
```

如果你的情况是你的程序的逻辑太冗长而无法使用 `match` 来表示，请记住 `if let` 也在你的 Rust 工具箱中。

## 总结

我们现在已经介绍了如何使用枚举创建自定义类型，这些类型可以是一组枚举值之一。我们已经展示了标准库的 `Option<T>` 类型如何帮助您使用类型系统来防止错误。当枚举值中包含数据时，您可以使用 `match` 或 `if let` 来提取和使用这些值，具体取决于您需要处理多少个情况。

您的 Rust 程序现在可以使用结构和枚举来表达作用域中的概念。创建要在 API 中使用的自定义类型可确保类型安全：编译器将确保您的函数仅获取每个函数期望的值。

为了给你的用户提供一个组织良好的 API，它易于使用，并且只公开你的用户需要的东西，现在让我们转向 Rust 的模块。
