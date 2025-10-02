# UDP通信
urecv:
	cargo run --bin udp_recv

usend:
	cargo run --bin udp_send

# TCP通信
trecv:
	cargo run --bin tcp_recv

tsend:
	cargo run --bin tcp_send

.PHONY: urecv usend trecv tsend