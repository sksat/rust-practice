use std::io;

fn main() { // 関数宣言．引数の型を書いていないので空のタプルとして扱われる
    println!("Guess the number!"); // println!は文字列表示マクロ

    println!("Please input your guess.");

    let mut guess = String::new(); // 可変(mutable)な変数guessを作りString(UTF-8な文字列)にする

    io::stdin().read_line(&mut guess) // use std::ioしていないとstd::io::stdinと書くことになる．参照もデフォルトでimutableなので&mutとする．
        .expect("Failed to read line"); // read_lineが変えるio::Resultのexpectメソッド．

    println!("You guessed: {}", guess); // {}はプレースホルダ
}
