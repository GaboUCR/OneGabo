# OneGabo

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
