pub fn add_two(x: i32) -> i32 {
    if x > 100 {
        panic!("panicked val over 100")
    } else if x < 0 {
        panic!("panicked val under 0")
    }
    x + 2
}

fn greeting() -> String {
    String::from("Hello")
}

fn greeting_with_name(name: &str) -> String {
    format!("hello: {}", name)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exploration() {
        assert_eq!(22, add_two(20));
    }

    #[test]
    fn another() {
        // panic!("Make this test fail");
        assert_ne!(23, add_two(20));
    }

    #[test]
    fn test_greeting_with_name() {
        let r = greeting_with_name("lilhuihui");
        assert!(r.contains("lilhuihui"));
    }

    #[ignore]
    #[test]
    fn test_greeting() {
        let r = greeting();
        // 2つ目の引数としてテストが失敗した時用のメッセージを指定している
        assert!(r.contains("lilhuihui"), ("did not contain name. val was: `{}`", r));
    }

    #[test]
    #[should_panic(expected = "panicked val over 100")]
    fn greater_than_100() {
        add_two(200);
        // add_two(-200);
    }
}
