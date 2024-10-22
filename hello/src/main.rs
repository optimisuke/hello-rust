use std::env;

fn main() {
    // コマンドライン引数を取得
    let args: Vec<String> = env::args().collect();

    // 引数が1つ（プログラム名のみ）の場合、標準的な挨拶
    if args.len() < 2 {
        println!("Hello, world!");
    } else {
        // 引数が複数ある場合、それぞれの名前に対して挨拶
        for name in &args[1..] {
            println!("Hello, {}!", name);
        }
    }
}
