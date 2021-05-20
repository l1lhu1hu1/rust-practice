use std::sync::{Arc, Mutex};
use std::thread;
fn main() {
    let mut handles = Vec::new();
    // Arcだけだと排他制御はできないのMutexを用いている
    // Arc stands for Atomically Reference Counted
    // Mutexはlockメソッドをもつ
    // Arcはマルチスレッド版のrcで、所有者カウンタが0になったらメモリを解放するという仕組みになっている
    let data = Arc::new(Mutex::new(vec![1; 10]));

    for x in 0..10 {
        // cloneは所有者カウンタをインクリメントするメソッド
        let data_ref = data.clone();
        handles.push(thread::spawn(move || {
            // 一度ロックして同時に外部から書き換えられないようにしている
            let mut data = data_ref.lock().unwrap();
            data[x] += 1;
        }));
    }

    for handle in handles {
        let _ = handle.join();
    }
    dbg!(data);
}
