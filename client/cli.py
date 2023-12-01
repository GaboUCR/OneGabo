import argparse
from websocket_client import websocket_handler
import asyncio

def parse_arguments():
    parser = argparse.ArgumentParser(description='')
    parser.add_argument('address', help='IP address or domain name of the server.')
    parser.add_argument('path', help='Path of the folder to be synchronized relative to the location of the script.')

    return parser.parse_args()

async def main():
    args = parse_arguments()
    uri = args.address
    base_path = args.path

    await websocket_handler(args.address, args.path)

if __name__ == "__main__":
    asyncio.run(main())