# Smart Home — веб-сервис управления умным домом

Проект состоит из двух компонентов:

- **backend** — REST API на [Actix Web](https://actix.rs/), хранит данные в памяти.
- **desktop** — десктопное приложение на [Tauri 2](https://tauri.app/), UI на Vanilla HTML/CSS/JS.

UI не обращается к backend напрямую: все запросы идут через Tauri-команды(`#[tauri::command]`),которые внутри используют `reqwest`.

---

## Архитектура

```text
┌──────────────────────────────┐        invoke("list_rooms")        ┌────────────────────────────┐                                                                                                                                      
│  Desktop UI (HTML/CSS/JS)    │ ─────────────────────────────────► │  Tauri Rust-слой           │                                                                                                                                      
│  window.__TAURI__.core       │ ◄───────────────────────────────── │  #[tauri::command] fn ...  │                                                                                                                                      
└──────────────────────────────┘            Result<Value, String>   └─────────────┬──────────────┘                                                                                                                                      
                                                                                  │ reqwest::Client                                                                                                                                      
                                                                                  ▼                                                                                                                                      
                                                                    ┌────────────────────────────┐                                                                                                                                      
                                                                    │  Backend (Actix Web)       │                                                                                                                                      
                                                                    │  REST API, in-memory state │                                                                                                                                      
                                                                    └────────────────────────────┘      
```

Ключевой принцип: **UI не знает про HTTP**. Он вызывает `invoke("list_rooms")`,`invoke("create_room", { name })` и т.д.
**Rust**-команды в `desktop/src-tauri/src/commands.rs` делают `reqwest`-запросы к **backend** и возвращают **JSON** или строку ошибки.

## Backend (Actix Web)

- Точка входа: `backend/src/main.rs` — поднимает `HttpServer` на `127.0.0.1:3000`.
- Общее состояние: `Arc<Mutex<HashMap<Uuid, Room>>>` в `web::Data<AppState>`.
- Маршруты регистрируются в `backend/src/lib.rs` (`pub fn config`).
- Ошибки: `AppError` реализует `ResponseError` → **HTTP**-статус + **JSON** `{ "error": "..." }`.
- Модели: `Room`, `Device`, `DeviceKind` (`backend/src/model.rs`), сериализуются через `serde`.

## Desktop (Tauri 2)

- Точка входа: `desktop/src-tauri/src/main.rs` — регистрирует команды через `tauri::generate_handler!`.
- Команды: `desktop/src-tauri/src/commands.rs` — делают **HTTP**-запросы к **backend** через `reqwest`.
- **UI**: `desktop/ui/` **(HTML/CSS/JS)**, общается с **Rust** только через `invoke`.
- Адрес **backend** задан константой `BACKEND = "http://127.0.0.1:3000"` в `commands.rs`.

## REST API


| Метод  | Путь                         | Описание                        |
|--------|------------------------------|---------------------------------|
| GET    | /rooms                       | Список комнат                   |
| POST   | /rooms                       | Создать комнату { "name": ... } |
| GET    | /rooms/{id}                  | Комната по id                   |
| DELETE | /rooms/{id}                  | Удалить комнату                 |
| GET    | /rooms/{id}/devices          | Устройства комнаты              |
| POST   | /rooms/{id}/devices          | Добавить устройство             |
| GET    | /rooms/{id}/devices/{dev_id} | Устройство по id                |
| DELETE | /rooms/{id}/devices/{dev_id} | Удалить устройство              |
| GET    | /report                      | Отчёт о доме                    |


Тело устройства:

```json
{ "name": "Чайник", "kind": "socket", "state": "on" }
```

kind — "socket" или "thermometer".

## Требования

- **Rust** (stable, edition 2024)
- **Node.js 18+** и **npm** — для сборки фронтенда (**Tauri CLI**)
- **Системные зависимости Tauri 2**:
   - **Windows**: WebView2 (обычно предустановлен), MSVC Build Tools
   - **Linux**: `libwebkit2gtk-4.1-dev`, `build-essential`, `libssl-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`
   - **macOS**: Xcode Command Line Tools
   - Подробнее: https://tauri.app/start/prerequisites/
   
## Запуск backend
cargo run -p smart-home-backend

Ожидаемый вывод:

```text
Бэкенд слушает http://127.0.0.1:3000
```
Проверка из другого терминала:

	curl http://127.0.0.1:3000/rooms 
	curl -X POST http://127.0.0.1:3000/rooms -H `"content-type: application/json"` -d `"{\"name\":\"Кухня\"}"`

## Запуск desktop-приложения
Перед первым запуском необходимо установить зависимости фронтенда и сгенерировать иконки.

1. Перейдите в папку `desktop`:

	cd desktop

2. Установите npm-зависимости (Tauri CLI):

	npm install

3. Запустите приложение:

	npm run tauri dev

(Альтернативный вариант без npm: `cargo tauri dev` из папки `desktop/src-tauri`)

**Важно**: Desktop требует, чтобы backend был уже запущен — иначе UI отобразит ошибку «бэкенд недоступен».

Для production-сборки используйте `npm run tauri build`.

## Запуск тестов

	cargo test -p smart-home-backend

Тесты используют `actix_web::test::init_service` и не требуют запущенного **HTTP**-сервера. Они проверяют создание/получение комнат и устройств, списки, удаление, отчёт и обработку ошибок (404).

Полный набор проверок перед сдачей:

	cargo test --workspace
	cargo clippy --workspace --all-targets -- -D warnings
	cargo fmt --check

## Конфигурация

| Параметр      | Где задаётся                                         | Значение по умолчанию |
|---------------|------------------------------------------------------|-----------------------|
| Адрес backend | desktop/src-tauri/src/commands.rs, константа BACKEND | http://127.0.0.1:3000 |
| Порт backend  | backend/src/main.rs, bind(("127.0.0.1", 3000))       | 3000                  |
		
		
## Структура репозитория
```text
.
├── Cargo.toml           # workspace
├── README.md
├── backend/             # Actix Web REST API
│   ├── Cargo.toml
│   ├── src/
│   └── tests/
└── desktop/             # Tauri 2
    ├── package.json
    ├── ui/              # HTML/JS фронтенд
    └── src-tauri/       # Rust-часть Tauri
        ├── Cargo.toml
        ├── tauri.conf.json
        ├── icons/
        └── src/
```
## Известные ограничения
- Данные хранятся в памяти: при перезапуске backend состояние теряется.
- Нет аутентификации — проект учебный, рассчитан на локальный запуск.
