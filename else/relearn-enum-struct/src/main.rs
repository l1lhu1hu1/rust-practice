// 複数の定数をIpAddKindという一つの型で一括管理
enum IpAddrKind {
    V4,
    V6,
}

// IpAddrをこのように定義すれば、kindに
// 種類、addressにアドレスを入れることでIP addressを表現可能
// ただこれだとstructとIpAddrKindの両方が必要になる
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let ip = IpAddr {
        kind: IpAddrKind::V4,
        address: "2000".to_string(),
    };
    println!("Hello, world!, {}", ip.address);
}
