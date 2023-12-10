#client/websocket_client.py
from file_manager import periodic_file_check, load_state_from_disk
import websockets
import asyncio

async def websocket_handler(uri, base_path="disk"):
    sent_files, sent_folders = load_state_from_disk()
    while True:
        try:
            async with websockets.connect(uri) as websocket:
                await periodic_file_check(websocket, sent_files, sent_folders, base_path)

                break
        except (ConnectionRefusedError, websockets.exceptions.WebSocketException) as e:
            print(f"Connection failed: {e}, retrying in 3 seconds...")
            await asyncio.sleep(3)  # Wait for 3 seconds before retrying
