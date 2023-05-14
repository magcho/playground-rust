// fn fizz_buzz(value: i32) -> String {
//     let result = if value % 15 == 0 {
//         "fizz buzz".to_string()
//     } else if value % 5 == 0 {
//         "buzz".to_string()
//     } else if value % 3 == 0 {
//         "fizz".to_string()
//     } else {
//         value.to_string()
//     };
//     result
// }

use std::fmt::{self, Display};
use std::ops::Add;

// fn fizz_buzz_pattern_match(value: i32) -> String {
//     let result = match value {
//         v if v % 15 == 0 => "fizz buzz".to_string(),
//         v if v % 5 == 0 => "buzz".to_string(),
//         v if v % 3 == 0 => "fizz".to_string(),
//         _ => value.to_string(),
//     };
//     result
// }
enum Color {
    Red,
    Blue,
    Green,
    Hex(String),
}
fn string_to_color(value: &str) -> Option<Color> {
    match value {
        "red" => Some(Color::Red),
        "blue" => Some(Color::Blue),
        "green" => Some(Color::Green),
        _ => None,
    }
}

fn color_to_string(color: Color) -> String {
    match color {
        Color::Red => "red".to_string(),
        Color::Blue => "blue".to_string(),
        Color::Green => "green".to_string(),
        Color::Hex(hex) => hex,
        _ => panic!(),
    }
}

fn if_let_color(value: &str) {
    if let Some(aaaa) = string_to_color(value) {
        println!("{} is a color{:?}", "value", color_to_string(aaaa));
    }
}

fn add_until_loop(start: i32, end: i32) -> i32 {
    let mut sum = 0;
    let mut temp = start;
    loop {
        sum += temp;
        if temp == end {
            break sum;
        }
        temp += 1;
    }
}

fn add_until_while(start: i32, end: i32) -> i32 {
    let mut sum = 0;
    let mut temp = start;

    while temp <= end {
        sum += temp;
        temp += 1;
    }
    sum
}

fn not_copy() {
    let a = String::from("hello");

    println!("{}", a);
}

fn print_string(str: String) {
    println!("{}", str);
}

// fn lifetime_ng() -> &str {
//     "hello, world"
// }

fn lifetime_ok() -> &'static str {
    "hello, world"
}

fn feap_memory() {
    let s = Box::new("hello, world");

    println!("{}", s);
}

struct User {
    name: String,
    age: u32,
}

impl Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "user name is {}, age is {}", &self.name, &self.age)
    }
}

impl User {
    fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }

    fn description(&self) -> String {
        format!("user name is {}, age is {}", self.name, self.age)
    }

    fn rename(&mut self, name: String) {
        self.name = name;
    }
}

// ユーザー定義trait
trait Area {
    fn area(&self) -> u32;
}
struct Square(u32);

impl Area for Square {
    fn area(&self) -> u32 {
        self.0.pow(2)
    }
}

impl Square {
    fn new(side: u32) -> Self {
        Self(side)
    }
}

//

// 分数
struct Fraction(u32, u32);
impl Fraction {
    fn new(numerator: u32, denominator: u32) -> Self {
        let gcr_value = Self::gcf(numerator, denominator);
        Self(numerator / gcr_value, denominator / gcr_value)
    }

    fn gcf(value1: u32, value2: u32) -> u32 {
        let (mut a, mut b) = if value1 > value2 {
            (value1, value2)
        } else {
            (value2, value1)
        };

        let mut r = a % b;
        while r != 0 {
            a = b;
            b = r;
            r = a % b
        }

        b
    }
}
impl Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", &self.0, &self.1)
    }
}
impl Add for Fraction {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let lcm = self.1 / Self::gcf(self.1, other.1) * other.1;

        let a = self.0 * (lcm / self.1);
        let b = other.0 * (lcm / other.1);
        Fraction::new(a + b, lcm)
    }
}

fn main() {
    // println!("{}", fizz_buzz_pattern_match(1));
    // println!("{}", fizz_buzz_pattern_match(3));
    // println!("{}", fizz_buzz_pattern_match(15));

    // if_let_color("red");

    // println!("{}", add_until_loop(1, 10));
    // println!("{}", add_until_while(1, 10));

    // not_copy();

    // let hoge = String::from("hoge");
    // print_string(hoge.clone());
    // println!("{}", hoge);

    // feap_memory();

    // let mut user = User::new("magcho".to_string(), 23);
    // println!("{}", user.description());
    // println!("{}", user);

    // user.rename("hogemaru".to_string());
    // println!("{}", user.description())

    // let sq = Square::new(10);
    // println!("{}", sq.area());

    let frac1 = Fraction::new(8, 18);
    let frac2 = Fraction::new(1, 2);
    println!("{}", frac1 + frac2);
}
