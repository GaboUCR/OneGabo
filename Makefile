all: server client

server:
	cd server && cargo run

dev_server:
	cd server && cargo run --release -- -A warnings

client:
	cd client && python3 main.py

.PHONY: all server client
