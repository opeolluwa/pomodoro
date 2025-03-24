use js_bindgen::tauri_plugins::sqlite::Database;
use serde::{Deserialize, Serialize};

pub async fn load_database() -> Database {
    Database::load("sqlite:pomodoro.db").await
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]

pub struct PomodoroTemplate {
    pub id: String,
    pub title: String,
    pub timer_config: PomodoroTemplateTimerConfig,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PomodoroTemplateTimerConfig {
    pub count: u8,
    pub short_break: u8,
    pub long_break: u8,
}





//  spawn_local(async move {
//             let name = name.get_untracked();
//             if name.is_empty() {
//                 return;
//             }

//             let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &name }).unwrap();
//             // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//             let new_msg = invoke("greet", args).await.as_string().unwrap();
//             set_greet_msg.set(new_msg);
//         });