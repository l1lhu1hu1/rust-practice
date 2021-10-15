use std::fs::File;
use std::io;
use std::io::Read;
use std::fs;

fn read_username_from_file(file_name: &str) -> Result<String, io::Error> {
    let f = File::open(file_name);
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
fn read_username_from_file_q_op(file_name: &str) -> Result<String, io::Error> {
    // errorになった時にすぐにreturn
    // イメージ的にはガード節
    let mut f = File::open(file_name)?;
    let mut s= String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
fn read_username_from_file_q_op_shorter(file_name: &str) -> Result<String, io::Error> {
    let mut s= String::new();
    File::open(file_name)?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_shortest(file_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(file_name)
}

fn print_result(r: Result<String, io::Error>) {
    match r {
        Ok(res) => println!("res: {}", res),
        Err(e) => println!("e: {}", e),
    };
}

fn main() {
    let f1 = "hello.txt".to_string();
    let f2 = "bye.txt".to_string();
    let a = read_username_from_file(&f1);
    println!("a");
    print_result(a);
    let b = read_username_from_file_q_op(&f2);
    println!("b");
    print_result(b);
    let c = read_username_from_file_q_op_shorter(&f1);
    println!("c");
    print_result(c);
    let d = read_username_from_file_q_op_shorter(&f1);
    println!("d");
    print_result(d);
    let e = read_username_from_file_shortest(&f1);
    println!("e");
    print_result(e);

}
