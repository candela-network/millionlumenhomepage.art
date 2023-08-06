docker run --rm -it \
	-p 8000:8000 \
	--name stellar-standalone \
	stellar/quickstart:soroban-dev@sha256:53d36cfc176a5ffba0d5210bb2ac5333a99fd401564993abf63737f2f9f655ad \
	--standalone \
	--enable-soroban-rpc
