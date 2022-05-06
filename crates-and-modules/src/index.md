# 使用包、Crate 和模块管理不断增长的项目

模块化可以更好的组织代码和功能，在 rust 中有这模块系统，模块系统包括

- 包
- crate
- 模块
- 路径

## 包和 crate

我们通过 cargo new project 生成一个 project，这个 project 就是一个包，src/main.rs 就是一个 二进制 crate，并且根据约定 main.rs 是与包同名的二进制 crate，src/lib.rs 就是一个与包同名的库 crate，一个包中可以有多个二进制 crate，但是最多只能有一个库 crate，通过将文件放在 src/bin 目录下，可以声明多个二进制 crate。

## 定义模块来控制作用域与私有性

模块可以将一个 crate 中的代码进行分组，并且控制项的私有性。

```rust
mod front_of_house {
  mod hosting {
    fn add_to_waitlist() {}

    fn seat_at_table() {}
  }

  mod serving {
    fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}
  }
}

```

通过 mod 关键字来定义一个模块，在模块内可以定义其他模块，也可以定义函数、枚举等。刚刚声明的模块会形成一个模块树，如下。

```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

### 路径用于引用模块树中的项

路径引用有两种形式

- 绝对路径（absolute path）从 crate 根开始，以 crate 名或者字面值 crate 开头。
- 相对路径（relative path）从当前模块开始，以 self、super 或当前模块的标识符开头。

```rust
mod front_of_house {
  mod hosting {
    fn add_to_waitlist() {}
  }
}

pub fn eat_at_restaurant() {
  // 绝对路径
  crate::front_of_house::hosting::add_to_waitlist();

  // 相对路径
  front_of_house::hosting::add_to_waitlist();
}
```

但是模块中默认所有项都是私有的，所以无法调用 hosting 模块和 add_to_waitlist，需要将其指定为共有模块。

```rust
mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist() {}
  }
}

pub fn eat_at_restaurant() {
  // 绝对路径
  crate::front_of_house::hosting::add_to_waitlist();

  // 相对路径
  front_of_house::hosting::add_to_waitlist();
}
```

### 使用 super 起始的相对路径

super 类似于 .. ，作用是跳出当前作用域，到其父级模块下寻找要调用的模块或者方法。

```rust
fn serve_order() {}

mod back_of_house {
  fn fix_incorrect_order() {
    cook_order();
    super::serve_order();
  }

  fn cook_order() {}
}

```

### 创建公有的结构体和枚举

我们可以通 pub 关键字将结构体设置为共有的，但是结构体变成共有的，但是里面的字段还是私有的。

```rust
mod back_of_house {
  pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
  }

  impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
      Breakfast {
        toast: String::from(toast),
        seasonal_fruit: String::from("peaches"),
      }
    }
  }
}

pub fn eat_at_restaurant() {
  // 在夏天订购一个黑麦土司作为早餐
  let mut meal = back_of_house::Breakfast::summer("Rye");
  // 改变注意更换想要面包的类型
  meal.toast = String::from("Wheat");
  println!("I'd like {} toast please", meal.toast);

  // 如果取消下一行的注释代码不能编译；
  // 不允许查看或修改早餐附带的季节水果
  // meal.seasonal_fruit = String::from("blueberries");
}

```

与之相反，如果我们将枚举设为公有，则它的所有成员都将变为公有。

```rust
  mod back_of_house {
    pub enum Appetizer {
      Soup,
      Salad,
    }
  }

  pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
  }
```

## 使用 use 关键字将名称引入作用域

我们可以通过 use 关键字，将路径一次性引入到作用域中

```rust
  mod front_of_house {
    pub mod hosting {
      pub fn add_to_waitlist() {}
    }
  }

  use crate::front_of_house::hosting;

  pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
  }

```

在作用域中使用 use 就像在文件系统中添加软链一样。
同样的，也可以使用 use 来引入相对路径

```rust
  mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
  }

  use self::front_of_house::hosting;

  pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
  }
```

### 使用 as 关键字提供新的名称

```rust
  use std::fmt::Result;
  use std::io::Result as IoResult;

  fn function1() -> Result {
    // --snip--
  }

  fn function2() -> IoResult<()> {
    // --snip--
  }
```

### 使用 pub use 重导出名称

使用 use 关键字将某个模块引入到当前作用域下，这样在这个作用域下就可以使用这个模块了，但是这个模块在此作用域之外还是私有的，如果我们希望别人在使用我们的模块时也能同时使用导入的模块，需要使用 pub + use

```rust
  mod front_of_house {
    pub mod hosting {
      pub fn add_to_waitlist() {}
    }
  }

  pub mod test {
    pub use crate::front_of_house::hosting;
  }

  pub fn eat_at_restaurant() {
    test::hosting::add_to_waitlist();
    test::hosting::add_to_waitlist();
    test::hosting::add_to_waitlist();
  }

```

### 嵌套路径来消除大量的 use 行

```rust
use std::cmp::Ordering;
use std::io;

use std::{cmp::Ordering, io};
```

### 通过 glob 运算符将所有的公有定义引入作用域

```rust
use std::collections::*;
```

## 将模块分割进不同文件