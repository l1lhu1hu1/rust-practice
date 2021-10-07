enum LongLongLongLongLongLongLongLongEnum {
    Add,
    Substract,
}

// alias
type le = LongLongLongLongLongLongLongLongEnum;

// impl LongLongLongLongLongLongLongLongEnum {
impl le {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Substract => x - y,
        }
    }
}

fn main() {
    let a = le::Add;
    let s = le::Substract;
    let r1 = a.run(20, 30);
    let r2 = s.run(20, 30);
    println!("r1: {}, r2: {}", r1, r2);
}
