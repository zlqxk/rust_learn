# 错误处理

## panic! 与不可恢复的错误

panic 意思是恐慌，与 js 中的 error 概念相同。

rust 中的 panic 模式是展开的，他会回溯调用栈并且清理每一个函数。也可以指定 panic = 'abort'将切换改为终止

## Result 与可恢复的错误

大部分错误并没有严重到需要程序完全停止执行。有时，一个函数会因为一个容易理解并做出反应的原因失败。例如，如果因为打开一个并不存在的文件而失败，此时我们可能想要创建这个文件，而不是终止进程。

例如读取一个文件，这个文件可能是存在的也可能不存在，如果不存在我们对其做一些处理，例如创建这个文件。

```rust
  let content = File::open("hello.txt");

  let content = match content {
    Ok(file) => file,
    Err(error) => panic!("Problem opening the file: {:?}", error),
  };
```

现在是在打开失败的时候直接抛出异常，我们期望的是在文件不存在的时候创建文件，如果是没有权限的情况下再抛出恐慌

```rust
  let content = File::open("hello.txt");

  let content = match content {
    Ok(file) => file,
    Err(error) => match error.kind() {
      ErrorKind::NotFound => match File::create("hello.txt") {
        Ok(fc) => fc,
        Err(e) => panic!("Problem creating the file: {:?}", e),
      },
      other_error => {
        panic!("Problem opening the file: {:?}", other_error)
      }
    },
  };
```

### 失败时 panic 的简写：unwrap 和 expect

unwrap 的作用就是 Result 的结果是 Ok 的时候返回 Ok 的参数，如果是 Err 则调用 panic，所以上述可以简写为

```rust
  let content = File::open("hello.txt").unwrap();
```

expect 可以自定义错误信息

```rust
  let f = File::open("hello.txt").expect("Failed to open hello.txt");
```

### 传播错误
