# OneGabo
### Running OneGabo

#### Server Component

The server part of the OneGabo application is responsible for handling file synchronization requests from the client. Here's how to run it:

1. **Navigate to the Server Directory**: Open a terminal and change to the directory where your server code is located. The `server` folder in your project.

2. **Run the Server**: Execute the server by running the following command:

   ```bash
   cargo run
   ```

   This command compiles and starts the Rust server. The server will then listen for incoming connections and handle synchronization requests.

#### Client Component (CLI)

The client part of the OneGabo application, implemented as a Command Line Interface (CLI), communicates with the server to synchronize a specified folder. Here's how to use it:

1. **Navigate to the Client Directory**: Open a new terminal window and navigate to the directory containing the client code, the `client` folder in your project.

2. **Run the Client CLI**: To start the client and synchronize a folder, use the following command format:

   ```bash
   python3 cli.py [server_address] [path_to_folder]
   ```

   - `[server_address]`: This is the WebSocket URL of the server. For example, if running locally, it might be `ws://127.0.0.1:8080`.
   
   - `[path_to_folder]`: This is the path to the folder you want to synchronize with the server. The path should be relative to the current directory where you're running the command.

   **Example**:
   If your server is running locally and you want to synchronize a folder named `disk` in the current directory, run:

   ```bash
   python3 cli.py ws://127.0.0.1:8080 disk
   ```

#### Example Usage

- **Server**:
  - Open a terminal.
  - Navigate to your server's directory.
  - Run `cargo run`.
  - Wait for the 'Server listening!!' message

- **Client**:
  - Open another terminal.
  - Navigate to your client's directory.
  - Run `python3 cli.py ws://127.0.0.1:8080 disk`.

The server will start and wait for connections from the client. When you run the client CLI with the specified server address and folder path, it will connect to the server and begin the synchronization process for the given folder. The client will attempt to reconnect every 3 seconds if the connection fails.

### WebSocket Message Format

#### Overview
Our application utilizes WebSocket for client-server communication, where the client sends structured messages for file operations. Each message follows a specific format enabling efficient processing by the server. There are two primary types of messages: File Update and File Deletion.

#### Message Types

1. **File Update Message**: Used for creating or updating files on the server.

   - **Operation Code (4 bytes)**: An integer in little-endian format representing the operation (`UpdateFile`). Corresponds to the `MsgCode::UpdateFile` value.
   - **Filename Size (4 bytes)**: An integer in little-endian format indicating the size of the filename.
   - **Filename (Variable length)**: The filename in text format, length determined by the Filename Size field.
   - **File Content (Variable length)**: The remaining part of the message contains the file content.

2. **File Deletion Message**: Used for deleting files or directories on the server.

   - **Operation Code (4 bytes)**: An integer in little-endian format representing the operation (`DeleteFile`). Corresponds to the `MsgCode::DeleteFile` value.
   - **File Path (Remaining bytes)**: The path of the file or directory to be deleted, in text format.


#### Implementation Notes

- **Little-Endian Format**: Integers (like operation codes and filename sizes) are in little-endian format for cross-platform compatibility.
- **Error Handling**: The server is designed to handle incorrect message formats, such as invalid operation codes or mismatched filename sizes.
- **Modular Design**: This structured approach allows for flexible handling of various file operations and clear parsing of messages.