all: server client

server:
	cd server && cargo run

client:
	cd client && python3 main.py

.PHONY: all server client
