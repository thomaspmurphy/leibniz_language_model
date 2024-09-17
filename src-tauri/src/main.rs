// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rand::prelude::IteratorRandom;
use rand::seq::SliceRandom;
use std::sync::Mutex;

static LAST_QUOTE: Mutex<Option<usize>> = Mutex::new(None);

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_leibniz_quote])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn get_leibniz_quote() -> String {
    let quotes = vec![
        "...it is clear that minds are to simple souls as the infinite is to the finite, or as the finite is the infinitely small.",
        "When God calculates and thinks things through, the world arises",
        "Music is the pleasure the human mind experiences from counting without being aware that it is counting.",
        "A body corresponds to the situation of a point or present state; but souls correspond to the degree of change in the motion of the point.",
    ];

    let mut last_index = LAST_QUOTE.lock().unwrap();
    let mut rng = rand::thread_rng();

    let new_index = loop {
        let index = (0..quotes.len()).choose(&mut rng).unwrap();
        if Some(index) != *last_index {
            break index;
        }
    };

    *last_index = Some(new_index);
    quotes[new_index].to_string()
}
