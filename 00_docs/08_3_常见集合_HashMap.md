# 前言

我们最后一个常见的集合是哈希映射。类型`HashMap<K, V>`使用哈希函数存储类型K的键到类型V的值的映射，这决定了它如何将这些键和值放入内存中。许多编程语言都支持这种数据结构，但它们通常使用不同的名称，例如哈希、映射、对象、哈希表、字典或关联数组等。



# 内容

当你想通过使用键（可以是任何类型）而不是使用索引（如向量中所做的那样）来查找数据时，哈希映射是很有用的。例如，在游戏中，你可以在一个哈希映射中跟踪每个团队的得分，其中每个键都是一个团队的名称，值是每个团队的得分。给定一个团队名称，你可以检索它的得分。

# 内容

在本节中，我们将介绍哈希映射的基本 API，但标准库在 `HashMap<K, V>` 上定义的函数中隐藏了更多好东西。与往常一样，请查看标准库文档以获取更多信息。



## 创建HashMap

通过使用 `new` 并使用`insert` 添加元素，来创建一个空的hash map。在下面的示例中，我们跟踪了两支球队的分数，他们的名字是 ***Blue*** 和 ***Yellow***。蓝队以 10 分开始，黄队以 50 分开始。

```rust
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}
```

请注意，我们首先需要使用标准库中集合部分的 `HashMap`。在我们三种常见的集合中，这个集合是最不常用的，所以它没有包括在预加载范围内自动引入的功能中。`HashMap` 也较少受到标准库的支持；例如，没有内置的宏来构造它们。

就像向量一样，`HashMap`将它们的数据存储在堆上。这个`HashMap`有`String`类型的键和`i32`类型的值。像向量一样，哈希映射是同质的：所有的键必须具有相同的类型，所有的值也必须具有相同的类型。

## 访问HashMap

我们可以通过将哈希映射的`key` 提供给 `get`方法，来从哈希映射中获取一个值，如下所示：

```rust
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
}
```

在这里，`score` 将具有与蓝队关联的值，结果将为 `10`。`get` 方法返回 `Option<&V>`; 如果哈希映射中该键没有值，`get` 将返回 `None`。该程序通过调用 `copied` 来处理`Option`，获取 `Option<i32>` 而不是`Option<&i32>`，如果 `scores` 没有键的条目，`unwrap_or`将 `score` 设置为零。

我们可以像使用向量一样，使用for循环以类似的方式遍历哈希映射中的每一个键值对：

```rust
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
```

这个代码将以任意顺序打印每对元素：

```shell
/Users/wangyang/.cargo/bin/cargo run --color=always --package n08_hashMap --bin n08_hashMap --profile dev
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/n08_hashMap`
Yellow: 50
Blue: 10

进程已结束，退出代码为 0
```



## HashMap和所有权

对于实现 `Copy` trait 的类型，如 `i32`，值将被复制到哈希映射中。对于像 `String` 这样的拥有所有权的值，值会被移动，哈希映射将成为这些值的所有者，如下所示：

```rust
fn main() {
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
}
```

我们无法使用变量 `field_name` 和 `field_value` 在通过调用 `insert` 将其移动到哈希映射中后。

如果我们向哈希映射中插入对值的引用，这些值不会被移动到哈希映射中。引用所指向的值必须至少在哈希映射有效时同样有效。



## 更新HashMap

虽然键值对的数量是可增长的，但每个唯一的键一次只能关联一个值（反之则不成立：例如，蓝队和黄队都可能在分数哈希映射中存储值10）。

当你想在一个哈希映射中改变数据时，你必须决定如何处理一个key已经分配了一个值的情况。你可以用新值替换旧值，完全忽略旧值。你可以保留旧值并忽略新值，只有当key还没有值的时候才添加新值。或者你可以将旧值和新值结合起来。让我们看看如何做这些事情！

### 覆盖值

如果我们将一个 key 和一个值插入到`hashMap` 中，然后插入具有不同值的相同 key，则与该 key 关联的值将被替换。即使示例 8-23 中的代码调用了两次 `insert`，哈希映射也只包含一个键值对，因为我们两次都插入了蓝队的键对应的值。

```rust
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");
}
```

运行后，我们将得到以下结果：

```shell
/Users/wangyang/.cargo/bin/cargo run --color=always --package n08_hashMap --bin n08_hashMap --profile dev
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/n08_hashMap`
{"Blue": 25}
```

很明显原始值10被覆盖了。

## key不存在时才插入key和value

通常需要检查哈希映射中是否已经存在特定键和对应的值，然后采取以下操作：如果该键确实存在于哈希映射中，则保持现有值不变；如果不存在，则插入该键和其对应的值。

哈希映射有一个特殊的API，称为`entry`，它将您要检查的键作为参数。`entry`方法的返回值是一个名为`Entry`的枚举，表示可能存在或不存在的值。假设我们想检查黄队的键是否有与之关联的值。如果没有，我们想要插入值50，蓝队也是如此。使用`entry `API，代码如下所示：

```rust
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");
}
```

`Entry` 上的 `or_insert`方法被定义为：如果相应的` Entry `键存在，则返回该键对应值的可变引用；如果不存在，则将参数插入为该键的新值，并返回新值的可变引用。这种技术比我们自己编写逻辑要清晰得多，而且，此外，它与借用检查器的配合更好。

运行代码将打印 `{"Yellow": 50, "Blue": 10}`。第一次调用 `entry` 将插入值为`50` 的黄队的键，因为黄队还没有值。对`entry` 的第二次调用不会更改哈希映射，因为蓝队已经存在值 `10`。

## 根据旧值更新值

哈希映射的另一个常见用例是查找键的值，然后根据旧值更新它。例如，下面的代码显示了计算某个文本中每个单词出现次数的代码。我们使用一个哈希映射，以单词作为键，并递增该值来跟踪我们已经见过该单词的次数。如果我们是第一次看到一个单词，我们将首先插入值0。

```rust
fn main() {
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
```

此代码将打印 `{"world": 2, "hello": 1, "wonderful": 1}`。你可能会看到相同的键值对以不同的顺序打印出来，遍历哈希映射是以任意顺序进行的。

`split_whitespace`方法返回一个迭代器，该迭代器按空白字符分隔`text`中的值的子切片。`or_insert`方法返回指定键的值的可变引用（`&mut V`）。在这里，我们将该可变引用存储在`count`变量中，因此为了给该值赋值，我们必须首先使用星号（`*`）对`count`进行解引用。可变引用在for循环结束时超出作用域，因此所有这些更改都是安全的，并且符合借用规则。

## 哈希函数

默认情况下，`HashMap` 使用一种称为 *SipHash*  的哈希函数，该函数可以抵御涉及哈希表[1](https://en.wikipedia.org/wiki/SipHash) 的拒绝服务 （DoS） 攻击。。这不是最快的哈希算法，但是为了更好的安全性而付出的性能下降的代价是值得的。如果你对你的代码进行性能分析，发现默认的哈希函数对于你的目的来说太慢了，你可以通过指定一个不同的哈希器来切换到另一个函数。哈希器是实现`BuildHasher` trait 的类型。你不必从头开始实现自己的哈希器；[crates.io](https://crates.io/) 有由其他 Rust 用户共享的库，提供了实现许多常见哈希算法的哈希器。



# 总结

向量、字符串和哈希映射将提供大量功能，当您需要存储、访问和修改数据时，这些功能在程序中是必需的。以下是您现在应该准备好解决的一些练习：

1. 给定一个整数列表，使用一个向量并返回列表的中位数（排序时，中间位置的值）和众数（最常出现的值；哈希映射在这里会有所帮助）。
2. 将字符串转换为 pig 拉丁语。每个单词的第一个辅音被移动到单词的末尾，并加上 *ay*，所以*first*辅音变成了 *irst-fay*。以元音开头的单词在末尾添加了 *hay* （*apple* 变成*apple-hay*）。请记住有关 UTF-8 编码的详细信息！
3. 使用哈希映射和向量，创建一个文本界面，以允许用户将员工姓名添加到公司的部门；例如，“将Sally添加到工程部门”或“将Amir添加到销售部门”。然后，让用户按部门检索部门中所有人员或公司中所有人员的列表，按字母顺序排序。

标准库 API 文档描述了向量、字符串和哈希映射所具有的方法，这些方法将对这些练习有所帮助！

我们正在研究操作可能会失败的更复杂的程序，因此现在是讨论错误处理的最佳时机。我们接下来会这样做！