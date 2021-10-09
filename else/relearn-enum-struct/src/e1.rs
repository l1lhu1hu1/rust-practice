enum IpAddr {
    V4(String),
    V6(String),
}

// 以下のように複数のデータを持つこともできる
// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

fn matcher(ip: &IpAddr) -> String {
    let g = match ip {
        IpAddr::V4(value) => ("ipv4".to_string() + value),
        IpAddr::V6(value) => ("ipv6".to_string() + value),
    };
    g.to_string()
}

fn main() {
    let ip = IpAddr::V4("aaaaaaaaaaaaaaaaaaaaaaa".to_string());
    let ip2 = IpAddr::V6("bbbbbbbbbbbbbbbbbbbbbb".to_string());
    // V4とV6の両方を持つことはできない
    // 以下はコンパイルできない
    // let ip3 = IpAddr("three", "333333");

    let r = matcher(&ip);
    println!("r is {}", r);

    let r2 = matcher(&ip2);
    println!("r2 is {}", r2);

    // match ip {
    //     IpAddr::V4(value) => println!("value: {}", value),
    //     IpAddr::V6(value) => println!("something else"),
    // }
}
