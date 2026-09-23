# 06. Alcance del MVP

**Estado:** definido  
**Última revisión:** septiembre de 2026  
**Horizonte:** cinco semanas  
**Equipo:** cinco integrantes

## Objetivo

Construir un prototipo educativo que ejecute una elección simulada sobre Stellar y permita verificar sus operaciones y resultado.

## Alcance funcional

| ID | Capacidad |
| --- | --- |
| MVP-001 | Crear un escenario de elección con título, opciones y estado inicial. |
| MVP-002 | Registrar identificadores ficticios de participantes habilitados. |
| MVP-003 | Abrir la elección después de validar su configuración. |
| MVP-004 | Permitir que un participante autorizado emita un voto. |
| MVP-005 | Rechazar un segundo voto del mismo participante. |
| MVP-006 | Rechazar votos cuando la elección no está abierta. |
| MVP-007 | Cerrar la elección de forma controlada. |
| MVP-008 | Consultar conteo y estado. |
| MVP-009 | Mostrar identificadores de evidencia en Stellar. |
| MVP-010 | Ejecutar un conjunto reproducible de pruebas. |
| MVP-011 | Documentar el despliegue y reinicio del escenario. |
| MVP-012 | Comparar el resultado con una implementación centralizada conceptual. |

## Restricciones confirmadas

- Stellar es la red seleccionada.
- TypeScript se utiliza en la aplicación y la integración.
- El entorno inicial no utiliza activos reales.
- Solo se emplean identidades y datos ficticios.
- El sistema no garantiza voto secreto.
- Solo se requiere una elección activa.
- El MVP es educativo y no vinculante.

## Decisiones técnicas no incluidas todavía

El alcance no obliga a elegir:

- Rust.
- Un framework frontend específico.
- Una billetera específica.
- Un backend.
- Una base de datos.
- Un servicio de hosting.
- Un explorador de red específico.

Estas decisiones están controladas en [13-decision-log.md](13-decision-log.md).

## Fuera del alcance

- Elecciones oficiales.
- Identidad institucional real.
- Mainnet.
- Voto secreto o resistente a coerción.
- Aplicaciones móviles nativas.
- Múltiples organizaciones.
- Recuperación de cuentas.
- Escalabilidad masiva.
- Integraciones gubernamentales.
- Auditoría de seguridad externa.
- Cumplimiento normativo para producción.

## Datos permitidos

- Nombres ficticios de candidatos.
- Direcciones o cuentas creadas para Testnet.
- Identificadores de transacciones de prueba.
- Métricas técnicas sin datos personales.

## Datos prohibidos

- Nombres o identificaciones reales.
- Correos y teléfonos.
- Información académica real.
- Claves privadas.
- Frases de recuperación.
- Correspondencia entre una persona y una dirección.
- Votos de procesos reales.

## Definición de terminado

El MVP se considera completado cuando:

- Todos los requisitos prioritarios están implementados.
- Las pruebas de aceptación PA-001 a PA-017 están aprobadas con evidencia; ninguna permanece fallida o bloqueada.
- Existe un despliegue reproducible en la red de pruebas.
- Un observador puede verificar el resultado siguiendo la documentación.
- El equipo presenta limitaciones y amenazas sin ocultarlas.
- La decisión de arquitectura está registrada con evidencia.
