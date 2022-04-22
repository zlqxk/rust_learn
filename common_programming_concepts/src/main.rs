fn main() {
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // let mut x = 1;
    // x = x + 1;

    // {
    //     x = x * 2;
    //     println!("this value of x is {}", x)
    // }

    // println!("this value of x is {}", x)

    // let str = 'a';

    // let tup: (u8, i32, f32, char) = (8, 100, 10.2, 'a');
    // // 取值方式1 - 解构
    // let (x, y, z, k) = tup;
    // // 取值方式2 - 根据下标
    // let n = tup.0;

    // let arr = [1, 2, 3, 4];
    // // 解构取值
    // let [a, b, c, d] = arr;
    // // 下标取值
    // let f = arr[0];

    // test('a', 1);

    let a = add(1);
    println!("a is {}", a)
}

fn test(a: char, b: i32) {
    println!("a is {}, b is {}", a, b)
}

fn add(a: i32) -> i32 {
    return a + 1;
}
