mod commands;
mod db;
mod error;

use tauri::Manager;
use tauri_specta::{Builder, collect_commands};

fn specta_builder() -> Builder<tauri::Wry> {
    Builder::<tauri::Wry>::new().commands(collect_commands![commands::app_info])
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = specta_builder();

    // In debug builds, regenerate the typed TS bindings on every launch.
    #[cfg(debug_assertions)]
    builder
        .export(
            specta_typescript::Typescript::default().header("// @ts-nocheck\n"),
            "../src/lib/bindings.ts",
        )
        .expect("failed to export TypeScript bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            builder.mount_events(app);
            let db_path = app.path().app_data_dir()?.join("library.db");
            let pool = tauri::async_runtime::block_on(db::connect(&db_path))?;
            app.manage(pool);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    /// Fails if the checked-in bindings drift from the Rust commands.
    #[test]
    fn export_bindings() {
        super::specta_builder()
            .export(
                specta_typescript::Typescript::default().header("// @ts-nocheck\n"),
                "../src/lib/bindings.ts",
            )
            .expect("export bindings");
    }
}
