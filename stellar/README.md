# Stellar / Soroban

```bash
stellar container start local
stellar contract build --package voting-contract
stellar contract deploy --wasm target/wasm32v1-none/release/voting_contract.wasm --source-account alice --network local
```

El contrato es educativo: no implementa voto secreto y debe auditarse antes de cualquier uso real.
