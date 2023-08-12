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
    fn new(side: u32) -> Square {
        Square(side) 
    }
}

fn main() {
    let my_square = Square::new(5);

    println!("Area of my_square is {}", my_square.area());
}
