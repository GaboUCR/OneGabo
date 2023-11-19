import asyncio
import websockets
from file_manager import periodic_file_check

async def websocket_client():
    uri = "ws://127.0.0.1:8080"  # Reemplaza con la URI de tu servidor WebSocket
    async with websockets.connect(uri) as websocket:
        binary_message = bytes([0])
        await websocket.send(binary_message)
        response = await websocket.recv()
        print(f"Received from server: {response}")

# asyncio.run(websocket_client())
asyncio.run(periodic_file_check())

