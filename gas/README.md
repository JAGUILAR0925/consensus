# Medición de recursos (equivalente a gas)

Soroban no usa gas EVM fijo. Se mide cada escenario con `simulateTransaction` y se conserva JSON de recursos y `resourceFee`.

```bash
stellar contract invoke --id "$CONTRACT_ID" --source-account alice --network local --send=yes -- cast --voter alice --option A
```

Guardar resultados en `docs/evidence/` (esa carpeta está ignorada para evitar evidencias accidentales con secretos).
