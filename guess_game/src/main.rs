extern crate rand;  //外部のクレートを使うよ宣言

//includeとかusing的な奴
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("数当てゲーム!");
    //乱数生成
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("秘密の数字は: {}です", secret_number);
    println!("数字を入力");

    //rustはデフォルトでconstなので、変更したい場合はmutを付ける
    let mut guess = String::new();

    //Rustの入力
    //read_line()は処理結果もしくはエラー内容を表す型としてResult型を返す
    //この戻り値を使わないと警告が出る
    //expect()はエラーが起きた時のメッセージを引数に取る
    io::stdin().read_line(&mut guess)
        .expect("read_lineに失敗");

    // {} はプレースホルダで変数の中身を出力する
    println!("入力した値は: {}", guess);
    //rustの変数は途中で型を変えることができる
    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");
    //パターンマッチ
    match guess.cmp(&secret_number) {
        Ordering::Less    => println!("ちいさい!"),
        Ordering::Greater => println!("おおきい!"),
        Ordering::Equal   => println!("あたり!"),
    }
}