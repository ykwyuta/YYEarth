//! 地球史図鑑 — ゲーム本体（地球・生きもの・観察・図鑑・宿題）と、描画・入力のつなぎ目
//!
//! - ブラウザ版（wasm32 など Windows 以外）：web.rs が web/main.js から入力とフレーム更新を受け取り、
//!   gfx.rs が WebGL2 / Canvas2D に、platform.rs が IndexedDB などに橋渡しする。
//! - デスクトップ版（Windows）：win/app.rs の Win32 メッセージループが動かし、
//!   win/gfx.rs が Direct3D11 / Direct2D / DirectWrite で描き、win/platform.rs が %APPDATA% に記録する。
//!
//! 描画・入力・記録の層は YYMongolia と同じものを使い、ゲーム本体（earth・life・dive・game など）を作り直した。

#[macro_use]
mod lang;
mod art;
mod artwork;
#[cfg(test)]
mod asset_list;
mod units;
mod reveal;
mod dive;
mod earth;
mod game;
mod life;
mod mission;
mod progress;
mod rng;
mod save;

#[cfg(not(windows))]
mod gfx;
#[cfg(windows)]
#[path = "win/gfx.rs"]
mod gfx;

#[cfg(not(windows))]
pub mod platform;
#[cfg(windows)]
#[path = "win/platform.rs"]
pub mod platform;

#[cfg(windows)]
#[path = "win/crashlog.rs"]
mod crashlog;

#[cfg(not(windows))]
mod web;

#[cfg(windows)]
#[path = "win/app.rs"]
mod app;

/// デスクトップ版（Windows）の入口
#[cfg(windows)]
pub fn desktop_main() {
    app::main();
}

#[derive(Clone, Copy, Debug)]
pub enum Ev {
    MouseMove(f32, f32),
    Down(u8, f32, f32),
    Up(u8, f32, f32),
    Wheel(f32),
    Key(u32, bool),
    Char(char),
    Resize(u32, u32),
    /// スマートフォン向けの詰めた画面か
    Compact(bool),
    /// 続く入力がタッチ操作か
    Touch(bool),
    /// 画面（図鑑などの一覧）を指でなぞったスクロール（実ピクセル）
    Scroll(f32),
}
