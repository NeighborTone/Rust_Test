/** https://doc.rust-jp.rs/the-rust-programming-language-ja/1.6/book/dining-philosophers.html */
use std::thread;
use std::time::Duration;
use std::sync::{Mutex,Arc};
#[cfg(target_os="windows")]
use device_query::{DeviceQuery, DeviceState, Keycode};  //キー入力を使用可能にする(macには対応していなかった...)

struct Philosopher 
{
    name: String,
    left: usize,
    right: usize,
}

impl Philosopher 
{
    //構造体の関連関数(メソッドではないC++でいう名前空間に近い)->は戻り値
    fn new(name: &str, left: usize, right: usize) -> Philosopher 
    {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }
    //自身の参照を引数に取るとメソッドになる
    fn eat(&self, table: &Table) 
    {
        //変数名の先頭に_をつけると未使用でも警告が出ない
        let _left = table.forks[self.left].lock().unwrap();
        thread::sleep(Duration::from_millis(150));
        let _right = table.forks[self.right].lock().unwrap();
        
        println!("{} は食べています.", self.name);
        thread::sleep(Duration::from_millis(600));
        println!("{} は食べ終わりました.", self.name);
    }
}

struct Table
{
    forks: Vec<Mutex<()>>,
}
//キー入力されるまで処理を止める
//strはプリミティブ型Stringはライブラから提供されている型
#[cfg(target_os="windows")]
fn wait_key(str: &str)
{
    println!("{}", str);
    loop
    {
        let device_state = DeviceState::new();
        let keys: Vec<Keycode> = device_state.get_keys();
        
        if !keys.is_empty()
        {
            break;
        }
        // for key in keys.iter()
        // {
        //     //{:?} では fmt::Debug の実装が使われる()
        //     //構造体の中身をダンプできる
        //     println!("{:?}を押しました", key);
        // }
    }
}
fn main() 
{
    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});
    let philosophers = vec!
    [
        Philosopher::new("Judith Butler", 0, 1),
        Philosopher::new("Gilles Deleuze", 1 , 2),
        Philosopher::new("Karl Marx", 2, 3),
        Philosopher::new("Emma Goldman", 3, 4),
        Philosopher::new("Michel Foucault", 0, 4),
    ];
    //スレッドを制御するハンドルを返す
    //Vec<_>はアノテーション(型をタグとして使う)
    //_はプレースホルダで、Rustに型を解決させる(コンパイル時型推論？)
    let handles: Vec<_> = philosophers.into_iter().map(|p|  //philosophersの所有権を持つイテレータを生成
    {
        let table = table.clone();
        //moveはクロージャを渡すときのキーワード
        thread::spawn(move ||
        {
            p.eat(&table); //各スレッドに処理を割り当てている
        })  
    }).collect(); //Vec<_>型(コレクション)を返す

    for h in handles
    {
        //実行が終わるまで処理をブロックする。（割り込み防止）
        h.join().unwrap();
    }
    #[cfg(target_os="windows")]
    wait_key("何かキーを押すと終了します");
}