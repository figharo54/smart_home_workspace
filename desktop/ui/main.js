const { invoke } = window.__TAURI__.core;

// Состояние UI
let currentRoomId = null;

function showError(err) {
    document.getElementById('error').textContent = String(err);
    console.error(err);
}

function clearError() {
    document.getElementById('error').textContent = '';
}

// --- Комнаты ---
async function loadRooms() {
    clearError();
    try {
        const rooms = await invoke('list_rooms');
        const list = document.getElementById('rooms-list');
        list.innerHTML = rooms.map(r =>
            `<li data-id="${r.id}">
                ${r.name} 
                <button style="float:right; color:red" onclick="window.deleteRoom('${r.id}')">×</button>
            </li>`
        ).join('');

        // Добавляем обработчики клика
        list.querySelectorAll('li').forEach(li => {
            li.addEventListener('click', (e) => {
                if (e.target.tagName !== 'BUTTON') {
                    selectRoom(li.dataset.id, li.textContent.trim().split('\n')[0]);
                }
            });
        });
    } catch (e) { showError(e); }
}

async function createRoom() {
    const name = document.getElementById('new-room-name').value.trim();
    if (!name) return;
    clearError();
    try {
        await invoke('create_room', { name });
        document.getElementById('new-room-name').value = '';
        await loadRooms();
    } catch (e) { showError(e); }
}

window.deleteRoom = async (id) => {
    if (!confirm('Удалить комнату?')) return;
    clearError();
    try {
        await invoke('delete_room', { id });
        if (currentRoomId === id) {
            currentRoomId = null;
            document.getElementById('devices-header').textContent = 'Выберите комнату';
            document.getElementById('devices-list').innerHTML = '';
        }
        await loadRooms();
    } catch (e) { showError(e); }
};

function selectRoom(id, name) {
    currentRoomId = id;
    document.getElementById('devices-header').textContent = `Устройства в комнате: ${name}`;
    loadDevices();
}

// --- Устройства ---
async function loadDevices() {
    if (!currentRoomId) return;
    clearError();
    try {
        const devices = await invoke('list_devices', { roomId: currentRoomId });
        const list = document.getElementById('devices-list');
        list.innerHTML = devices.map(d => `
            <li>
                <b>${d.name}</b> <i>(${d.kind})</i> — ${d.state}
                <button style="float:right; color:red" onclick="window.deleteDevice('${d.id}')">×</button>
            </li>
        `).join('');
    } catch (e) { showError(e); }
}

async function addDevice() {
    if (!currentRoomId) {
        showError('Сначала выберите комнату');
        return;
    }
    const name = document.getElementById('new-device-name').value.trim();
    const kind = document.getElementById('new-device-kind').value;
    if (!name) return;
    clearError();
    try {
        console.log("Отправка запроса add_device:", { roomId: currentRoomId, name, kind, state: 'off' });
        await invoke('add_device', { roomId: currentRoomId, name, kind, state: 'off' });
        document.getElementById('new-device-name').value = '';
        await loadDevices();
    } catch (e) { showError(e); }
}

window.deleteDevice = async (devId) => {
    if (!currentRoomId) return;
    if (!confirm('Удалить устройство?')) return;
    clearError();
    try {
        await invoke('delete_device', { roomId: currentRoomId, devId });
        await loadDevices();
    } catch (e) { showError(e); }
};

// --- Отчёт ---
async function showReport() {
    clearError();
    document.getElementById('report').textContent = 'Загрузка...';
    try {
        const report = await invoke('get_report');
        document.getElementById('report').textContent = JSON.stringify(report, null, 2);
    } catch (e) {
        document.getElementById('report').textContent = 'Ошибка';
        showError(e);
    }
}

// --- Инициализация ---
window.addEventListener('DOMContentLoaded', () => {
    document.getElementById('btn-load-rooms').addEventListener('click', loadRooms);
    document.getElementById('btn-add-room').addEventListener('click', createRoom);
    document.getElementById('btn-add-device').addEventListener('click', addDevice);
    document.getElementById('btn-report').addEventListener('click', showReport);

    loadRooms();
});
