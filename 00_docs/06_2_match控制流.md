# 前言

Rust 有一个非常强大的控制流结构，称为 `match`，它允许你将一个值与一系列模式进行比较，然后根据哪个模式匹配来执行代码。模式可由字面量、变量、通配符和许多其他内容构成；`match` 的强大之处在于模式的表达性，以及编译器检查，它确保了所有可能的情况都得到处理。



# 内容

可以把 `match` 表达式想象成一台硬币分拣机：硬币滑入有着不同大小孔洞的轨道，每一个硬币都会掉入符合它大小的孔洞。同样，值也会通过 `match` 的每一个模式，并且在遇到第一个 “符合” 的模式时，值会进入相关联的代码块并在执行中被使用。

说到硬币，让我们以 `match` 为例！我们可以编写一个函数，它接受一个未知的美国硬币，并以类似于计数机的方式，确定它是哪个硬币并返回以美分为单位的值。

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

拆开 `value_in_cents` 函数中的 `match` 来看。首先，我们列出 `match` 关键字，后跟一个表达式，在这个例子中是 `coin` 的值。这看起来非常像 `if` 使用的表达式，不过这里有一个非常大的区别：对于 `if`，表达式必须返回一个布尔值，而这里它可以是任何类型的。例子中的 `coin` 的类型是我们定义的 `Coin` 枚举。

接下来是 `match` 的分支。一个分支有两个部分：一个模式和一些代码。第一个分支的模式是值 `Coin::Penny` 而之后的 `=>` 运算符将模式和将要运行的代码分开。这里的代码就仅仅是值 `1`。每一个分支之间使用逗号分隔。

当 `match` 表达式执行时，它会按顺序将结果值与每个分支的模式进行比较。如果模式与该值匹配，则执行与该模式关联的代码。如果该模式与值不匹配，则继续执行到下一个分支，就像在硬币分拣机中一样。我们需要多少分支 就有多少：在上方的代码中，我们的 `match` 有 4 个 分支。

与每个分支关联的代码是一个表达式，匹配分支中表达式的结果值是为整个 `match` 表达式返回的值。

如果 match 分支代码很短，我们通常不会使用大括号，就像代码示例中每个分支只返回一个值。如果要在匹配的分支中运行多行代码，则必须使用大括号，并且分支后面的逗号是可选的。例如，以下代码在每次使用 `Coin::Penny`调用该方法时都会打印 “Lucky penny！”，但仍然返回代码块的最后一个值 `1`：

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```



## 绑定值的模式

match 分支的另一个有用功能是它们可以绑定到与模式匹配的值部分。这就是我们从枚举成员中提取值的方法。

例如，让我们更改其中一个枚举成员以在其中保存数据。从 1999 年到 2008 年，美国为 50 个州中的每一个州铸造了一面设计不同的 25 美分硬币。没有其他硬币有国家设计，所以只有 25 美分硬币有这个额外的价值。我们可以通过更改 `Quarter` 成员来包含存储在其中的 `UsState` 值，从而将此信息添加到我们的枚举中，我们在下面代码中已经完成了此操作。

```rust
#[derive(Debug)] // so we can inspect the state in a minute
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
```

假设一个朋友正在尝试收集所有 50 个州的 25 美分硬币。当我们按硬币类型对零钱进行排序时，我们还会说出与每个 25 美分硬币相关的州名，这样如果我们的朋友没有，他们可以将其添加到他们的收藏中。

在此代码的匹配表达式中，我们将一个名为 `state` 的变量添加到匹配变体 `Coin::Quarter` 的值的模式中。当 `Coin::Quarter` 匹配时，变量 `state` 将会绑定 25 美分硬币所对应州的值。接着在那个分支的代码中使用 `state`，如下：

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

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
}
```

如果我们调用 `value_in_cents(Coin::Quarter(UsState::Alaska))` ，`coin`将是 `Coin::Quarter(UsState::Alaska)` 。当我们将该值与每个匹配分支进行比较时，在我们到达 `Coin::Quarter（state）` 之前，它们都不匹配。此时，`state` 的绑定将是值 `UsState::Alaska`。然后，我们可以在 `println！`表达式中使用该绑定，从而从 `Quarter` 的 `Coin` 枚举成员中获取内部状态值。

## 匹配Option<T>

我们想在使用 `Option<T>` 时从 `Some` 情况中获取内部 `T` 值；我们也可以使用 `match` 来处理 `Option<T>`，就像我们对 `Coin` 枚举所做的那样！我们将比较 `Option<T>` 的成员，而不是比较硬币，但`match`表达式的工作方式保持不变。

假设我们想编写一个采用 `Option<i32>` 的函数，如果里面有一个值，则将该值加 1。如果内部没有值，则函数应返回 `None` 值，并且不尝试执行任何操作。

多亏了 `match`，这个函数很容易编写，看起来如下所示：

```rust
fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
```

让我们更详细地研究一下 `plus_one` 的第一次执行。当我们调用 `plus_one（five）` 时，`plus_one` 主体中的变量 `x` 将具有值 `Some（5）。`然后，我们将其与每个分支进行比较：

```rust
 None => None,
```

`Some（5）` 值与模式 `None` 不匹配，因此我们继续下一个分支：

```rust
 Some(i) => Some(i + 1),
```

`Some（5）` 是否与 `Some（i）` 匹配？确实如此！我们有相同的成员。`i` 绑定到 `Some` 中包含的值，因此 `i` 取值 `5`。然后执行匹配分支中的代码，因此我们将 `i` 的值加 1 并创建一个新的 `Some` 值，其中总共为 `6`。



接着考虑下示例中 `plus_one` 的第二个调用，这里 `x` 是 `None`。我们进入 `match` 并与第一个分支相比较。

```rust
None => None,
```

匹配上了！这里没有值来加一，所以程序结束并返回 `=>` 右侧的值 `None`，因为第一个分支就匹配到了，其他的分支将不再比较。

在许多情况下，将 `match` 和枚举组合在一起很有用。你会在 Rust 代码中经常看到这种模式：`match`一个枚举，将一个变量绑定到里面的数据，然后基于它执行代码。一开始有点棘手，但一旦你习惯了它，你会希望你拥有所有语言的它。它一直是用户的最爱。

## 匹配是穷尽的

我们需要讨论`match`的另一个方面：分支的模式必须涵盖所有可能性。考虑这个版本的 `plus_one` 函数，它有一个 bug，不会编译：

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}
```

们没有处理 `None` 的情况，所以这段代码会导致一个 bug。幸运的是，这是一个 Rust 知道如何捕获的 bug。如果我们尝试编译此代码，我们将收到此错误：

```shell
error[E0004]: non-exhaustive patterns: `Some(_)` not covered
   --> src/main.rs:202:15
    |
202 |         match x {
    |               ^ pattern `Some(_)` not covered
    |
note: `Option<i32>` defined here
   --> /Users/wangyang/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/option.rs:571:1
    |
571 | pub enum Option<T> {
    | ^^^^^^^^^^^^^^^^^^
...
579 |     Some(#[stable(feature = "rust1", since = "1.0.0")] T),
    |     ---- not covered
    = note: the matched value is of type `Option<i32>`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
    |
203 ~             None => None,
204 ~             Some(_) => todo!(),
    |
```

Rust 知道我们没有涵盖所有可能的情况，甚至知道我们忘记了哪个模式！Rust 中的匹配是*穷举式的*：我们必须穷举到最后的可能性才能使代码有效。特别是在 `Option<T>` 的情况下，当 Rust 防止我们忘记显式处理 `None` 情况时，它保护了我们避免在可能有 null 时假设我们有一个值，从而使前面讨论的数十亿美元的错误变得不可能。

## 通配模式和 _ 占位符

使用枚举，我们还可以为一些特定值采取特殊操作，但对于所有其他值，采取一个默认操作。想象一下，我们正在实现一个游戏，如果您在掷骰子时掷出 3，您的玩家不会移动，而是得到了一顶新的漂亮帽子。如果您掷出 7，您的玩家将失去一顶漂亮的帽子。对于所有其他值，玩家在游戏板上移动该数量的空格。下面是一个实现该逻辑的`match`，其中掷骰子的结果是硬编码的，而不是随机值，所有其他逻辑都由没有主体的函数表示，因为实际实现它们超出了此示例的范围：

```rust
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
}
```

对于前两个分支，模式是文本值 `3` 和 `7`。对于覆盖所有其他可能值的最后一个分支，模式是我们选择命名为 `other` 的变量。为`other`分支运行的代码通过将变量传递给 `move_player` 函数来使用该变量。

即使我们没有列出 `u8` 可以具有的所有可能的值，这段代码也会编译，因为最后一个模式将匹配所有未明确列出的值。这种匹配模式满足 `match` 必须被穷尽的要求。请注意，我们必须将通配分支放在最后，因为模式是按顺序匹配的。如果我们早点放置通配分支，其他分支将永远不会运行，因此如果我们在通配分支之后添加分支，Rust 会警告我们！

Rust 也有一个模式，当我们想要一个通配模式但不想使用通配模式中的值时可以使用： `_` 是一个特殊的模式，它匹配任何值，并且不绑定到那个值。这告诉 Rust 我们不会使用这个值，所以 Rust 不会警告我们未使用的变量。

让我们改变游戏规则：现在，如果您掷出 3 或 7 以外的任何值的时候，则必须再次掷骰。我们不再需要使用这个值，因此我们可以将代码更改为使用 `_` 而不是名为 `other` 的变量：

```rust
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
}
```

此示例还满足穷举性要求，因为我们显式忽略了最后一个分支中的所有其他值；我们没有忘记任何东西。

最后，我们将再次更改游戏规则，这样，如果您掷出 3 或 7 以外的任何值，则轮到您时不会发生任何其他事情。我们可以通过使用单元值（我们在[03_通用编程概念](./03_通用编程概念.md)中元组类型一节中提到的空元组类型）作为 `_` 分支对应的代码来表示这一点：

```rust
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
}
```

在这里，我们明确告诉 Rust，我们不会使用任何其他与前面分支中的模式不匹配的值，并且在这种情况下我们不想运行任何代码。

我们将在后续中介绍更多关于模式和匹配的内容。现在，我们将继续使用 `if let` 语法，该语法在 `match` 表达式有点冗长的情况下可能很有用。



