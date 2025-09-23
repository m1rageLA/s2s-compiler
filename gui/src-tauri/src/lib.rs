use std::time::{SystemTime, UNIX_EPOCH};

use ir::{ir_docs, IrDocNode};
use serde::Serialize;

#[derive(Serialize)]
struct IrDocsPayload {
    generated_at: u64,
    docs: Vec<IrDocNode>,
    json: String,
}

#[tauri::command]
fn sync_ir_docs() -> Result<IrDocsPayload, String> {
    let docs = ir_docs();
    let json = serde_json::to_string_pretty(&docs).map_err(|err| err.to_string())?;
    let generated_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|err| err.to_string())?
        .as_secs();

    Ok(IrDocsPayload {
        generated_at,
        docs,
        json,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![sync_ir_docs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
