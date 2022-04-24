# 所有权

所有程序都必须管理其运行时使用计算机内存的方式。一些语言中具有垃圾回收机制，在程序运行时不断地寻找不再使用的内存；在另一些语言中，程序员必须亲自分配和释放内存。Rust 则选择了第三种方式：通过所有权系统管理内存，编译器在编译时会根据一系列的规则进行检查。如果违反了任何这些规则，程序都不能编译。在运行时，所有权系统的任何功能都不会减慢程序。

## 所有权规则

- rust 中每一个值都有一个所有者的变量
- 值在任一时刻只能有一个所有者
- 当所有者离开作用域，这个值就会被销毁

### 字符串

字符串字面量是被硬编码到执行文件中的字符串，是不可被修改的。

```rust
  let str = "111";
```

通过 from 来创建字符串，String::from 的意思就是在 String 的命名空间调用 from 方法。

```rust
  let mut str = String::from("hello");
  str.push_str(" world");
  println!("str is {}", str);
```

为什么 String 可变而字面量不可变，因为他们在内存上的处理是不一样的。字符串字面量是直接硬编码到最后执行的二进制文件中，而很多情况我们是无法预知这个字符串的内容，例如处理用户输入内容。为了支持这么一个可变、可增长的字符串文本，我们需要在堆中开辟出一块空间，这就意味着由以下两步。

- 在运行时向内存分配器请求内存
- 在处理完 String 以后将内存返还给内存分配器

第一步我们通过 String::from 来完成。
第二步就是我们常说的垃圾回收机制，有的语言通过 GC 来进行回收，有的需要手动回收，而 rust 采用的就是所有权的方式，即：当变量离开他所在的作用域时，这个变量所占用的内存将会被释放掉。

```rust
  {
    let s = String::from("hello"); // 从此处起，s 是有效的

    // 使用 s
  } // 此作用域已结束，
    // s 不再有效
```

### 值的移动

```rust
  let x = 5;
  let y = x
```

整型的值大小是固定的，所以整型的变量是存在栈中的，声明一个 x = 5，然后将 x 的值拷贝到 y，这两个值都被压入了栈中。

```rust
  let s1 = String::from("hello");
  let s2 = s1;
```

换成 String 类型就不一样了，String 会在栈中存放一个指针，指针指向堆内存，在堆内存中存放字符串的值。

```rust

栈         堆

s1   ->    "hello"
s2   ->

```

这时的 s1 和 s2 指向同一个 Hello 字符串。
之前我们提到过，一个变量在离开作用域后，这个变量所占用的内存就会被释放。那这时 s1 和 s2 同时离开作用域的话，则会对 Hello 所占用的内存释放两次，也就是二次释放问题，会造成内存污染和安全问题。因此 Rust 不需要在 s1 在离开作用域后做任何事情，看看在 s2 创建以后再访问 s1 会发生什么。

```rust
  let s1 = String::from("hello");
  let s2 = s1;
  println!("s1 is {}", s1) // borrow of moved value: `s1` value borrowed here after move
```

在其他语言中，s2 = s1 的操作我们通常称之为浅拷贝。但是在 Rust 中我们将其称之为移动，移动后的变量就不能通过之前的指针进行访问了。这样当 s1 离开作用域的时候是不会释放内存的，当 s2 离开作用域释放内存，就不会引起二次释放的问题了。

### 值的克隆

如果我们需要同时使用 s1 和 s2 呢，这就需要用到 clone 方法

```rust
  let s1 = String::from("hello");
  let s2 = s1.clone();
  println!("s1 is {}, s2 is {}", s1, s2) // s1 is hello, s2 is hello
```

### 所有权与函数

将值传递给函数和直接赋值给另个变量，本质上是相同的。

```rust
  fn main() {
    // str被创建
    let str = String::from("Hello");
    // str被移动到test_str内
    test_str(str);
    // 因为str发生了移动，所以这里不能访问str
    println!("{}", str);

    // num 是整型，存放在栈中，所以不存在移动
    let num = 5;
    test_num(num);
    // 在这里还可以访问
    println!("{}", num);
  } // num 离开作用域，出栈，str已经被移动到test_str，所以不会对str做处理

  fn test_str(str: String) {
    // str 被移动到函数上下文
    println!("str is {}", str)
  } // 在这里str离开了作用域，会被销毁

  fn test_num(num: i32) {
    // num 进入
    println!("num is {}", num)
  } // num 离开作用域，出栈

```

### 返回值与作用域

函数的返回值可以看做是一个赋值。

```rust

  fn main() {
    // create_str 将返回值移动给str1
    let str1 = create_str();
    // 创建str2
    let str2 = String::from("hello");
    // 将 str2 移动给 move_str，move_str将返回值移动给 str3
    let str3 = move_str(str2);
  } // 这里str1离开作用域被销毁，str2已经被移走，无事发生，str3离开作用域被销毁

  fn create_str() -> String {
    // str 进入作用域
    let str = String::from("hello");
    // 将str返回，也就是移动给调用这个方法的变量
    str
  } // str 已经被移动出去了，这里无事发生

  fn move_str(str: String) -> String {
    // str 进入作用域
    str
  } // str 已经被移动出去了，这里无事发生

```

所以将移动到函数内的变量再移动出来，就可以继续使用这个变量的，但是如果每次都这样做会很麻烦

## 引用与借用

引用就像一个指针，可以通过引用访问到本不属于他的变量，但同时不获得这个变量的所有权

```rust
  fn main() {
    let str = String::from("hello");
    let len = get_length(&str);
    // 这里依然可以访问到hello
    println!("{} len is {}", str, len);
  }

  fn get_length(str: &String) -> usize {
    str.len()
  }
```

但是引用默认是不可变的。

```rust
  fn main() {
    let mut str = String::from("hello");
    let len = get_length(&str);
    // 这里依然可以访问到hello
    println!("{} len is {}", str, len);
  }

  fn get_length(str: &String) -> usize {
    str.push_str("world"); // `str` is a `&` reference, so the data it refers to cannot be borrowed as mutable
    str.len()
  }
```

想要改变引用的值需要声明一个可变引用，通过 &mut 来声明一个可变引用。

```rust
  fn main() {
    let mut str = String::from("hello");
    change_str(&mut str);
    println!("str is {}", str);
  }

  fn change_str(str: &mut String) {
    str.push_str(" world");
  }
```

但是可变引用有一个限制：同一时间，针对一个变量只能存在一个可变引用。因为同时存在多个可变引用会在出现问题时难以定位。

```rust
  let mut str: String = String::from("hello");

  let str1 = &mut str;
  let str2 = &mut str; // cannot borrow `str` as mutable more than once at a time second mutable borrow occurs here

  println!("{} {}", str1, str2);
```

但是可以同时存在多个不可变引用，因为不可变引用不会对数据进行修改。但是不允许同时出现多个不可变引用可可变引用

```rust
  let mut s = String::from("hello");

  let r1 = &s; // 没问题
  let r2 = &s; // 没问题
  let r3 = &mut s; // cannot borrow `s` as mutable because it is also borrowed as immutable

  println!("{}, {}, and {}", r1, r2, r3);
```

注意，一个引用的作用域是从这个引用被创建到最后一个使用这个引用，所以如果是在这个其他引用最后一次使用后再创建其他可变引用是可以的。

```rust
  let mut s = String::from("hello");

  let r1 = &s; // 没问题
  let r2 = &s; // 没问题
  println!("{}, {}", r1, r2);

  let r3 = &mut s; // 没问题
  println!("{}", r3);
```

### 悬垂引用

在其他存在指针的语言中，可能会存在内存已经被释放，但是指向这个区域的指针还存在。这样就无法通过指针获取到正确的数据，这种指针也被称作为悬垂指针。而在 Rust 中，返回一个无效的引用就被称作悬垂引用。

```rust
fn no_dangle() -> &String {
  // this function's return type contains a borrowed value
  let str = String::from("hello");

  &str
}
```

总结一下引用的规则

- 同一时间只能存在一个可变引用，可以同时存在多个不可变引用
- 引用必须是有效的

## slice

### 字符串 slice

截取字符串中的某一部分

```rust
  let str = String::from("hello world");
  let str1 = &str[2..4];
  let str2 = &str[..4];
  let str3 = &str[2..];
  println!("{} {} {}", str1, str2, str3)
```

### 字符串字面值就是 slice

字符串字面量其实就是一个 slice

```rust
  let str: &str = "hello";
```

正因为字面量是一个不可变引用，所以他的值不能修改。

### 字符串 slice 作为参数

将 &String 类型的参数修改为&str 会更灵活

```rust
  let my_string = String::from("hello world");

  // `first_word` 适用于 `String`（的 slice），整体或全部
  let word = first_word(&my_string[0..6]);
  let word = first_word(&my_string[..]);
  // `first_word` 也适用于 `String` 的引用，
  // 这等价于整个 `String` 的 slice
  let word = first_word(&my_string);

  let my_string_literal = "hello world";
  let b = &my_string_literal[0..6];
  // `first_word` 适用于字符串字面值，整体或全部
  let word = first_word(&my_string_literal[0..6]);
  let word = first_word(&my_string_literal[..]);

  // 因为字符串字面值已经 **是** 字符串 slice 了，
  // 这也是适用的，无需 slice 语法！
  let word = first_word(my_string_literal);

  fn first_word(s: &str) -> &str {
    s
  }

```

所有权、借用和 slice 这些概念让 Rust 程序在编译时确保内存安全。Rust 语言提供了跟其他系统编程语言相同的方式来控制你使用的内存，但拥有数据所有者在离开作用域后自动清除其数据的功能意味着你无须额外编写和调试相关的控制代码。
