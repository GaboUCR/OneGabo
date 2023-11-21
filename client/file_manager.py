import os
import datetime
import asyncio
import struct
from enum import Enum

class MsgCode (Enum):
    InitialMessage = 0,
    CreateFile = 1,
    UpdateFile = 2,
    NewFile = 3,
    DeleteFile = 4

def get_files_info(directory_path):
    files_info = []
    for root, dirs, files in os.walk(directory_path):
        for filename in files:
            full_path = os.path.join(root, filename)
            if os.path.isfile(full_path):
                modified_time = os.path.getmtime(full_path)
                last_modified_date = datetime.datetime.fromtimestamp(modified_time)
                relative_path = os.path.relpath(full_path, directory_path)
                files_info.append((relative_path, last_modified_date))
    return files_info

def build_message(operation_code, filename, file_content):
    # Empaquetar el código de operación (MsgCode.UpdateFile) en 4 bytes (little-endian)
    operation_bytes = struct.pack('<I', operation_code)

    # Empaquetar el tamaño del nombre del archivo en 4 bytes (little-endian)
    filename_size_bytes = struct.pack('<I', len(filename))

    # Convertir el nombre del archivo a bytes
    filename_bytes = filename.encode()

    # Construir el mensaje completo
    message = operation_bytes + filename_size_bytes + filename_bytes + file_content
    return message


async def periodic_file_check(websocket, sent_files):
    while True:
        current_files_info = get_files_info("disk")
        for filename, modified_date in current_files_info:
            # Comprobar si el archivo es nuevo o ha sido modificado desde el último envío
            if filename not in sent_files or sent_files[filename] != modified_date:
                with open(os.path.join("disk", filename), 'rb') as file:
                    file_content = file.read()
                    message = build_message(MsgCode.UpdateFile.value[0], filename, file_content)
                    await websocket.send(message)
                    
                    # Actualizar el diccionario con la nueva fecha de modificación
                    sent_files[filename] = modified_date

        await asyncio.sleep(3)
