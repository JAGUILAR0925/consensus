# 08. Enfoque técnico

**Estado:** restricciones confirmadas; arquitectura en evaluación  
**Última revisión:** septiembre de 2026

## Tecnologías confirmadas

| Área | Decisión |
| --- | --- |
| Red | Stellar |
| Lenguaje de aplicación e integración | TypeScript |
| Entorno inicial | Testnet o entorno equivalente de pruebas |
| Datos | Exclusivamente ficticios |
| Repositorio | Repositorio público |

## Arquitectura lógica

La solución se divide en responsabilidades, independientemente del framework:

1. **Interfaz:** presenta configuración, votación, estado y resultados.
2. **Aplicación TypeScript:** coordina casos de uso.
3. **Adaptador Stellar:** construye, firma, envía y consulta operaciones.
4. **Reglas de votación:** aplica autorización, unicidad y estados.
5. **Verificación:** obtiene evidencia sin depender de la vista principal.
6. **Pruebas:** valida reglas y flujos.
7. **Despliegue:** reproduce cuentas, configuración y datos de prueba.

## Decisión abierta principal — OPEN-001

### ¿Dónde se ejecutarán las reglas?

#### Opción A: contrato inteligente

Ventajas:

- Reglas compartidas y ejecutadas en la red.
- Estado consultable.
- Pruebas específicas del contrato.
- Mejor demostración del modelo descentralizado.

Consideraciones:

- La documentación oficial de Stellar indica que actualmente Rust es el lenguaje soportado oficialmente para contratos.
- TypeScript seguiría utilizándose como cliente e integración.
- Introduce una curva adicional de aprendizaje.

#### Opción B: operaciones nativas y lógica TypeScript

Ventajas:

- Menor cantidad de tecnologías.
- Aprendizaje inicial más accesible.
- Permite demostrar firmas, transacciones e historial.

Consideraciones:

- Parte de las reglas se ejecutaría fuera de la red.
- La prevención de doble voto y el conteo requieren un diseño distinto.
- Puede ofrecer menor valor para el objetivo de reglas verificables.

## Lenguaje de contrato

Rust **no está seleccionado todavía**.

Si el equipo decide construir un contrato, debe considerar que:

- El SDK de contratos mantenido oficialmente por Stellar es Rust.
- Existe un SDK comunitario en AssemblyScript.
- TypeScript cuenta con SDK oficial para interactuar con la red y con contratos, pero no es el lenguaje oficial para escribirlos.

La prueba EXP-002 resolverá esta decisión.

## Frontend

El framework permanece abierto. Criterios:

- Compatibilidad con TypeScript.
- Experiencia del equipo.
- Integración con el SDK de Stellar.
- Simplicidad de despliegue.
- Pruebas.
- Accesibilidad.
- Tamaño del MVP.

No se presupone React, Vue, Nuxt, Next ni otro framework.

## Billetera y firma

La estrategia permanece abierta:

- Extensión de billetera.
- Kit de billeteras.
- Cuentas de prueba controladas.
- Flujo asistido para demostración.

La selección debe priorizar seguridad, reproducibilidad y claridad educativa.

## Backend y almacenamiento off-chain

No están confirmados. Solo se incorporarán si un requisito no puede resolverse razonablemente con la aplicación cliente y Stellar.

## Estructura del repositorio

La elección entre monorepo, workspace u otra organización permanece abierta y se resolverá después de seleccionar la arquitectura. La documentación no presupone una estructura de código.

## Interfaces conceptuales TypeScript

La implementación debe aislar Stellar detrás de contratos propios:

- `ElectionRepository`
- `WalletGateway`
- `TransactionGateway`
- `EvidenceReader`
- `Clock`

Esto evita acoplar los casos de uso al framework o proveedor de billetera.

## Seguridad mínima

- Nunca almacenar claves privadas en el repositorio.
- Separar cuentas administrativas y votantes.
- Validar red y passphrase.
- Verificar simulación y estado de transacción.
- Prevenir repetición.
- Registrar errores sin secretos.
- Limitar datos publicados.
- Hacer reproducible el entorno.
- Revisar dependencias antes de incorporarlas.

## Fuentes oficiales

- [Stellar SDKs](https://developers.stellar.org/docs/tools/sdks)
- [Aplicaciones con y sin contratos](https://developers.stellar.org/docs/build)
- [Smart contracts en Stellar](https://developers.stellar.org/docs/build/smart-contracts/overview)
- [SDKs de contratos](https://developers.stellar.org/docs/tools/sdks/build-your-own)
- [Guía frontend e integración TypeScript](https://developers.stellar.org/docs/build/guides/dapps/frontend-guide)
