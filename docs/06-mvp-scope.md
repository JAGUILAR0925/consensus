# 06. Alcance del MVP

**Estado:** propuesta acotada  
**Última revisión:** septiembre de 2026  
**Duración objetivo:** cinco semanas  
**Equipo:** cinco integrantes

## Objetivo del MVP

Demostrar que un grupo puede ejecutar una votación simulada y verificar sus reglas, votos y resultado mediante un contrato desplegado en Stellar Testnet.

El MVP valida una capacidad, no constituye un producto electoral listo para producción.

## Caso piloto

Una elección simulada de representantes estudiantiles con:

- Una elección activa.
- Máximo diez candidatos.
- Entre diez y quince participantes durante el piloto.
- Direcciones habilitadas previamente.
- Una persona facilitadora que actúa como administradora.

## Dentro del alcance

| ID | Capacidad | Criterio resumido |
| --- | --- | --- |
| MVP-001 | Configurar elección | El administrador registra candidatos y votantes antes de abrir. |
| MVP-002 | Controlar estados | La elección pasa de Configuración a Abierta y luego a Cerrada. |
| MVP-003 | Conectar billetera | El usuario conecta Freighter en Stellar Testnet. |
| MVP-004 | Autorizar voto | Solo una dirección habilitada puede firmar su voto. |
| MVP-005 | Evitar duplicados | Un segundo voto de la misma dirección es rechazado. |
| MVP-006 | Consultar resultado | Cualquier visitante consulta el conteo sin autenticarse. |
| MVP-007 | Mostrar evidencia | La interfaz enlaza o identifica las transacciones relevantes. |
| MVP-008 | Administrar lo mínimo | Existe una interfaz mínima o, si el tiempo no alcanza, un flujo documentado con Stellar CLI. |
| MVP-009 | Probar reglas | Las restricciones críticas tienen pruebas automatizadas. |
| MVP-010 | Ejecutar piloto | Se recopilan métricas y retroalimentación sin datos sensibles. |

## Fuera del alcance

- Elecciones oficiales o jurídicamente vinculantes.
- Voto secreto.
- Mainnet.
- Integración con sistemas universitarios.
- Aplicación móvil.
- Múltiples elecciones simultáneas.
- Recuperación de cuentas o custodia de claves.
- Encriptación homomórfica o pruebas de conocimiento cero.
- Gobernanza multifirma.
- Backend propio como fuente de verdad.
- Analítica avanzada.

## Restricción de privacidad

En el MVP, la dirección que firma puede quedar relacionada con su elección. No se deben usar nombres reales ni ejecutar una votación sensible. La demostración debe informar esta limitación antes de participar.

## Criterio de recorte

Si el equipo no dispone del tiempo esperado, se recorta en este orden:

1. Panel visual de administración; se reemplaza por Stellar CLI.
2. Actualización en tiempo real; se utiliza actualización manual.
3. Elementos visuales no esenciales.
4. Métricas secundarias.

Nunca se recortan las reglas de autorización, prevención de doble voto, control de estados, pruebas ni advertencias de privacidad.

## Definición de terminado

El MVP está terminado cuando:

- Un administrador configura, abre y cierra la elección.
- Un votante habilitado vota una vez.
- Los intentos inválidos son rechazados.
- El resultado de la web coincide con el estado del contrato.
- Existe evidencia reproducible en Testnet.
- El piloto y sus aprendizajes están documentados.
