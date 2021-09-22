fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    // errorになる(stringなので)
    // println!("{}", s);
    let x = 5;
    makes_copy(x);
    // errorにならない(i32なので)
    // println!("{}", x);
    {
        let s1 = gives_ownership();
        println!("{}", s1);
        let s2 = String::from("hello");
        let s3 = takes_and_gives_back(s2);
        // s2はまずtakes_and_gives_backにmoveされて、その後にs3にmoveされているのでerror
        // println!("{}", s2);
        println!("{}", s3);
    }

    {
        let s1 = String::from("hello");
        let (s2, len) = calculate_length(s1);
        println!("var: {}, length: {}", s2, len);
    }
    {
        let s1 = String::from("hello");
        let len = calculate_length_borrow(&s1);
        println!("var: {}, length: {}", s1, len);
    }
    // TODO 以下をやる
    {
        let mut s = String::from("hello");

        let r1 = &s; // no problem
        let r2 = &s; // no problem
                     // let r3 = &mut s; // BIG PROBLEM

        // println!("{}, {}, and {}", r1, r2, r3);
    }
    {
        let mut s = String::from("hello");

        let r1 = &s; // no problem
        let r2 = &s; // no problem
        println!("{} and {}", r1, r2);
        // variables r1 and r2 will not be used after this point

        let r3 = &mut s; // no problem
        println!("{}", r3);
    }
}

// &は参照を表す
fn calculate_length_borrow(s: &String) -> usize {
    s.len()
}

// 一々ownershipを渡して値を参照してまたownershipを返すの結構めんどくさい
// そこでreferenceが登場する
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn gives_ownership() -> String {
    let some_string = String::from("sayonara");
    some_string
}

fn takes_and_gives_back(a: String) -> String {
    a
}

fn takes_ownership(s: String) {
    println!("{}", s);
}

fn makes_copy(i: i32) {
    println!("{}", i);
}
