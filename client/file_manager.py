import os
import datetime
import asyncio
from message_handler import create_update_message, create_delete_message

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

async def periodic_file_check(websocket, sent_files):
    while True:
        current_files_info = get_files_info("disk")
        current_file_names = set([file_info[0] for file_info in current_files_info])

        # Check for new or modified files
        for filename, modified_date in current_files_info:
            if filename not in sent_files or sent_files[filename] != modified_date:
                with open(os.path.join("disk", filename), 'rb') as file:
                    file_content = file.read()
                    message = create_update_message(filename, file_content)
                    await websocket.send(message)
                    
                    sent_files[filename] = modified_date

        # Check for deleted files
        deleted_files = set(sent_files.keys()) - current_file_names
        for filename in deleted_files:
            message = create_delete_message(filename)
            await websocket.send(message)

            del sent_files[filename]  # Remove from sent_files dictionary

        await asyncio.sleep(0.314)
