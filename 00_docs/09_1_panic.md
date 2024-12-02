# 前言

错误是软件中不可避免的事实，因此Rust提供了许多特性来处理出现问题的情况。在许多情况下，Rust要求你在代码编译之前发现错误的可能性并采取一些行动。这个要求通过确保你在将代码部署到生产环境之前发现并适当处理错误，使你的程序更加健壮！

Rust将错误分为两大类：可恢复和不可恢复的错误。对于可恢复的错误，例如找不到文件错误，我们很可能只想向用户报告问题并重试该操作。不可恢复的错误总是bug的症状，例如试图访问数组末尾之外的位置，因此我们希望立即停止程序。

# 内容

大多数编程语言不区分这两种错误，并使用异常等机制以相同的方式处理它们。Rust没有异常。相反，它具有用于可恢复错误的类型 `Result<T, E>` ，以及当程序遇到无法恢复的错误时停止执行的 `panic!`宏。本章首先介绍调用 `panic!`，然后讨论返回`Result<T, E>` 值。此外，我们还将探讨在决定是尝试从错误中恢复还是停止执行时需要考虑的因素。

## 不可恢复的错误panic!

有时候，你的代码中会发生一些糟糕的事情，而你对此无能为力。在这种情况下，Rust有一个 `panic!` 宏。在实践中，有两种方法可以引发异常：采取一个导致我们的代码异常的行为（例如访问数组末尾之后的元素）或显式调用`panic!`宏。在这两种情况下，我们都会在程序中引发异常。默认情况下，这些异常会打印一条失败消息，展开、清理堆栈并退出。通过设置环境变量，你还可以让Rust在发生异常时显示调用栈，以便更容易追踪异常的来源。

>响应异常时栈的展开或终止
>
>默认情况下，当发生异常时，程序开始展开（unwinding），这意味着Rust会沿着栈向上回溯，并清理它遇到的每个函数的数据。然而，回溯和清理是一项艰巨的工作。因此，Rust允许你选择立即中止（aborting）的替代方案，这将在不进行清理的情况下结束程序。
>
>然后，操作系统需要清理程序正在使用的内存。如果你在你的项目中需要使生成的二进制文件尽可能小，你可以通过在*Cargo.toml* 文件中将 `panic = 'abort'` 添加到适当的 `[profile]` 部分来从展开切换到 panic 时中止。例如，如果你想在 release 模式下在 panic 时中止，请添加以下内容：
>
>程序正在使用的内存将需要由操作系统清理。如果您的项目需要使生成的二进制文件尽可能小，您可以通过在Cargo.toml文件的[profile]部分添加`panic = 'abort'`来从展开切换到在异常时中止。例如，如果您想在生产模式下中止异常，请添加以下内容：
>
>```toml
>[profile.release]
>panic = 'abort'
>```



现在让我们尝试在一个简单的程序中调用`panic!`：

```rust
fn main() {
    panic!("crash and burn");
}
```

当您运行该程序时，您将看到如下内容：

```shell
/Users/wangyang/.cargo/bin/cargo run --color=always --package n09_panic --bin n09_panic --profile dev
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/n09_panic`
thread 'main' panicked at src/main.rs:2:5:
crash and burn
stack backtrace:
   0: rust_begin_unwind
             at /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/std/src/panicking.rs:662:5
   1: core::panicking::panic_fmt
             at /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/core/src/panicking.rs:74:14
   2: n09_panic::main
             at ./src/main.rs:2:5
   3: core::ops::function::FnOnce::call_once
             at /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

进程已结束，退出代码为 101
```

调用 `panic!` 会导致最后两行中包含的错误消息。第一行显示了我们的异常消息和源代码中发生异常的位置：`src/main.rs：2:5` 表示它是 *src/main.rs* 文件的第二行，第五个字符。

在这种情况下，指示的行是我们代码的一部分，如果我们跳转到该行，我们会看到 `panic!`宏调用。在其他情况下，`panic!`调用可能在我们的代码调用的代码中，错误消息报告的文件名和行号将是其他人代码中调用 `panic!`宏的地方，而不是最终导致 `panic!`调用的代码行。

我们可以使用 `panic!` 调用的函数的回溯来找出导致问题的代码部分。为了理解如何使用 `panic!` 回溯，让我们看另一个例子，看看 `panic!` 调用来自库时是什么感觉，因为我们代码中存在错误，而不是来自直接调用宏的代码。在下面的示例代码中，我们试图访问一个超出有效索引范围的向量索引。

```rust
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
```

在这里，我们试图访问向量的第100个元素（由于索引从0开始，所以它的索引值是99），但向量只有三个元素。在这种情况下，Rust会产生报错。使用`[]`应该返回一个元素，但是如果你传递了一个无效的索引，在这里Rust无法返回一个正确的元素。

在C语言中，尝试读取数据结构末尾之外的内容是未定义的行为。你可能会得到内存中对应于该数据结构元素的位置的任何东西，尽管该内存并不属于该结构。这被称为缓冲区越界读取，如果攻击者能够以这样一种方式操纵索引，以便读取他们不应该被允许访问的存储在数据结构之后的数据，这可能导致安全漏洞。

为了保护你的程序免受这种漏洞的影响，如果你试图读取一个不存在的索引位置的元素，Rust将会停止执行并拒绝继续。让我们尝试一下看看：

```shell
$ cargo run                     
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/n09_panic`
thread 'main' panicked at src/main.rs:4:6:
index out of bounds: the len is 3 but the index is 99
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

这个错误指向我们 *main.rs* 的第 4 行，我们试图访问向量 `v` 中的索引`99`。

注释：这一行告诉我们，我们可以设置RUST_BACKTRACE环境变量来获取导致错误的确切发生的回溯。回溯是调用所有函数以达到此点的列表。 Rust中的回溯与其他语言中的作用相同：阅读回溯的关键是从顶部开始阅读，直到看到您编写的文件。那就是问题起源的地方。该点上方的行是您的代码调用的代码；下方的行是调用您的代码的代码。这些前后的行可能包括核心Rust代码、标准库代码或您正在使用的板条箱。让我们尝试通过将RUST_BACKTRACE环境变量设置为除0以外的任何值来获取回溯。列表9-2显示了类似于您将看到的输出。

`note:` 这行告诉我们，我们可以设置 `RUST_BACKTRACE` 环境变量来回溯导致错误的确切原因。回溯跟踪是为到达此点而调用的所有函数的列表。Rust 中的回溯和其他语言一样工作：读取回溯的关键是从顶部开始阅读，直到看到你写的文件。这就是问题的根源。该点上方的行是您的代码调用的代码；下方的行是调用您的代码的代码。这些前后的行可能包括核心Rust代码、标准库代码或您正在使用的 crate。让我们尝试通过将 `RUST_BACKTRACE` 环境变量设置为除 `0` 之外的任何值来获取回溯。如下所示：

```shell
$ RUST_BACKTRACE=1 cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/n09_panic`
thread 'main' panicked at src/main.rs:4:6:
index out of bounds: the len is 3 but the index is 99
stack backtrace:
   0: rust_begin_unwind
             at /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/std/src/panicking.rs:662:5
   1: core::panicking::panic_fmt
             at /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/core/src/panicking.rs:74:14
   2: core::panicking::panic_bounds_check
             at /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/core/src/panicking.rs:276:5
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/core/src/slice/index.rs:302:10
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/core/src/slice/index.rs:16:9
   5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
             at /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/alloc/src/vec/mod.rs:2920:9
   6: n09_panic::main
             at ./src/main.rs:4:6
   7: core::ops::function::FnOnce::call_once
             at /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

你看到的确切输出可能会因你的操作系统和Rust版本而有所不同。为了获取带有此信息的回溯，必须启用调试符号。当我们使用 `cargo build` 或 `cargo run` 而不带`--release`标志时，默认情况下会启用调试符号，就像我们在这里所做的那样。

在上面的输出中，回溯的第6行指向我们项目中导致问题的行：src/main.rs的第4行。如果我们不想让程序出现异常，我们应该从提到我们编写的文件的第一个行所指向的位置开始调查。在前面的代码中，我们故意编写了会导致异常的代码，修复异常的方法是不要请求超出向量索引范围的元素。当您的代码在未来出现异常时，您需要找出代码正在使用哪些值执行哪些操作来导致异常，以及代码应该做什么来替代。

接下来，我们将了解如何使用 `Result` 从错误中恢复。