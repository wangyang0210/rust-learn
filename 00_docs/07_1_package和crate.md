# 前言

Rust 具有许多功能，允许您管理代码的组织，包括公开哪些内容、哪些内容是私有的以及程序中每个作用域中的名称。这些功能有时统称为***模块系统***，包括：

- **Packages：**一个 Cargo 功能，可让您构建、测试和共享 crate

- **Crates：**生成库或可执行文件的模块树
- **Modules**：用于控制作用域和路径的私有性
- **Paths：**一种命名的方法，例如结构、函数或模块

本章将会涵盖所有这些概念，讨论它们如何交互，并说明如何使用它们来管理作用域。到最后，你会对模块系统有深入的了解，并且能够像专业人士一样使用作用域！

# 内容

首先我们将介绍的模块系统的第一部分 package 和 crate。

*crate* 是 Rust 编译器一次考虑的最小代码量。即使你运行 `rustc` 而不是 `cargo` 并传递一个源代码文件，编译器也会认为该文件是一个 crate。crate 可以包含模块，并且这些模块可以使用 crate 编译的其他文件中定义，正如我们将在接下来的部分中看到的那样。

crate 可以有两种形式：二进制 crate 或库 crate。*二进制 crate* 是可以编译为可运行的可执行文件的程序，例如命令行程序或服务器。每个 API 都必须有一个名为 `main` 的函数，该函数定义可执行文件运行时发生的情况。到目前为止，我们创建的所有 crate 都是二进制 crate。

*库 crate* 没有 `main` 函数，它们不会编译为可执行文件。相反，它们定义了要与多个项目共享的功能。例如，我们在猜谜游戏中使用的 `rand` crate 提供了生成随机数的功能。*crate 根*是 Rust 编译器从源文件开始，并构成了 crate 的根模块。

软件包是一个或多个 crate 的捆绑包，可提供一组功能。一个包，包含一个 *Cargo.toml* 文件，该文件描述了如何构建这些 crate。Cargo 实际上是一个包，其中包含您用于构建代码的命令行工具的二进制 crate。Cargo 包还包含二进制 crate 所依赖的库 crate。其他项目可以依赖 Cargo 库 crate 来使用 Cargo 命令行工具使用的相同逻辑。一个包可以包含任意数量的二进制 crate，但最多只能包含一个库 crate。一个包必须至少包含一个 crate，无论是库crate还是二进制 crate。

让我们来看看创建 package 时会发生什么。首先，我们输入命令 `cargo new my-project`：

```shell
$ cargo new my-project
         Creating binary (application) `my-project` package
				 note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

$ ll my-project
total 8
-rw-r--r--@ 1 wangyang  staff    81B Nov 14 17:29 Cargo.toml
drwxr-xr-x@ 3 wangyang  staff    96B Nov 14 17:29 src


$ ll my-project/src
total 8
-rw-r--r--@ 1 wangyang  staff    45B Nov 14 17:29 main.rs
```

在我们运行 `cargo new my-project` 之后，我们使用 `ll` 来查看 Cargo 创建的内容。在项目目录中，有一个 *Cargo.toml* 文件，为我们提供了一个包。还有一个包含 *main.rs* 的 *src* 目录。在文本编辑器中打开 *Cargo.toml*，请注意没有提到 *src/main.rs*。Cargo 遵循一个约定，*即 src/main.rs* 是与包同名的二进制 crate 的 crate 根。同样，Cargo 知道如果 package 目录包含 *src/lib.rs*，则该 package 包含一个与该 package 同名的库 crate，*而 src/lib.rs* 是其 crate 根。Cargo 将 crate 根文件传递给 `rustc` 以构建库或二进制文件。

在这里，我们有一个只包含 *src/main.rs* 的包，这意味着它只包含一个名为 `my-project` 的二进制 crate。如果一个包包含 *src/main.rs* 和 *src/lib.rs*，它有两个 crate：一个 binary 和一个 library，它们都与包同名。通过将文件放在 *src/bin* 目录中，一个包可以有多个二进制 crate：每个文件将是一个单独的二进制 crate。
