# Consensus

Proyecto educativo de código abierto para diseñar y construir un sistema de votaciones comunitarias verificables sobre Stellar.

> **Estado actual:** definición de producto y base documental completadas. La implementación técnica aún no ha comenzado.

## Propósito

Consensus permite estudiar, mediante un caso práctico, cómo diseñar un sistema distribuido que registre reglas, participación y resultados de una votación de manera verificable.

El proyecto adopta estándares de trabajo propios de un producto real: evidencia trazable, requisitos identificados, decisiones registradas, criterios de aceptación, análisis de riesgos, revisión por pull request y separación entre información confirmada y decisiones pendientes.

Consensus no debe utilizarse en elecciones oficiales, vinculantes o que requieran voto secreto.

## Caso de estudio

El primer caso de estudio es una elección simulada de representantes estudiantiles. Se utiliza porque permite analizar claramente:

- Elegibilidad.
- Autorización.
- Unicidad del voto.
- Apertura y cierre de la elección.
- Conteo.
- Auditoría.
- Privacidad.
- Experiencia de usuario.

La plataforma se diseña para que el modelo pueda adaptarse posteriormente a asociaciones, comunidades, fundaciones, colectivos y equipos.

## Problema estudiado

En un proceso centralizado, la misma entidad puede administrar participantes, recibir votos, calcular el resultado y publicar la evidencia. Esto obliga a los demás actores a confiar en el operador cuando no existe un mecanismo independiente de verificación.

La investigación pública confirma que los sistemas electorales deben considerar integridad, auditabilidad, seguridad, accesibilidad y preservación de evidencia. También muestra que la tecnología distribuida no resuelve automáticamente identidad, privacidad, coerción o seguridad del dispositivo del votante.

Consulta el [problem brief](docs/01-problem-brief.md) y la [investigación documental](docs/research/01-desk-research.md).

## Objetivo educativo

Diseñar y construir un MVP reproducible que permita al equipo aprender y demostrar:

- Fundamentos de Stellar.
- Integración de aplicaciones mediante TypeScript.
- Diseño de transacciones y autorización.
- Evaluación de contratos inteligentes.
- Modelado de amenazas.
- Pruebas automatizadas.
- Trazabilidad de requisitos.
- Documentación y colaboración técnica.

## Decisiones confirmadas

| Área | Decisión |
| --- | --- |
| Red blockchain | Stellar |
| Lenguaje de aplicación e integración | TypeScript |
| Entorno inicial | Red de pruebas |
| Equipo | Cinco integrantes |
| Naturaleza | Proyecto educativo y open source |
| Caso inicial | Elección estudiantil simulada |
| Uso permitido | Aprendizaje, experimentación y demostración |

## Decisiones abiertas

Todavía deben evaluarse y registrarse:

- Uso de contrato inteligente o de operaciones nativas de Stellar.
- Lenguaje del contrato si se utiliza uno.
- Framework frontend.
- Estrategia de billetera y firma.
- Necesidad de servicios off-chain.
- Hosting y automatización de despliegue.

Estas decisiones no se presentan como cerradas. Consulta el [registro de decisiones](docs/13-decision-log.md).

## Alcance del MVP

El MVP deberá permitir:

1. Crear una elección simulada.
2. Registrar opciones y participantes habilitados.
3. Abrir y cerrar la votación.
4. Autorizar un voto por participante.
5. Consultar el resultado.
6. Verificar evidencia registrada en Stellar.
7. Ejecutar pruebas reproducibles de las reglas principales.

Consulta el [alcance detallado](docs/06-mvp-scope.md).

## Documentación

| Documento | Contenido |
| --- | --- |
| [Índice documental](docs/README.md) | Organización, estados y reglas de mantenimiento |
| [Problem brief](docs/01-problem-brief.md) | Problema, límites y evidencia |
| [Personas y actores](docs/02-people-and-actors.md) | Roles sintéticos y necesidades |
| [Flujo de valor](docs/03-current-value-flow.md) | Proceso base y fricciones |
| [Oportunidad e hipótesis](docs/04-opportunity-and-hypothesis.md) | Resultados esperados y supuestos |
| [Evaluación de blockchain](docs/05-blockchain-fit.md) | Utilidad y limitaciones |
| [Alcance del MVP](docs/06-mvp-scope.md) | Incluido, excluido y definición de terminado |
| [Plan de evaluación](docs/07-validation-plan.md) | Investigación y experimentos |
| [Enfoque técnico](docs/08-technical-approach.md) | Restricciones y alternativas arquitectónicas |
| [Plan de ejecución](docs/09-delivery-plan.md) | Organización del equipo |
| [Riesgos y evolución](docs/10-risks-and-roadmap.md) | Riesgos, mitigaciones y fases posteriores |
| [Requisitos](docs/11-requirements.md) | Requisitos funcionales y no funcionales |
| [Pruebas de aceptación](docs/12-acceptance-test-plan.md) | Casos verificables del MVP |
| [Decisiones](docs/13-decision-log.md) | Decisiones confirmadas y pendientes |
| [Glosario](docs/14-glossary.md) | Términos funcionales y técnicos |
| [Trazabilidad](docs/15-traceability.md) | Relación entre problema, requisitos y pruebas |

## Estado de completitud

- [x] Problema y límites definidos.
- [x] Investigación documental registrada.
- [x] Escenarios sintéticos documentados.
- [x] Personas y actores modelados.
- [x] Alcance del MVP definido.
- [x] Requisitos identificados.
- [x] Criterios y pruebas de aceptación definidos.
- [x] Riesgos y decisiones pendientes registrados.
- [x] Plan de trabajo para cinco integrantes definido.
- [ ] Implementación técnica.
- [ ] Pruebas ejecutadas.
- [ ] Despliegue del MVP.

## Equipo

Consensus es desarrollado por cinco integrantes. Las responsabilidades se distribuyen entre producto e investigación, Stellar e integración, experiencia frontend, calidad y seguridad, y documentación técnica.

Consulta [CONTRIBUTING.md](CONTRIBUTING.md) antes de realizar cambios.
