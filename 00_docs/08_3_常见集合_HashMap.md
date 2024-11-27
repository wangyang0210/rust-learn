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

