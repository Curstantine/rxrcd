use tauri::Runtime;

#[tauri::command]
pub async fn setup<R: Runtime>(app: tauri::AppHandle<R>) -> Result<(), String> {
	Ok(())
}
