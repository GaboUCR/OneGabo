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

The client in our application sends messages to the server using the WebSocket protocol. Each message is structured in a specific format that enables the server to process and act upon it effectively. The format of the message is as follows:

1. **Operation Code (4 bytes)**:
   - The first 4 bytes of the message represent an integer in little-endian format indicating the operation to be performed.
   - Each operation corresponds to a value defined in the `MsgCode` enumeration.

2. **Filename Size (4 bytes)**:
   - The next 4 bytes represent another integer in little-endian format indicating the size of the filename.
   - This size determines how many bytes of the message should be read to extract the filename.

3. **Filename (Variable length)**:
   - The following bytes, the length of which is determined by the previous field, contain the filename.
   - The filename is encoded in a text format (e.g., UTF-8).

4. **File Content (Remaining bytes)**:
   - The rest of the message contains the actual file content.
   - This part of the message will vary in length depending on the size of the file being sent.

### Implementation Notes

- **Little-Endian Format**: The use of little-endian format for integers (operation code and filename size) ensures compatibility across different platforms and architectures.

- **Modular Design**: The message is structured in a way that allows for easy parsing and flexibility in handling different types of file operations.

- **Error Handling**: The server implementation is expected to handle cases where the message format does not conform to this specification, such as incorrect operation codes or mismatched filename sizes.

This message format is designed to facilitate efficient and clear communication between the client and server in our WebSocket-based file management system. 

---
