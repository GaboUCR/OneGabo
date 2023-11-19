import os
import datetime
import asyncio

def get_files_info(directory_path):
    files_info = []
    for filename in os.listdir(directory_path):
        full_path = os.path.join(directory_path, filename)
        if os.path.isfile(full_path):
            modified_time = os.path.getmtime(full_path)
            last_modified_date = datetime.datetime.fromtimestamp(modified_time)
            files_info.append((filename, last_modified_date))
    return files_info

async def periodic_file_check():
    while True:
        print("Revisando archivos...")
        files_info = get_files_info("disk")
        for filename, modified_date in files_info:
            print(f"Archivo: {filename}, Última modificación: {modified_date}")

        await asyncio.sleep(3)