import asyncio
import websockets
from file_manager import periodic_file_check

async def websocket_client():
    uri = "ws://127.0.0.1:8080"  # Reemplaza con la URI de tu servidor WebSocket
    sent_files = {}

    async with websockets.connect(uri) as websocket:
        await periodic_file_check(websocket, sent_files)

asyncio.run(websocket_client())