# Consensus

Plataforma open source para crear decisiones y votaciones comunitarias verificables con Stellar y Soroban.

> **Estado:** descubrimiento y validación del problema. Aún no es un sistema apto para elecciones reales.

## Visión

Consensus busca que una comunidad pueda comprobar las reglas, la participación y el resultado de una votación sin depender únicamente de la organización que la administra.

El producto se plantea como una solución reutilizable para comunidades, universidades, fundaciones, colectivos, equipos y organizaciones. El primer caso piloto será una elección simulada de representantes estudiantiles.

## Primera hipótesis de producto

Si las reglas y el conteo de una votación quedan registrados en un contrato inteligente, entonces participantes, candidatos y observadores podrán verificar el resultado de manera independiente y aumentará su confianza en el proceso.

Esta hipótesis todavía debe validarse con usuarios. Blockchain es una posible solución, no el punto de partida del problema.

## Alcance inicial

El MVP propone una elección:

- Ejecutada en Stellar Testnet.
- Administrada mediante un contrato inteligente en Soroban.
- Con votantes habilitados previamente.
- Con un voto por dirección.
- Con resultados y transacciones verificables.
- Sin voto secreto en la primera versión.

Consulta el [alcance completo del MVP](docs/06-mvp-scope.md).

## Documentación del proyecto

La información se organiza siguiendo el método trabajado en la sesión 1: problema, persona, recorrido, fricción, oportunidad, hipótesis y supuestos.

| Documento | Pregunta que responde |
| --- | --- |
| [Sistema documental](docs/README.md) | ¿Cómo organizamos hechos, hipótesis, decisiones y evidencia? |
| [Problem brief](docs/01-problem-brief.md) | ¿Qué problema queremos validar? |
| [Personas y actores](docs/02-people-and-actors.md) | ¿Quién sufre el problema y quién participa? |
| [Recorrido y flujo de valor](docs/03-current-value-flow.md) | ¿Cómo ocurre hoy y dónde se pierde valor? |
| [Oportunidad e hipótesis](docs/04-opportunity-and-hypothesis.md) | ¿Qué cambio esperamos producir? |
| [Justificación de blockchain](docs/05-blockchain-fit.md) | ¿Por qué blockchain podría ser adecuada? |
| [Alcance del MVP](docs/06-mvp-scope.md) | ¿Qué se construirá y qué quedará fuera? |
| [Plan de validación](docs/07-validation-plan.md) | ¿Cómo comprobaremos los supuestos? |
| [Enfoque técnico](docs/08-technical-approach.md) | ¿Cómo podría implementarse el MVP? |
| [Plan de entrega](docs/09-delivery-plan.md) | ¿Qué entregaremos durante las cinco semanas? |
| [Riesgos y hoja de ruta](docs/10-risks-and-roadmap.md) | ¿Qué puede fallar y qué viene después? |

## Seguridad y privacidad

La primera versión ofrece verificabilidad, pero **no garantiza voto secreto**: una elección on-chain puede relacionar una dirección pública con una opción. Por esta razón, el piloto debe ser simulado o no vinculante hasta implementar y validar un mecanismo de privacidad adecuado.

Nunca deben publicarse nombres, documentos de identidad, correos, claves privadas o frases de recuperación.

## Colaboración

Revisa [CONTRIBUTING.md](CONTRIBUTING.md) antes de trabajar. Cada cambio debe partir de un issue, desarrollarse en una rama y revisarse mediante pull request.

## Tecnología propuesta

- Stellar Testnet.
- Contratos inteligentes de Stellar (Soroban) con Rust.
- Stellar CLI y pruebas del SDK de Soroban.
- Aplicación web en React, Vite y TypeScript.
- Freighter para firmar transacciones.

Las decisiones técnicas siguen siendo provisionales hasta quedar registradas y aceptadas por el equipo.

## Equipo

Proyecto desarrollado por un equipo de tres integrantes durante un programa de construcción blockchain de cinco semanas.
