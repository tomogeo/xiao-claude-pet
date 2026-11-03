// 不要在 Windows 上多開一個黑色命令列視窗
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    xiao_claude_lib::run()
}
