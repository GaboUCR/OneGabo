import os
import datetime
import asyncio
from message_handler import create_update_message, create_delete_message

def get_files_and_folders_info(directory_path):
    files_info = []
    folders_info = []
    for root, dirs, files in os.walk(directory_path):
        # Include directory information
        for dir_name in dirs:
            dir_path = os.path.join(root, dir_name)
            folders_info.append(os.path.relpath(dir_path, directory_path)) 

        # Include file information
        for filename in files:
            full_path = os.path.join(root, filename)
            modified_time = os.path.getmtime(full_path)
            last_modified_date = datetime.datetime.fromtimestamp(modified_time)
            files_info.append((os.path.relpath(full_path, directory_path), last_modified_date))
    
    return (files_info, folders_info)

async def periodic_file_check(websocket, sent_files, sent_folders):
    while True:
        current_files_info, current_folders_info = get_files_and_folders_info("disk") 
        current_files_names = set([file_info[0] for file_info in current_files_info])

        # Check for new or modified files
        for filename, modified_date in current_files_info:
            if filename not in sent_files or sent_files[filename] != modified_date: # possible bug, only checks if dates are different
                with open(os.path.join("disk", filename), 'rb') as file:
                    file_content = file.read()
                    message = create_update_message(filename, file_content)
                    await websocket.send(message)
                    sent_files[filename] = modified_date
            
        # Check for deleted files
        deleted_items = set(sent_files.keys()) - current_files_names
        for filename in deleted_items:
            message = create_delete_message(filename)
            await websocket.send(message)
            del sent_files[filename] 
        
        # Check for deleted folders
        deleted_folders =  sent_folders - set(current_folders_info) 
        for folder_path in deleted_folders:
            message = create_delete_message(folder_path)
            await websocket.send(message)
            sent_folders.remove(folder_path)
        
        # Adding new files if there's any
        for folder_path in current_folders_info:
            sent_folders.add(folder_path)

        await asyncio.sleep(0.314)