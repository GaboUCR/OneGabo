import os

def check_file_content(file_path, expected_content):
    """Check if the file at `file_path` has `expected_content`."""
    try:
        with open(file_path, 'r') as file:
            content = file.read()
            assert content == expected_content, f"Content mismatch in {file_path}"
            # print(f"Content of {file_path} is correct.")
    except FileNotFoundError:
        print(f"File not found: {file_path}")
    except AssertionError as error:
        print(error)

def main():
    base_path = "drive/127.0.0.1"
    # Define the test files and their expected contents
    test_files = {
        'file1.txt': 'Content of file 1',
        'file2.txt': 'Content of file 2',
        'folder1/file3.txt': 'Content of file 3',
        'folder1/file4.txt': 'Content of file 4',
        'folder2/file5.txt': 'Content of file 5',
        'folder2/subfolder1/file6.txt': 'Content of file 6',
        'folder2/subfolder1/file7.txt': 'Content of file 7',
        'folder3/file8.txt': 'Content of file 8',
        'folder3/file9.txt': 'Content of file 9',
        'folder3/subfolder2/file10.txt': 'Content of file 10',
        'folder3/subfolder2/file11.txt': 'Content of file 11',
        'large_files/large_file1.txt': 'This is a large file content.' * 10000,  # Large content
        'large_files/large_file2.txt': 'This is another large file content.' * 15000,  # Even larger content
        'folder4/file12.txt': 'Content of file 12',
        'folder4/file13.txt': 'Content of file 13',
        'folder4/subfolder3/file14.txt': 'Content of file 14',
        'folder4/subfolder3/file15.txt': 'Content of file 15',
        'folder4/subfolder3/file16.txt': 'Content of file 16',
    }

    # Check each file
    for file_path, expected_content in test_files.items():

        check_file_content(os.path.join(base_path, file_path), expected_content)

if __name__ == '__main__':
    main()
