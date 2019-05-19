use std::io;
use std::io::Write;

fn main() {
    print!("任意の値を入力してください: ");
    io::stdout().flush().unwrap(); 
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("入力エラー。read_line()で失敗しました。");
}
