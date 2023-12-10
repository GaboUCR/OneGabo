#client/message_handler.py
import struct
from enum import Enum

class MsgCode (Enum):
    InitialMessage = 0
    CreateFile = 1
    UpdateFile = 2
    NewFile = 3
    DeleteFile = 4

def create_update_message(filename, file_content):
    # Empaquetar el código de operación (MsgCode.UpdateFile) en 4 bytes (little-endian)
    code = MsgCode.UpdateFile.value
    operation_bytes = struct.pack('<I', code)

    # Empaquetar el tamaño del nombre del archivo en 4 bytes (little-endian)
    filename_size_bytes = struct.pack('<I', len(filename))

    # Convertir el nombre del archivo a bytes
    filename_bytes = filename.encode()

    # Construir el mensaje completo
    message = operation_bytes + filename_size_bytes + filename_bytes + file_content
    return message

def create_delete_message(file_name: str) -> bytes:
    # Message code for deletion
    code = MsgCode.DeleteFile.value

    # Pack the code into 4 bytes (little-endian)
    message = struct.pack('<I', code)

    # Encode the filename and append it
    message += file_name.encode()

    return message
