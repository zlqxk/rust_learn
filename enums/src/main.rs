fn main() {
    // let four = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let six = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    // let four = IpAddrKind::V4(String::from("127.0.0.1"));
    // let six = IpAddrKind::V6(String::from("::1"));

    // four.call();

    // let a = Some("aaa");
    // let num = Some(12);
    // let none: Option<i32> = None;

    let num = Some(1);

    let num2 = plus_one(num);
    plus_one(None);
    println!("{}", num2);
    println!("{:?}", num);
}

// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// enum IpAddrKind {
//     V4(String),
//     V6(String),
// }

// impl IpAddrKind {
//     fn call(&self) {
//         //
//     }
// }

// #[derive(Debug)] // 这样可以立刻看到州的名称
// enum UsState {
//     Alabama,
//     Alaska,
//     // --snip--
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("hahah");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {:?}!", state);
//             25
//         }
//     }
// }

fn plus_one(num: Option<i32>) -> i32 {
    match num {
        None => 0,
        Some(t) => t + 1,
    }
}
