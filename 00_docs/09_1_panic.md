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