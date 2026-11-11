pub mod error;
pub mod model;

use actix_web::{HttpResponse, Responder, web};
use error::AppError;
use model::{AppState, Device, DeviceKind, Room};
use uuid::Uuid;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/rooms")
            .route("", web::get().to(list_rooms))
            .route("", web::post().to(create_room))
            .service(
                web::scope("/{room_id}")
                    .route("", web::get().to(get_room))
                    .route("", web::delete().to(delete_room))
                    .service(
                        web::scope("/devices")
                            .route("", web::get().to(list_devices))
                            .route("", web::post().to(add_device))
                            .route("/{dev_id}", web::get().to(get_device))
                            .route("/{dev_id}", web::delete().to(delete_device)),
                    ),
            ),
    )
    .route("/report", web::get().to(report));
}

async fn list_rooms(state: web::Data<AppState>) -> impl Responder {
    let rooms = state.lock().unwrap();
    HttpResponse::Ok().json(rooms.values().cloned().collect::<Vec<_>>())
}

async fn create_room(
    state: web::Data<AppState>,
    body: web::Json<serde_json::Value>,
) -> Result<HttpResponse, AppError> {
    let name = body["name"]
        .as_str()
        .ok_or_else(|| AppError::BadRequest("требуется name".into()))?;
    let room = Room::new(name);
    let mut rooms = state.lock().unwrap();
    rooms.insert(room.id, room.clone());
    Ok(HttpResponse::Ok().json(room))
}

async fn get_room(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let rooms = state.lock().unwrap();
    rooms
        .get(&id)
        .map(|r| HttpResponse::Ok().json(r))
        .ok_or(AppError::RoomNotFound)
}

async fn delete_room(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let mut rooms = state.lock().unwrap();
    rooms
        .remove(&id)
        .map(|_| HttpResponse::Ok().finish())
        .ok_or(AppError::RoomNotFound)
}

async fn list_devices(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let rooms = state.lock().unwrap();
    rooms
        .get(&id)
        .map(|r| HttpResponse::Ok().json(r.devices.values().cloned().collect::<Vec<_>>()))
        .ok_or(AppError::RoomNotFound)
}

async fn add_device(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<serde_json::Value>,
) -> Result<HttpResponse, AppError> {
    let room_id = path.into_inner();
    let name = body["name"]
        .as_str()
        .ok_or_else(|| AppError::BadRequest("требуется name".into()))?;
    let kind = serde_json::from_value::<DeviceKind>(body["kind"].clone())
        .map_err(|_| AppError::BadRequest("kind должен быть socket или thermometer".into()))?;
    let state_str = body["state"].as_str().unwrap_or("unknown").to_string();

    let device = Device {
        id: Uuid::new_v4(),
        name: name.to_string(),
        kind,
        state: state_str,
    };

    let mut rooms = state.lock().unwrap();
    let room = rooms.get_mut(&room_id).ok_or(AppError::RoomNotFound)?;
    room.devices.insert(device.id, device.clone());
    Ok(HttpResponse::Ok().json(device))
}

async fn get_device(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, AppError> {
    let (room_id, dev_id) = path.into_inner();
    let rooms = state.lock().unwrap();
    rooms
        .get(&room_id)
        .and_then(|r| r.devices.get(&dev_id))
        .map(|d| HttpResponse::Ok().json(d))
        .ok_or(AppError::DeviceNotFound)
}

async fn delete_device(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, AppError> {
    let (room_id, dev_id) = path.into_inner();
    let mut rooms = state.lock().unwrap();
    let room = rooms.get_mut(&room_id).ok_or(AppError::RoomNotFound)?;
    room.devices
        .remove(&dev_id)
        .map(|_| HttpResponse::Ok().finish())
        .ok_or(AppError::DeviceNotFound)
}

async fn report(state: web::Data<AppState>) -> impl Responder {
    let rooms = state.lock().unwrap();
    let total_devices: usize = rooms.values().map(|r| r.devices.len()).sum();
    let mut by_kind: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for r in rooms.values() {
        for d in r.devices.values() {
            let key = format!("{:?}", d.kind).to_lowercase();
            *by_kind.entry(key).or_default() += 1;
        }
    }
    HttpResponse::Ok().json(serde_json::json!({
        "room_count": rooms.len(),
        "total_devices": total_devices,
        "by_kind": by_kind,
        "rooms": rooms.values().map(|r| serde_json::json!({
            "id": r.id,
            "name": r.name,
            "device_count": r.devices.len(),
        })).collect::<Vec<_>>(),
    }))
}
