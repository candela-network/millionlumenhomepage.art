soroban config identity generate admin
sleep 10
soroban config identity fund admin --network standalone
sleep 10
soroban lab token wrap --asset native --network standalone --source admin
