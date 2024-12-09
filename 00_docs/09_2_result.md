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

这个函数可以用更短的方式编写，但是我们打算先手动完成大部分工作以便探索错误处理；最后，我们将展示更短的方法。让我们看看 函数的返回类型：`Result<String, io::Error>`。这意味着该函数返回 `Result<T, E>` 类型的值，其中泛型参数 `T` 已填充为具体类型 `String`，泛型类型 `E` 已填充为具体类型 `io::Error`。

如果此函数执行成功且没有任何问题，调用这个函数的代码将收到一个 `Ok` 值，该值包含一个 `String`，即此函数从文件中读取的`username`。如果此函数遇到任何问题，调用代码将接收一个 `Err` 值，该值包含一个 `io::Error`实例，其中包含有关问题所在的更多信息。我们选择`io::Error` 作为这个函数的返回类型，因为它恰好是我们在这个函数体中调用的两个可能失败的操作返回的错误值的类型：`File::open` 函数和`read_to_string` 方法。

函数的主体首先调用 `File::open` 函数。然后 `match` 处理 `Result` 值，如果 `File::open` 成功，则模式变量 `file`中的文件句柄将成为可变变量 `username_file` 中的值，并且函数继续执行 。在 `Err` 情况下，我们不是调用 `panic!`，而是使用 `return`关键字提前完全退出函数，并将错误值从 `File::open`（现在在模式变量 `e` 中）作为该函数的错误值返回给调用代码。

因此，如果我们在 `username_file` 中有一个文件句柄，该函数就会在变量 `username` 中创建一个新的 `String`，并在 `username_file` 中对文件句柄调用 `read_to_string` 方法，将文件内容读入`username`。`read_to_string` 方法返回 `Result`，因为即使 `File::open` 成功，它也可能会失败。所以我们需要另一个`match`来处理这个 `Result`：如果 `read_to_string` 成功，那么我们的函数就成功了，我们从现在包含在 `Ok` 中的 `username`文件中返回 username。如果`read_to_string`失败，我们返回错误值的方式与在处理 `File::open` 返回值的`match`返回错误值的方式相同。但是，我们不需要明确的`return`，因为这是函数中的最后一个表达式。

然后，调用此代码的代码将处理获取包含用户名的 `Ok` 值或包含 `io::Error` 的 `Err` 值。由调用代码决定如何处理这些值。如果调用代码获得 `Err` 值，它可以调用 `panic!`并导致程序崩溃，使用默认用户名，或者从文件以外的其他位置查找用户名，例如。我们没有足够的信息来了解调用代码实际尝试做什么，因此我们将所有成功或错误信息向上传播，以便它进行适当的处理。

这种传播错误的模式在 Rust 中非常常见，以至于 Rust 提供了问号运算符 `?`来简化此操作。



## 传播错误的快捷方式：? 操作符

现在让我们来尝试基于`?`来实现`read_username_from_file` :

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
```

放在 `Result` 值后面的 `?`被定义为与我们为处理 `Result` 值而定义的 `match` 表达式的工作方式几乎相同。如果 `Result` 的值为 `Ok`，则 `Ok` 中的值将从此表达式返回，并且程序将继续。如果 value是 `Err`，则 `Err` 将从整个函数返回，就像我们使用了 `return` 关键字一样，因此错误值会传播到调用代码。

 `match` 表达式和 `?`运算符的作用是不同的：调用 `?`运算符的错误值会通过 `from` 函数，该函数在标准库的 `From` trait 中定义，用于将值从一种类型转换为另一种类型。当 `?`运算符调用 `from` 函数时，接收到的错误类型会被转换为当前函数返回类型中定义的错误类型。这在函数返回一种错误类型来表示函数可能失败的所有方式时非常有用，即使部分原因可能有很多不同的原因导致失败。

例如，我们可以更改 `read_username_from_file` 函数，以返回我们定义的名为 `OurError` 的自定义错误类型。如果我们还为 `impl From<io::Error> for OurError` ，以便从 `io::Error` 构造`OurError` 的实例，那么`read_username_from_file` 主体中的 `?`运算符调用将调用 `from` 并转换错误类型，而无需向函数添加更多代码。

`File::open` 调用末尾的 `?`会将 `Ok` 中的值返回给变量 `username_file`。如果发生错误，`?`运算符将提前返回整个函数，并将任何 `Err` 值提供给调用代码。同样的事情也适用于 `read_to_string` 调用末尾的 `?`。

`?`运算符让我们的代码更加的简洁，我们甚至可以通过在链式方法调用之后立即调用 `?` 来进一步缩短这段代码，如下所示。

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
```

我们已将 `username` 中新 `String` 的创建移动到函数的开头；这部分没有改变。我们没有创建变量`username_file`，而是将对 `read_to_string` 的调用直接链接到 `File::open("hello.txt")?`上。在`read_to_string` 调用的末尾仍然有一个 `?`，当 `File::open` 和 `read_to_string` 都成功时，我们仍然返回一个包含 `username`的 `Ok` 值，而不是返回错误。

现在我们尝试使用 `fs::read_to_string` 来缩短它的方法，如下所示：

```rust
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

将文件读入 String 是一个相当常见的操作，因此标准库提供了方便的 `fs::read_to_string` 函数，该函数打开文件，创建新的 `String`，读取文件的内容，将内容放入该 `String` 中，然后返回它。当然，使用 `fs::read_to_string`并不能给我们解释所有错误处理的机会，因此我们先做了更长的方法。

## 哪里可以使用?操作符

`?` 只能用于返回类型与使用 `?` 的值兼容的函数。这是因为 `?`运算符被定义为从函数中提前返回一个值，与我们在示例中定义的 `match` 表达式相同。`match`使用了 `Result` 值，而提前返回了一个`Err(e)` 值。函数的返回类型必须是 `Result`，以便与此次`return`兼容。

在下面的示例中，让我们看看如果我们在一个返回类型与我们使用`?`操作符的值类型不兼容的 `main` 函数中使用 `?`运算符，我们会得到什么错误。

```rust
use std::fs::File;

fn main() {
    let _greeting_file = File::open("hello.txt")?;
}
```

因为`?` 运算符遵循 `File::open` 返回的 `Result`值，但此`main`函数的返回类型为 `()`而不是 `Result`。当我们编译此代码时，我们会收到以下错误消息：

```shell
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
   --> src/main.rs:100:49
    |
99  | fn main() {
    | --------- this function should return `Result` or `Option` to accept `?`
100 |     let _greeting_file = File::open("hello.txt")?;
    |                                                 ^ cannot use the `?` operator in a function that returns `()`
    |
    = help: the trait `FromResidual<Result<Infallible, std::io::Error>>` is not implemented for `()`
help: consider adding return type
    |
99  ~ fn main() -> Result<(), Box<dyn std::error::Error>> {
100 |     let _greeting_file = File::open("hello.txt")?;
101 +     Ok(())
    |
```

此错误指出我们只允许在返回 `Result`、`Option` 或其他实现`FromResidual` 的类型的函数中使用 `?`运算符。

要修复此错误，您有两种选择。一种选择是更改函数的返回类型，使其与您正在使用的值兼容 `?`操作符，只要您没有限制阻止这样做。另一种选择是使用`match`或 `Result<T, E>` 方法之一来处理 `Result<T, E>`以任何合适的方式。

错误消息还提到 `?`也可以与 `Option<T>` 值一起使用。与在 `Result` 上使用`?`一样，你只能在返回`Option`的函数中使用 `?`。在 `Option<T>` 上调用 `?` 运算符的行为与其在`Result<T, E>`上调用的行为类似：如果值为None，则在该点提前从函数返回`None`。如果值为`Some`，则`Some`内的值是表达式的结果值，函数继续执行。下面有一个示例函数，用于查找给定文本中第一行的最后一个字符。

```	rust
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn main() {
    assert_eq!(
        last_char_of_first_line("Hello, world\nHow are you today?"),
        Some('d')
    );

    assert_eq!(last_char_of_first_line(""), None);
    assert_eq!(last_char_of_first_line("\nhi"), None);
}
```

这个函数返回 `Option<char>`，因为那里可能有一个字符，但也有可能没有。这段代码接收一个文本字符串切片参数，并对其调用 `lines` 方法，该方法返回一个迭代器，用于遍历字符串中的行。由于此函数要检查第一行，因此它在迭代器上调用 `next` 以从迭代器获取第一个值。如果 `text` 是空字符串，则对 `next` 的调用将返回 `None`，在这种情况下，我们使用 `?`停止并从`last_char_of_first_line` 返回 `None`。如果 `text` 不是空字符串，`next` 将返回一个包含 `text` 中第一行的字符串切片的`Some` 值。

`?` 提取字符串切片，我们可以在此字符串切片上调用 `chars` 来获取其字符的迭代器。我们对第一行中的最后一个字符感兴趣，因此我们调用 `last` 以返回迭代器中的最后一项。这是一个 `Option`，因为有可能第一行是空字符串；例如，如果`text`以空白行开头，但在其他行上包含字符，如 `"\nhi"`但是，如果第一行上有最后一个字符，它将在 `Some` 变体中返回。中间的 `?`运算符为我们提供了一种简洁的方式来表达这个逻辑，使我们能够在一行中实现该功能。如果我们不能在 `Option` 上使用 `?`运算符，我们必须使用更多的方法调用或 `match` 表达式来实现这个逻辑。

请注意，在返回`Result`的函数中，您可以在`Result上`使用`?`操作符，在返回`Option`的函数中，您可以在`Option`上使用`?`操作符，但不能混用。`?`操作符不会自动将`Result`转换为`Option`，反之亦然；在这些情况下，您可以使用`Result`上的`ok`方法或`Option`上的`ok_or`方法等方法来明确地进行转换。

到目前为止，我们使用的所有`main`函数都返回 `()`，`main` 函数很特殊，因为它是可执行程序的入口点和出口点，而且它的返回类型有限制，以便程序按预期运行。

幸运的是，`main` 还可以返回 `Result<(), E>`。在下面的例子中，我们将 `main` 的返回类型更改为`Result<(), Box<dyn Error>>`并在末尾添加了返回值 `Ok(())`，这样后这个代码就能编译了。

```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let _greeting_file = File::open("hello.txt")?;
    Ok(())
}
```

`Box<dyn Error>` 类型是一个 *trait 对象*，您可以将 `Box<dyn Error>` 读作“任何类型的错误”。在具有错误类型为 `Box<dyn Error>` 的`main`函数中使用 `?`是允许的，因为它允许提前返回任何 `Err` 值。即使此 `main` 函数的主体只会返回 `std::io::Error` 类型的错误，但通过指定 `Box<dyn Error>`，即使将更多返回其他错误的代码添加到 `main` 主体中，此签名也将继续正确。

当 `main` 函数返回 `Result<(), E>` 时，如果 `main` 返回 `Ok(())`则可执行文件将以值 `0` 退出，如果`main` 返回 `Err` 值，则可执行文件将以非零值退出。用 C 语言编写的可执行文件在退出时返回整数：成功退出的程序返回整数 `0`，而出错的程序返回非 `0` 的整数。为了与这个约定兼容，Rust也从可执行文件中返回整数。

`main` 函数可以返回实现 [`std::process::Termination` 特征](https://doc.rust-lang.org/std/process/trait.Termination.html)的任何类型的类型，其中包含返回 `ExitCode` 的函数`report`。有关为您自己的类型实现 `Termination` trait 的更多信息，请参阅标准库文档。

现在我们已经讨论了调用 `panic!`或返回 `Result` 的细节，让我们回到如何决定在哪些情况下使用哪个合适的话题。

