/** https://doc.rust-jp.rs/the-rust-programming-language-ja/1.6/book/dining-philosophers.html */
use std::thread;
use std::time::Duration;
use device_query::{DeviceQuery, DeviceState, Keycode};  //キー入力を使用可能にする

struct Philosopher 
{
    name: String,
}

impl Philosopher 
{
    //構造体の関連関数(メソッドではないC++でいう名前空間に近い)->は戻り値
    fn new(name: &str) -> Philosopher 
    {
        Philosopher {
            name: name.to_string(),
        }
    }
    //自身の参照を引数に取るとメソッドになる
    fn eat(&self) 
    {
        
        println!("{} は食べています.", self.name);
        thread::sleep(Duration::from_millis(100));
        println!("{} は食べ終わりました.", self.name);
    }
}

fn wait_key()
{
    println!("何かキーを押すと終了します");
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

    let philosophers = vec!
    [
        Philosopher::new("Judith Butler"),
        Philosopher::new("Gilles Deleuze"),
        Philosopher::new("Karl Marx"),
        Philosopher::new("Emma Goldman"),
        Philosopher::new("Michel Foucault"),
    ];
    // for(const auto& i : array)的な
    for p in &philosophers 
    {
        p.eat();
    }
    wait_key();
}