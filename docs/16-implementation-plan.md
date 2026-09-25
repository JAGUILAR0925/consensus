# 16. Plan técnico de implementación

**Estado:** plan propuesto para revisión del equipo  
**Horizonte:** 6 semanas para el MVP educativo  
**Regla:** ninguna identidad, rostro, clave privada o voto real entra al repositorio.

## Decisión de arquitectura propuesta

Consensus se implementará como una dapp de Stellar usando contratos Soroban escritos en Rust. TypeScript será el cliente web, el orquestador de casos de uso y el adaptador de Stellar. Python/FastAPI será opcional y únicamente para servicios off-chain aislados (por ejemplo, un experimento de detección facial con datos sintéticos). El contrato nunca recibirá imágenes ni plantillas biométricas.

```text
Navegador TypeScript
  ├─ autenticación local (WebAuthn/TOTP para demo)
  ├─ WalletGateway + Stellar SDK
  └─ API de lectura/verificación
          │
          ├── Soroban/Rust: estado, reglas, eventos y unicidad
          └── Python/FastAPI (opcional): elegibilidad experimental
                    └── base de datos efímera; sin datos biométricos reales
```

### Responsabilidad por tecnología

| Tecnología | Uso en Consensus | Fuera del MVP |
| --- | --- | --- |
| Stellar/Soroban | Registro verificable, estados, reglas y eventos | Mainnet y elecciones reales |
| Rust | Contrato pequeño, seguro y testeable | Lógica de negocio del frontend |
| TypeScript | UI, SDK, firma, consulta y pruebas de integración | Guardar secretos en navegador |
| Python/FastAPI | Endpoint experimental de elegibilidad | Decidir o contar votos |
| PostgreSQL/SQLite | Persistencia mínima off-chain, si hace falta | Fuente de verdad del resultado |
| WebAuthn o TOTP | Segundo factor para administradores de demo | Promesa de anonimato electoral |
| Docker | Entorno reproducible local | Operación productiva |
| Terraform | Infraestructura de laboratorio opcional | Crear una red Stellar administrada |
| Kubernetes/Nginx | No necesarios para el MVP | Escalamiento futuro, tras medir carga |

## Contrato Soroban mínimo

El contrato tendrá una sola elección activa y mantendrá únicamente datos no sensibles:

- `create_election`, `add_option`, `open`, `close`.
- `authorize_voter` con identificador hash de prueba.
- `cast_vote` con comprobación de estado, autorización y voto único.
- `result` y eventos de auditoría.
- Administrador separado de las cuentas votantes; para acciones administrativas se evaluará multisig.

El voto se diseñará como demostración verificable, no como voto secreto. Publicar en cadena una relación entre identidad y opción puede revelar la preferencia; por eso no se usarán personas reales.

## Privacidad y Face API

La biometría no es un requisito del MVP. El flujo de aprendizaje será:

1. Generar embeddings sintéticos o datos de prueba.
2. Ejecutar una comparación fuera de cadena en Python.
3. Devolver solo un resultado de elegibilidad de corta duración.
4. Registrar en Stellar un identificador opaco, nunca la imagen, embedding o score.

El cifrado homomórfico y las pruebas de conocimiento cero se reservan para una investigación posterior. No se debe afirmar que una API facial o un hash proporcionan anonimato, anti-coerción o identidad legal.

## Entornos y medición de recursos

El equivalente educativo de Ganache no es otra cadena EVM: se usará `stellar container start local` (Stellar Quickstart con RPC), Stellar CLI y Stellar Lab. La prueba de una transacción se hará con `simulateTransaction`; se guardarán en `docs/evidence/` los recursos, `resourceFee`, versión del protocolo y fecha. Testnet servirá para la demostración compartida; Futurenet solo si una funcionalidad experimental lo exige.

Las tarifas de Soroban dependen de instrucciones, lecturas/escrituras, bytes, eventos y almacenamiento. Por ello no se publicará un “gas fijo”: cada escenario tendrá una medición reproducible y un presupuesto máximo definido por el equipo.

## AWS y operación (OPEX)

Amazon Managed Blockchain no aloja ni valida Stellar: AMB Access ofrece nodos de Ethereum/Bitcoin y AMB Networks usa Hyperledger Fabric. Para Consensus se mantendrá Stellar Quickstart local, Testnet o un RPC compatible de Stellar. AWS solo hospedará componentes auxiliares si el laboratorio lo necesita:

- S3/CloudFront o hosting estático para la UI.
- Lambda/API Gateway para endpoints de lectura muy pequeños.
- DynamoDB o RDS únicamente si se demuestra una necesidad off-chain.
- CloudWatch y AWS Budgets desde el primer despliegue.

**Presupuesto educativo:** USD 0 de infraestructura objetivo mientras se use local/Testnet y la cuenta tenga créditos/free tier elegibles. AWS no es “gratis por ser estudiante” automáticamente: la elegibilidad, duración, límites y servicios dependen de la cuenta. Se debe configurar una alerta de presupuesto de USD 5 y un límite mensual de USD 20; cualquier cifra real se anotará desde Billing después de medir.

Terraform describirá solo los recursos auxiliares de AWS. Kubernetes, Nginx y una base de datos administrada quedan fuera del MVP porque añaden coste y superficie operativa sin resolver una regla de votación.

## Plan por semanas y entregables

| Semana | Trabajo | Evidencia de terminado |
| --- | --- | --- |
| 1 | Rust/Soroban hello-world, TypeScript SDK, Quickstart local y elección de wallet | Spike reproducible y decisión ADR |
| 2 | Modelo de dominio y contrato mínimo | Tests Rust de estados, autorización y unicidad |
| 3 | Cliente TypeScript y flujo crear/abrir/votar/cerrar | Demo local de extremo a extremo |
| 4 | Verificador independiente, eventos y medición de recursos | Informe de `simulateTransaction` |
| 5 | API Python opcional, 2FA administrativo y revisión de amenazas | Datos sintéticos, secretos fuera del repo |
| 6 | Testnet, accesibilidad, documentación y despliegue auxiliar | Demo reproducible, matriz de aceptación y coste observado |

## Criterios de aceptación del MVP

- Un participante autorizado solo puede votar una vez.
- No se puede votar antes de abrir ni después de cerrar.
- Un tercero puede reconstruir el resultado usando el contrato y sus eventos.
- Las pruebas se ejecutan sin claves reales ni datos personales.
- La guía distingue claramente red local, Testnet y cualquier servicio AWS.
- El despliegue se puede destruir sin dejar recursos costosos activos.

## Fuentes técnicas consultadas

- [Stellar CLI y contenedores locales](https://developers.stellar.org/docs/tools/cli/stellar-cli)
- [Modos local, Testnet y Futurenet](https://developers.stellar.org/docs/tools/quickstart/network-modes)
- [Medición de tarifas y recursos Soroban](https://developers.stellar.org/docs/learn/fundamentals/fees-resource-limits-metering)
- [AWS Free Tier y créditos](https://aws.amazon.com/free/free-tier-faqs/)
- [Alcance de Amazon Managed Blockchain](https://docs.aws.amazon.com/managed-blockchain/latest/ethereum-dev/managed-blockchain-ethereum-overview.html)
