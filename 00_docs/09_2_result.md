# 前言

大多数错误没有严重到需要程序完全停止的程度。有时，当函数失败时，这是由于您可以轻松解释和响应的原因。例如，如果您尝试打开一个文件，但该操作失败，因为该文件不存在，您可能希望创建该文件，而不是终止进程。



# 内容

这里我们就考虑到在最开始的时候，我们在猜谜游戏中使用`Result` 枚举被定义为具有两个成员，`Ok` 和 `Err`，如下所示：

```rust
#![allow(unused)]
fn main() {
  enum Result<T, E> {
      Ok(T),
      Err(E),
  }
}
```

`T` 和 `E` 是泛型类型参数，您现在需要知道的是，`T` 表示在 `Ok`成员中成功案例中将返回的值的类型，`E` 表示在 `Err` 成员中失败时将返回的错误类型。因为 `Result` 有这些泛型类型参数，所以我们可以在许多不同的情况下使用 `Result` 类型和它定义的函数，在这些情况下，我们想要返回的成功值和错误值可能不同。

让我们调用一个返回 `Result` 值的函数，因为该函数可能会失败。如下所示，我们尝试打开一个文件。

```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");
}
```

`File::open`的返回类型是一个`Result<T, E>`。泛型参数`T`已被`File::open`的实现填充，其类型是一个文件句柄。错误值中使用的`E`的类型是`std::io::Error`。这种返回类型意味着对`File::open`的调用可能会成功，并返回一个我们可以读取或写入的文件句柄。函数调用也可能会失败：例如，文件可能不存在，或者我们可能没有权限访问文件。`File::open`函数需要有一种方法来告诉我们它是成功还是失败，同时给我们提供文件句柄或错误信息。这些信息正是`Result`枚举所传达的。

在 `File::open` 成功的情况下，变量`greeting_file_result` 中的值将是包含文件句柄的 `Ok` 实例。如果失败，`greeting_file_result` 中的值将是中的值将是一个包含有关发生错误类型更多信息的 `Err` 的实例。

为了根据 `File::open` 返回的值执行不同的操作，现在我们来调整下代码。

```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
}
```

请注意，与 `Option` 枚举一样，`Result` 枚举及其成员已通过预导入引入到作用域中，因此我们不需要在`match`分支中的 `Ok` 和 `Err` 变体之前指定 `Result::`。

当结果为 `Ok` 时，此代码将从 `Ok` 成员中返回内部`file`值，然后将该文件 句柄值分配给变量`greeting_file`。匹配后，我们可以使用文件句柄进行读取或写入。

`match` 的另一分支处理我们从`File::open` 获取 `Err` 值的情况。在此示例中，我们选择调用 `panic!` 宏。如果当前目录中没有名为 *hello.txt* 的文件，我们运行这段代码，我们将从`panic!`宏看到以下输出：

```shell
warning: `n09_result` (bin "n09_result") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.66s
     Running `target/debug/n09_result`
thread 'main' panicked at src/main.rs:8:23:
Problem opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

```

像往常一样，这个输出准确地告诉我们出了什么问题。



## 匹配不同的错误

