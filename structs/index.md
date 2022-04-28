# 使用结构体组织相关联的数据

## 定义并实例化结构体

结构体可以理解为面向对象语言中的 class

```rust
  fn main() {
    let user1 = User {
      email: String::from("someone@example.com"),
      username: String::from("someusername123"),
      active: true,
      sign_in_count: 1,
    };
  }

  struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
  }
```

struct 可以被声明为可变的，这样我们就可以修改他的属性值

```rust
  let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
  };

  user1.active = false;
```

### 使用结构体更新语法从其他实例创建实例

可以理解为 js 的解构

```rust
  let user2 = User {
    active: false,
    ..user1
  };

```

解构复制本质和 = 是一样的，也是涉及到变量的移动，所以在 user2 声明以后，user1 将不能使用。

```rust
  // 此时打印user.username报错，borrow of moved value: `user1.username`
  println!("{}", user1.username);
  // 但是active不会报错，因为这个是bool类型，内部实现了copy的trait
  println!("{}", user1.active);
```

### 使用没有命名字段的元组结构体来创建不同的类型

元组结构体和元组基本一样，区别就是通过不同的元组结构体生成的实例，就算元组内部的元素类型都是一样的，但是实例化的两个元组结构体却是不同类型

```rust
struct Color(i32, i32, i32);
struct BgColor(i32, i32, i32);

let color = Color(1, 2, 3); // Color类型
let bgColor = BgColor(1, 2, 3); // bgColor类型
```

### 通过派生 trait 增加实用功能

如果我们直接 println 在终端输出 struct 的话，会收到一个错误

```rust

  struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
  }

 let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
  };

  println!("this is {}", user1)
  // `User` doesn't implement `std::fmt::Display`
  // the trait `std::fmt::Display` is not implemented for `User`
  // in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```

按照提示修改

```rust

  struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
  }

 let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
  };

  println!("this is {:?}", user1)
  // `User` doesn't implement `Debug`
  // the trait `Debug` is not implemented for `User`
  // add `#[derive(Debug)]` to `User` or manually `impl Debug for User
```

继续修改

```rust

  #[derive(Debug)]
  struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
  }

 let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
  };

  println!("this is {:?}", user1)
  // `User` doesn't implement `Debug`
  // the trait `Debug` is not implemented for `User`
  // add `#[derive(Debug)]` to `User` or manually `impl Debug for User
```

这次终于可以在终端看到输出了。

## 方法语法

方法与函数类型，不过方法是在 struct 内部的，并且他的第一个参数永远是 self，也就是结构体本身

```rust
  struct Rectangle {
    width: u32,
    heigth: u32,
  }

  impl Rectangle {
    fn area(&self) -> u32 {
      return self.width * self.heigth;
    }
  }

  main() {
    let rectangle = Rectangle {
      width: 10,
      heigth: 20,
    };

    println!("area is {}", rectangle.area()) // 200
  }
```

为什么 self 要是引用：我们并不想获取所有权，只希望能够读取结构体中的数据，而不是写入。如果需要在方法汇总修改结构体的数据，可以将&self 修改为&mut self

```rust
impl Rectangle {
  fn area(&self) -> u32 {
    return self.width * self.heigth;
  }

  fn change(&mut self) {
    self.width = 100
  }
}

rectangle.change();

println!("width is {}", rectangle.width)

```

结构体方法也可以声明多个参数。

```rust
impl Rectangle {
  fn mul(&mut self, a: u32, b: String) {
    self.width = 100
  }
}
```

### 关联函数

不以 self 为第一个参数的方法被称为关联函数，关联方法通过::调用，通常用来实例化一个 struct，例如我们常用的 String::from()。

```rust
impl Rectangle {
  fn pp(num: u32) -> Rectangle {
    return Rectangle {
      width: num,
      heigth: num,
    };
  }
}

let rectangle2 = Rectangle::pp(10);

println!("area is {}", rectangle2.area());
```
