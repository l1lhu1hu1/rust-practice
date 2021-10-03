use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

// Tが何であろうとnewは実装されている
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// PartialOrdとDisplayを実装しているような型のみ許している
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let p = Pair::new(20, 30);
    p.cmp_display();
    let p1 = Pair::new(50.0, 40.0);
    p1.cmp_display();
}
