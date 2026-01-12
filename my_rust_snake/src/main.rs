// #![allow(dead_code)]
// #![allow(unused)]

mod app;
mod game;
use crossterm::{
    cursor, execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use std::io::{self, stdout};

/// 终端守卫者：负责自动进入和退出特殊模式
pub struct TerminalGuard;
impl TerminalGuard {
    pub fn new() -> io::Result<Self> {
        enable_raw_mode()?; // 开启原始模式
        execute!(stdout(), EnterAlternateScreen, cursor::Hide)?; // 进入交替屏幕，隐藏光标
        Ok(TerminalGuard)
    }
}

// 当 TerminalGuard 变量生命周期结束时，自动执行以下代码
impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = execute!(stdout(), cursor::Show, LeaveAlternateScreen); // 显示光标，退出交替屏幕
        let _ = disable_raw_mode(); // 关闭原始模式
    }
}
fn main() -> io::Result<()> {
    let _guard = TerminalGuard::new()?;
    app::run();
    Ok(())
}
