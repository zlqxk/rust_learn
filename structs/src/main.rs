fn main() {
    // let mut user1 = User {
    //     email: String::from("someone@example.com"),
    //     username: String::from("someusername123"),
    //     active: true,
    //     sign_in_count: 1,
    // };

    // let user2 = User {
    //     email: String::from("another@example.com"),
    //     ..user1
    // };

    // println!("{}", user1.username);
    // println!("{}", user1.active);

    // let color = Color(1, 2, 3);
    // let bgColor = BgColor(1, 2, 3);

    // println!("this is {:?}", user1)

    let mut rectangle = Rectangle {
        width: 10,
        heigth: 20,
    };

    println!("area is {}", rectangle.area());

    rectangle.change();

    println!("width is {}", rectangle.width);

    let rectangle2 = Rectangle::pp(10);

    println!("area is {}", rectangle2.area());
}
// #[derive(Debug)]
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// struct Color(i32, i32, i32);
// struct BgColor(i32, i32, i32);

struct Rectangle {
    width: u32,
    heigth: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.heigth;
    }

    fn change(&mut self) {
        self.width = 100
    }

    fn pp(num: u32) -> Rectangle {
        return Rectangle {
            width: num,
            heigth: num,
        };
    }
}
