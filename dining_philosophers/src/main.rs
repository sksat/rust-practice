use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};

struct Philosopher { // 哲学者を表す構造体Philosopher
    name: String,   // 名前メンバ(String)
    left: usize,
    right: usize,
}

impl Philosopher { // 構造体Philosopherに関する定義
    fn new(name: &str, left: usize, right: usize) -> Philosopher { // 関連関数newの定義
        Philosopher {   // Philosopherを定義
            name: name.to_string(),
            left: left,
            right: right,
        } // 関数内の最後の式なのでこれが戻り値になる
    }

    fn eat(&self, table: &Table){ // 明示的なselfパラメータを取るので関連関数ではなくメソッドとなる
        let _left = table.forks[self.left].lock().unwrap(); // 単にlockを獲得したいだけなので未使用だが，アンダースコアを使うと警告がなくなる
        thread::sleep(Duration::from_millis(150));
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} is eating.", self.name);

        thread::sleep(Duration::from_millis(1000));

        println!("{} is done eating.", self.name);
    }
}

struct Table {
    forks: Vec<Mutex<()>>,
}

fn main() {
    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

    let philosophers = vec![ // 可変長配列型Vec<T>
        Philosopher::new("Judith Butler", 0, 1), // 関連関数newがあるのでPhilosopher{name: "hoge".to_string()};のように書かずに済む
        Philosopher::new("Gilles Deleuze", 1, 2),
        Philosopher::new("Karl Marx", 2, 3),
        Philosopher::new("Emma Goldman", 3, 4),
        Philosopher::new("Michel Foucault", 0, 4),
    ];

    let handles: Vec<_> = philosophers.into_iter().map(|p|{
        let table = table.clone();

        thread::spawn(move || { // thread::spawnはクロージャを1つ引数にとり，新しいスレッドでそれを実行．moveにより所有権がクロージャに移動
            p.eat(&table);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}
