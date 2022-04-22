# 常见的编程概念

## 变量和可变性

rust 的变量默认是不可变的

```rust
  let x = 5;
  println!("The value of x is: {}", x);
  x = 6; // cannot assign twice to immutable variable `x`
  println!("The value of x is: {}", x);
```

使用 mut 关键词使其可变

```rust
  let mut x = 5;
  println!("The value of x is: {}", x);
  x = 6;
  println!("The value of x is: {}", x);
```

## 常量

常量使用 const 声明，区别于变量

- 常量不止有默认的时候不可修改，而是永远不能修改
- 常量可以在任何作用域下声明
- 常量声明必须标注类型
- 常量必须是常量表达式可以计算的值，而不是需要在运行时才能得到结果的值

```rust
  const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

## 隐藏

rust 允许重复声明同一个变量，可以用同一个变量名称来隐藏这个变量

```rust
  let x = 1;
  let x = x + 1;

  {
      let x = x * 2;
      println!("this value of x is {}", x)
  }

  println!("this value of x is {}", x)

  // this value of x is 4
  // this value of x is 2
```

隐藏和可变的区别

- 隐藏可以改变这个变量的类型
- 隐藏有作用域

```rust
  let mut x = 1;
  x = x + 1;

  {
      x = x * 2;
      println!("this value of x is {}", x)
  }

  println!("this value of x is {}", x)

  // this value of x is 4
  // this value of x is 4
```

## 数据类型

rust 是静态类型语言，所以需要在编译时知道所有的变量类型，rust 的数据类型分为标量和复合。

### 标量

#### 整型

整型代表没有小数点的数字，i 开头代表有符号整型，u 开头代表无符号整型，例如 i8 代表 -(2^7) 到 2^7 - 1，而 u8 代表 0 到 2^8 - 1。isize 和 usize 和系统架构有关。如果变量赋值溢出，debug 阶段会引起恐慌，release 阶段会忽略，对溢出部分进行二进制补码包装。

```rust
// 长度	    有符号	  无符号
// 8-bit	   i8	      u8
// 16-bit	 i16	    u16
// 32-bit	 i32	    u32
// 64-bit	 i64	    u64
// 128-bit	 i128	    u128
// arch	   isize	   usize
```

#### 浮点型

rust 中有两个浮点型，f32 和 f64。在现在的 cpu 架构中通常使用 f64，因为和 f32 相比性能相差不大，但是精度更高。

#### 布尔型

rust 中的布尔型用 bool 表示

#### 字符类型

rust 中的字符类型用 char 表示，值用单引号包裹

```rust
  let str = 'a';
```

### 复合

rust 中的符合类型有两种，元组和数组。

#### 元组

元组是将多个不同类型的值复合到一起的类型，元组的长度一旦确定就不能改变。

```rust
  let tup: (u8, i32, f32, char) = (8, 100, 10.2, 'a');
  // 取值方式1 - 解构
  let (x, y, z, k) = tup;
  // 取值方式2 - 根据下标
  let n = tup.0;
```

#### 数组

数组和元组的区别是，数组中的每一个值类型必须相同。数组的长度也是固定的。

## 函数

rust 中的函数参数必须指定类型

```rust
  fn test(a: char, b: i32) {
    println!("a is {}, b is {}", a, b)
  }

  // 有返回值的函数，不加分号就是返回
  fn add(a: i32) -> i32 {
    a + 1
  }

  // 使用return关键字
  fn add(a: i32) -> i32 {
    return a + 1;
  }
```

## 控制流

### if - else

if 后面的必须是布尔类型

```rust

let num = 2;

if num > 2 {
  ...
}

// 报错
if num {

}

```

### 循环

- loop

- while

- for
