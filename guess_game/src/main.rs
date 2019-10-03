use std::io;

fn main() {
    println!("数当てゲーム!");

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
    println!("入力した値は: {}です", guess);
}