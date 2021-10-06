struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn add(&self) -> i32 {
        self.x + self.y
    }
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn main() {
    let p = Point::new(29, 30);
    let g = p.add();
    println!("g: {}", g);
}
