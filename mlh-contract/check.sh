soroban config identity generate test
curl -s "http://localhost:8000/friendbot?addr=$(soroban config identity address test)"
sleep 10
soroban contract invoke --id CDMLFMKMMD7MWZP3FKUBZPVHTUEDLSX4BYGYKH4GCESXYHS3IHQ4EIG4 --source test --network standalone -- transfer --from test --to test --amount 9000
soroban contract invoke --id CDMLFMKMMD7MWZP3FKUBZPVHTUEDLSX4BYGYKH4GCESXYHS3IHQ4EIG4 --source test --network standalone -- transfer --from test --to test --amount 9000
