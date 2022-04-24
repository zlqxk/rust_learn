fn main() {
    // let str = "111";

    // let mut str = String::from("hello");

    // str.push_str(" world");

    // println!("str is {}", str);

    // {
    //     let s = String::from("hello"); // 从此处起，s 是有效的

    //     // 使用 s
    // } // 此作用域已结束，
    //   // s 不再有效

    // let x = 5;
    // let y = x;

    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("s1 is {}", s1)

    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    // println!("s1 is {}, s2 is {}", s1, s2)

    // let str = String::from("Hello");
    // test_str(str);
    // println!("{}", str);

    // let num = 5;
    // test_num(num);
    // println!("{}", num);

    //     let str1 = create_str();
    //     let str2 = String::from("hello");
    //     let str3 = move_str(str2);

    // let str = String::from("hello");
    // let len = get_length(&str);
    // // 这里依然可以访问到hello
    // println!("{} len is {}", str, len);

    // let mut str = String::from("hello");
    // change_str(&mut str);
    // println!("str is {}", str);

    // let mut str: String = String::from("hello");

    // let str1 = &mut str;
    // let str2 = &mut str;

    // println!("{} {}", str1, str2);

    // let mut s = String::from("hello");

    // let r1 = &s; // 没问题
    // let r2 = &s; // 没问题
    // let r3 = &mut s; // 大问题

    // println!("{}, {}, and {}", r1, r2, r3);

    // let mut s = String::from("hello");

    // let r1 = &s; // 没问题
    // let r2 = &s; // 没问题
    // println!("{}, {}", r1, r2);

    // let r3 = &mut s; // 没问题
    // println!("{}", r3);

    // let str = String::from("hello world");
    // let str1 = &str[2..4];
    // let str2 = &str[..4];
    // let str3 = &str[2..];
    // println!("{}{}{}", str1, str2, str3)

    // let str: &str = "hello";

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
}

// fn test_str(str: String) {
//     println!("str is {}", str)
// }

// fn test_num(num: i32) {
//     println!("num is {}", num)
// }

// fn create_str() -> String {
//     let str = String::from("hello");
//     str
// }

// fn move_str(str: String) -> String {
//     str
// }

// fn get_length(str: &String) -> usize {
//     str.len()
// }

// fn change_str(str: &mut String) {
//     str.push_str(" world");
// }

// fn no_dangle() -> &String {
//     // this function's return type contains a borrowed value
//     let str = String::from("hello");

//     &str
// }

fn first_word(s: &str) -> &str {
    s
}
