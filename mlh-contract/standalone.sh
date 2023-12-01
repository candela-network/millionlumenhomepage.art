docker pull stellar/quickstart:soroban-dev
docker run --rm -it \
	-p 8000:8000 \
	--name stellar-standalone \
	stellar/quickstart:soroban-dev \
	--standalone \
	--enable-soroban-rpc
