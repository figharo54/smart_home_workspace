const BACKEND: &str = "http://127.0.0.1:3000";

#[tauri::command]
pub async fn list_rooms() -> Result<serde_json::Value, String> {
    reqwest::get(format!("{BACKEND}/rooms"))
        .await
        .map_err(|e| format!("бэкенд недоступен: {e}"))?
        .json()
        .await
        .map_err(|e| format!("ошибка разбора: {e}"))
}

#[tauri::command]
pub async fn create_room(name: String) -> Result<serde_json::Value, String> {
    let resp = reqwest::Client::new()
        .post(format!("{BACKEND}/rooms"))
        .json(&serde_json::json!({ "name": name }))
        .send()
        .await
        .map_err(|e| format!("бэкенд недоступен: {e}"))?;
    if !resp.status().is_success() {
        return Err(resp.text().await.unwrap_or_default());
    }
    resp.json().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_room(id: String) -> Result<(), String> {
    let resp = reqwest::Client::new()
        .delete(format!("{BACKEND}/rooms/{id}"))
        .send()
        .await
        .map_err(|e| format!("бэкенд недоступен: {e}"))?;
    if !resp.status().is_success() {
        return Err(resp.text().await.unwrap_or_default());
    }
    Ok(())
}

#[tauri::command]
pub async fn delete_device(room_id: String, dev_id: String) -> Result<(), String> {
    let resp = reqwest::Client::new()
        .delete(format!("{BACKEND}/rooms/{room_id}/devices/{dev_id}"))
        .send()
        .await
        .map_err(|e| format!("бэкенд недоступен: {e}"))?;
    if !resp.status().is_success() {
        return Err(resp.text().await.unwrap_or_default());
    }
    Ok(())
}

#[tauri::command]
pub async fn list_devices(room_id: String) -> Result<serde_json::Value, String> {
    reqwest::get(format!("{BACKEND}/rooms/{room_id}/devices"))
        .await
        .map_err(|e| format!("бэкенд недоступен: {e}"))?
        .json()
        .await
        .map_err(|e| format!("ошибка разбора: {e}"))
}

#[tauri::command]
pub async fn add_device(
    room_id: String,
    name: String,
    kind: String,
    state: String,
) -> Result<serde_json::Value, String> {
    let resp = reqwest::Client::new()
        .post(format!("{BACKEND}/rooms/{room_id}/devices"))
        .json(&serde_json::json!({ "name": name, "kind": kind, "state": state }))
        .send()
        .await
        .map_err(|e| format!("бэкенд недоступен: {e}"))?;
    if !resp.status().is_success() {
        return Err(resp.text().await.unwrap_or_default());
    }
    resp.json().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_report() -> Result<serde_json::Value, String> {
    reqwest::get(format!("{BACKEND}/report"))
        .await
        .map_err(|e| format!("бэкенд недоступен: {e}"))?
        .json()
        .await
        .map_err(|e| format!("ошибка разбора: {e}"))
}
