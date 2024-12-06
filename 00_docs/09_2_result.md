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

 `panic!`不管 `File::open` 失败的原因是什么。但是，我们希望针对不同的失败原因采取不同的操作。如果`File::open` 因为文件不存在而失败，我们想要创建文件并将句柄返回给新文件。如果 `File::open` 因为任何其他原因失败，例如，因为我们没有打开文件的权限，我们仍然希望代码 `panic!`就像在上面代码所做的那样。为此，我们添加了一个内部 `match` 表达式，如下所示：

```rust

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };
}
```

`File::open` 在 `Err` 变体中返回的值的类型是`io::Error`，它是标准库提供的结构体。这个结构有一个方法`kind`，我们可以调用它来获取 `io::ErrorKind` 值。枚举`io::ErrorKind` 由标准库提供，它包含了表示可能由I/O操作导致的不同错误类型的成员，我们想要使用的成员是 `ErrorKind::NotFound`，它表示我们尝试打开的文件尚不存在。所以我们在`greeting_file_result` 上匹配，但我们在 `error.kind()` 上也有一个内部匹配。

我们要在内部匹配中检查的条件是 `error.kind()` 返回的值是否是 `ErrorKind` 枚举的 `NotFound` 成员。如果是，我们尝试使用 `File::create` 创建文件。但是，由于 `File::create`也可能失败，因此我们需要在内部 `match` 表达式中使用第二个分支。当无法创建文件时，将打印不同的错误消息。外部`match`第二个分支保持不变，因此程序会因除缺少文件错误之外的任何错误都会抛出异常。

但是这个代码中我们使用了大量的`match`，`match` 表达式非常有用，但也非常原始。在代码中处理 `Result<T, E>` 值时，使用闭包可能比使用 `match` 更简洁。通过闭包和和 `unwrap_or_else` 方法，能够使我们这个代码更加简洁。

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}
```

在处理错误时，还有很多这类方法可以消除大量嵌套的 `match` 表达式。

## unwrap and expect

使用 `match` 效果很好，但它可能有点冗长，并且并不总是能很好地传达意图。`Result<T, E>` 类型定义了许多帮助程序方法来执行各种更具体的任务。`unwrap` 方法是一个快捷的方法，实现就像我们写的 `match` 表达式一样。如果 `Result` 值是 `Ok` 变体，`unwrap` 将返回 `Ok`，如果 `Result` 是 `Err` 成员，`unwrap` 将为我们调用 `panic!`宏。下面是一个 `unwrap` 实际应用的一个例子：

```rust
use std::fs::File;

fn main() {
    let _greeting_file = File::open("hello.txt").unwrap();
}
```

现在我们删除文件中的`hello.txt`来看看在本地没有相应文件时候，运行该代码看看会发生什么情况。

```shell
/Users/wangyang/.cargo/bin/cargo run --color=always --package n09_result --bin n09_result --profile dev
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/n09_result`
thread 'main' panicked at src/main.rs:49:50:
called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

很明显，我们看到来自 `unwrap` 方法发出的 `panic!`调用的错误消息，类似地，`expect` 方法也允许我们选择 `panic!` 错误消息。使用 `expect` 代替 `unwrap` 可以提供更好的错误消息，这会让排查异常的来源更容易。`expect` 的语法如下所示：

```rust
use std::fs::File;

fn main() {
    let _greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}
```

我们使用 `expect` 的方式与 `unwrap` 相同：返回文件句柄或调用 `panic!` 宏。`expect` 在调用 `panic!`将是传递给 `expect` 的参数，而不是 `unwrap` 使用的默认`panic!` 消息。这是它的样子：

```shell
/Users/wangyang/.cargo/bin/cargo run --color=always --package n09_result --bin n09_result --profile dev
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/n09_result`
thread 'main' panicked at src/main.rs:57:10:
hello.txt should be included in this project: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

在生产质量的代码中，大多数 Rustacean 选择 `expect` 而不是`unwrap`，并给出更多关于为什么操作预期总是成功的上下文。这样，如果您的假设被证明是错误的，您就有更多信息可用于调试。



## 传播错误

当函数的实现调用可能失败的内容时，你可以将错误返回给调用代码，以便它可以决定做什么，而不是在函数本身中处理错误。这称为**传播**（*propagating*）错误，并赋予调用代码更多控制权，其中可能有更多的信息或逻辑来指示应如何处理错误，而不是代码上下文中可用的信息或逻辑。

例如，下面的代码展示了一个从文件中读取用户名的函数。如果文件不存在或无法读取，此函数会将这些错误返回给调用该函数的代码。

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
```

