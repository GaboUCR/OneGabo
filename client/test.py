import os
import unittest
import shutil
from websocket_client import websocket_handler
import asyncio

class TestClient(unittest.TestCase):

    def setUp(self):
        print("run")
        try:
            self.test_env = setup_test_environment()
        except Exception as e:
            print(f"Error in setUp: {e}")
    
    def tearDown(self):
        cleanup_test_environment() 
        

    def testSendOperation(self):
        asyncio.run(websocket_handler("ws://127.0.0.1:8080", "test_env"))
        
def setup_test_environment(base_path='test_env'):
    os.makedirs(base_path, exist_ok=True)

    # Create test files
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

    for path, content in test_files.items():
        full_path = os.path.join(base_path, path)
        os.makedirs(os.path.dirname(full_path), exist_ok=True)
        with open(full_path, 'w') as file:
            file.write(content)
    
    return base_path

def cleanup_test_environment():
    # Remove test files and folders
    shutil.rmtree('test_env')

if __name__ == '__main__':
    unittest.main()