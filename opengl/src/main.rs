// https://gist.github.com/aita/28fef7fa91fce3122b1005ae5d34a7ef
// https://rust-sdl2.github.io/rust-sdl2/sdl2/
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
fn main() {
    //sdlの初期化
    let sdl_context = sdl2::init().unwrap();
    //コンテキスト経由で必要な機能をもらう
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("SDL", 640, 480) //タイトルと解像度
        .position_centered() //ウィンドウの初期位置
        .build() //ウィンドウ生成ß
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(64, 128, 224));//ウィンドウカラー
    //ダブルバッファリング
    canvas.clear();

    canvas.present();
    //この変数に入力などのイベントが入る
    let mut event_pump = sdl_context.event_pump().unwrap();
    //イベントループ
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                //Escキーが押されるかEvent::Quitが呼ばれたらループ終了
                //memo:rustはswitch文よりmatchを使うことが多い（らしい）
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

    canvas.present();
    //60fpsになるように待つ
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
