struct Point {
    x: i32,
    y: i32,
}

impl Drop for Point {
    fn drop(&mut self) {
        println!("Dropping Point");
    }
}

struct HasDrop;

impl Drop for HasDrop {
    fn drop(&mut self) {
        println!("dropping hasdrop");
    }
}

struct HasTwo {
    one: HasDrop,
    two: HasDrop,
}

impl Drop for HasTwo {
    fn drop(&mut self) {
        println!("dropping hastwo");
    }
}

fn main() {
    {
        let p = Point { x: 20, y: 39 };
    }
    println!("Hello");
    {
        let h = HasTwo {
            one: HasDrop {},
            two: HasDrop {},
        };
    }
}
