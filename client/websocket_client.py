from file_manager import periodic_file_check, load_state_from_disk
import websockets

async def websocket_handler(uri, base_path="disk"):
    # uri = "ws://127.0.0.1:8080"  # Reemplaza con la URI de tu servidor WebSocket

    sent_files, sent_folders = load_state_from_disk()

    async with websockets.connect(uri) as websocket:
        await periodic_file_check(websocket, sent_files, sent_folders, base_path)
