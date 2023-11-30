all: server client

server:
	cd server && cargo run

client:
	cd client && python3 main.py

client_test:
	cd client && python3 -m unittest test.py

server_test:
	cd server && python3 test.py


.PHONY: all server client