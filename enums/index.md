# 枚举与模式匹配

## 定义枚举

rust 中的枚举和其他语言中的枚举所代表含义相似，都是用来指几个可以枚举出所有类型或者值的类型，例如 ip 地址的类型只有 ipv4 和 ipv6 两种。

```rust
  enum IpAddrKind {
    V4,
    V6,
  }

  let four = IpAddrKind::V4;
  let six = IpAddrKind::V6;
```

使用 enum 关键字声明枚举，使用::来创建枚举。

现在我们的枚举只有类型，但是没有值，通常我们需要声明一个值和他对应的类型。用我们之前学过的知识。可以使用 struct 来组合。

```rust
fn main() {
  let four = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
  };

  let six = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
  };
}

enum IpAddrKind {
  V4,
  V6,
}

struct IpAddr {
  kind: IpAddrKind,
  address: String,
}

```

这样就可以将值和类型聚合到一起了，但是使用起来过于麻烦，其实枚举已经提供了声明值和类型的能力。

```rust
  enum IpAddrKind {
    V4(String),
    V6(String),
  }

  let four = IpAddrKind::V4(String::from("127.0.0.1"));
  let six = IpAddrKind::V6(String::from("::1"));
```

枚举也可以使用 impl 来定义方法

```rust
  enum IpAddrKind {
    V4(String),
    V6(String),
  }

  impl IpAddrKind {
    fn call(&self) {
        //
    }

  let four = IpAddrKind::V4(String::from("127.0.0.1"));
  four.call();

```

## Option 枚举和其相对于空值的优势

rust 中没有 null ，但是是有 null 的概念的，这 rust 中想要表达一个值是空值，就要用到 Option 了，这时一个定义在标准库中的枚举。他在 rust 中的表示如下。

```rust
  enum Option<T> {
    None,
    Some(T),
  }
```

我们可以直接使用 Some 和 None 来声明，因为 Option 枚举已经被预导入。

```rust
  let a = Some("aaa");
  let num = Some(12);
  // 因为只有一个None，rust无法推断类型，所以声明为None的需要主动指定类型
  let none: Option<i32> = None;
```

通过 Option，再加上一些流程控制运算符，就可以简单的处理有值和无值的情况了。

## match 控制流运算符

match 运算符有点类似其他语言中的 switch，通过多个分支来处理多种情况。

```rust
  enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
  }

  fn value_in_cents(coin: Coin) -> u8 {
    match coin {
      Coin::Penny => {
        println!("hahah");
        1
      },
      Coin::Nickel => 5,
      Coin::Dime => 10,
      Coin::Quarter => 25,
    }
  }

```

## 绑定值的模式

```rust
  #[derive(Debug)] // 这样可以立刻看到州的名称
  enum UsState {
    Alabama,
    Alaska,
    // --snip--
  }

  enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
  }

  fn value_in_cents(coin: Coin) -> u8 {
    match coin {
      Coin::Penny => {
        println!("hahah");
        1
      }
      Coin::Nickel => 5,
      Coin::Dime => 10,
      Coin::Quarter(state) => {
        println!("State quarter from {:?}!", state);
        25
      }
    }
  }

```

## 匹配 Option<T>

我们可以使用 match 来获取 Option 枚举中的值

```rust
fn plus_one(num: Option<i32>) -> i32 {
  match num {
    None => 0,
    Some(T) => T + 1,
  }
}
```

## 通配模式和 \_ 占位符

类似于 switch 中如果不能枚举所有的情况，必须添加 default。
