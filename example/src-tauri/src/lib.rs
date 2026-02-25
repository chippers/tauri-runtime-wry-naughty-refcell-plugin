use tauri_runtime_wry_naughty_refcell_plugin::TauriRuntimeWryNaughtyRefcellPlugin;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut app = tauri::Builder::default()
        .build(tauri::generate_context!())
        .expect("error while creating tauri application");

    app.wry_plugin(TauriRuntimeWryNaughtyRefcellPlugin);
    app.run(|_, _| {});
}
