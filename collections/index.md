# 常见集合

## vector

### 创建 vector

```rust
  // 创建空vector
  let vec: Vec<i32> = Vec::new();
  // 通过vec!宏
  let vec2 = vec![1, 2, 3];
  // 添加元素
  let mut vec3 = vec![1];
  vec3.push(2);
```

当 vector 离开作用域时，其所有权也会被销毁

```rust
{
  let vec = vec![1,2,3];
}
// 被销毁
```

如果 vec 当值存储的是基本数据的话，事情还算正常，如果存储的是引用的话，就比较麻烦了。

### 读取 vector

读取 vector 有两种方式，一个是直接通过下标读取，一个是通过 get 方法读取。
get 方法的返回值是一个 Option 类型。如果获取的位置超出 vec 的长度，系统就会出现恐慌，如果不希望出现恐慌的话可以是用 Option 来处理异常情况。

```rust
  let vec = vec![1, 2, 3, 4, 5];

  // &i32
  let first = &vec[0];
  // Option<&i32>
  let second = vec.get(1);
```

同一个作用域值不能同时存在可变医用和不可变引用，这个规则在 vec 中也适用。

```rust
  let mut vec = vec![1, 2, 3, 4, 5];

  let first = &vec[0];

  // cannot borrow `vec` as mutable because it is also borrowed as immutable
  // mutable borrow occurs here
  vec.push(7);

  println!("{}", first);
```

为什么只引用 vec 的第一个元素也会被这个规则限制呢，我们又不是引用了整个 vec，这是因为 vec 在 push 的过程中可能会扩容，这时候第一个元素在内存中的位置会可能会发生变化。

### 遍历 vector 中的元素

通过 for ... in 对 vec 进行遍历。

```rust
  let mut vec = vec![1, 2, 3, 4, 5, 6];

  for mut i in &mut vec {
      *i += 10;
  }

  for i in vec {
      println!("{}", i)
  }
```

### 使用枚举来储存多种类型

```rust
  enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
  }

  let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
  ];
```

之所以使用 vec 前要确定每个元素的类型，因为这样才能确定声明的这个 vec 要在内存中分配多大的空间。

## String

rust 的核心语言中只有一种字符串，那就是 str，也就是字符串 slice，String 类型是由标准库实现的。

### 创建/更新字符串

```rust
  // String
  let mut string = String::from("hello");
  // &str
  let str = "hello2";

  // str -> String
  let str3 = str.to_string();

  // 相加
  let str4 = string.push_str("world");
  let str5 = string + str;
  let str6 = format!("{}{}{}", str5, str, str3);
```

## 哈希 map

### 创建 hashmap

```rust
  use std::collections::HashMap;

  let mut map = HashMap::new();
  map.insert("name", "zhanglu");
  map.insert("name2", "zhanglu2");
```

### 哈希 map 和所有权

```rust
  let field_name = String::from("Favorite color");
  let field_value = String::from("Blue");

  let mut map = HashMap::new();
  map.insert(field_name, field_value);
  // 所有权报错
  // 可以在insert的时候插入引用，但是要保证引用在hashmap的生命周期中是一直有效的
  println!("{}{}", field_name, field_value);
```

使用 get 方法来获取 hashmap 的值，使用 entry 方法更新不存在的 key。
