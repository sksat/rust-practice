extern crate rand; // randを使う

use std::io;
use std::cmp::Ordering;
use rand::Rng; // gen_rangeメソッドが動作するためにトレイトがスコープにある必要がある

fn main() { // 関数宣言．引数の型を書いていないので空のタプルとして扱われる
    println!("Guess the number!"); // println!は文字列表示マクロ

    let secret_number = rand::thread_rng().gen_range(1,101); // 現在いるスレッドにローカルな乱数生成器のコピーを取得，範囲を指定して乱数を生成

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // 可変(mutable)な変数guessを作りString(UTF-8な文字列)にする

        io::stdin().read_line(&mut guess) // use std::ioしていないとstd::io::stdinと書くことになる．参照もデフォルトでimutableなので&mutとする．
            .expect("Failed to read line"); // read_lineが変えるio::Resultのexpectメソッド．

        let guess: u32 = match guess.trim().parse(){ // 新しくu32なguessを定義(シャドーイングを使い名前を再利用)
            Ok(num) => num, // 成功した場合はOkに内包された値であるnumをそのまま返す
            Err(_) => continue, // 失敗した場合はcontinueする
        };

        println!("You guessed: {}", guess); // {}はプレースホルダ

        match guess.cmp(&secret_number){ // cmpで比較．これはOrderingを返す
            Ordering::Less      => println!("Too small!"),  // match文を使ってOrderingのどれであるかを判断
            Ordering::Greater   => println!("Too big!"),    // Orderingはenum
            Ordering::Equal     => {
                println!("You win!");
                break;
            }
        }
    }
}
