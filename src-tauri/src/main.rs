#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod gui;

fn main() {
    gui::gui_main();
}
