import asyncio
from websocket_client import websocket_handler

asyncio.run(websocket_handler("ws://127.0.0.1:8080"))