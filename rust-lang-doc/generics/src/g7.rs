struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn sum(&self) -> i32 {
        self.x + self.y
    }
    fn get_area(&self, other: &Self) -> i32 {
        let width = (self.x - other.x).abs();
        let height = (self.y - other.y).abs();
        width * height
    }
}

fn main() {
    let p1 = Point { x: 32, y: 100 };
    let p2 = Point { x: 22, y: 55 };
    let r1 = p1.sum();
    let r2 = p1.get_area(&p2);
    println!("r1: {}", r1);
    println!("r2: {}", r2);
}
