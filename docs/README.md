# Documentación de Consensus

Esta carpeta contiene la especificación completa de la fase de definición del proyecto.

## Principios

1. Una afirmación pública necesita una fuente.
2. Un escenario creado por el equipo debe etiquetarse como sintético.
3. Una decisión pendiente no se presenta como tecnología seleccionada.
4. Una prueba planificada no se presenta como ejecutada.
5. La documentación debe poder revisarse sin conocer el contexto académico del equipo.
6. El proyecto educativo se gestiona con estándares de producto real.

## Estados documentales

| Estado | Significado |
| --- | --- |
| Borrador | El contenido está en elaboración. |
| Definido | El equipo completó el análisis y puede revisarlo. |
| Confirmado | Existe una decisión o evidencia suficiente. |
| En implementación | El contenido guía trabajo técnico activo. |
| Verificado | Los criterios fueron ejecutados y existe evidencia. |
| Reemplazado | Existe una versión posterior enlazada. |

## Tipos de evidencia

| Tipo | Identificador | Uso |
| --- | --- | --- |
| Fuente externa | `SRC-###` | Estándar, investigación o documentación pública. |
| Escenario sintético | `SCN-###` | Caso creado por el equipo para diseñar y probar. |
| Hipótesis | `H-###` | Resultado esperado que requiere evaluación. |
| Supuesto | `S-###` | Condición utilizada para acotar el diseño. |
| Decisión | `DEC-###` | Elección confirmada por el equipo. |
| Decisión abierta | `OPEN-###` | Elección pendiente con criterios definidos. |
| Requisito funcional | `RF-###` | Comportamiento que el sistema debe ofrecer. |
| Requisito no funcional | `RNF-###` | Propiedad de calidad o restricción. |
| Prueba de aceptación | `PA-###` | Procedimiento para verificar un requisito. |
| Riesgo | `R-###` | Evento que puede afectar el proyecto. |

## Fuente de verdad

- El README resume.
- Los documentos numerados contienen la especificación.
- `docs/research/` conserva investigación y escenarios.
- `docs/templates/` contiene formatos reutilizables.
- Las decisiones se consolidan en [13-decision-log.md](13-decision-log.md).
- La relación entre artefactos se mantiene en [15-traceability.md](15-traceability.md).

## Orden de lectura

1. [Problema](01-problem-brief.md)
2. [Personas y actores](02-people-and-actors.md)
3. [Flujo actual](03-current-value-flow.md)
4. [Oportunidad](04-opportunity-and-hypothesis.md)
5. [Evaluación de blockchain](05-blockchain-fit.md)
6. [Alcance](06-mvp-scope.md)
7. [Plan de evaluación](07-validation-plan.md)
8. [Enfoque técnico](08-technical-approach.md)
9. [Plan de ejecución](09-delivery-plan.md)
10. [Riesgos](10-risks-and-roadmap.md)
11. [Requisitos](11-requirements.md)
12. [Pruebas](12-acceptance-test-plan.md)
13. [Decisiones](13-decision-log.md)
14. [Glosario](14-glossary.md)
15. [Trazabilidad](15-traceability.md)
16. [Plan técnico de implementación](16-implementation-plan.md)

## Reglas de actualización

- Todo cambio funcional actualiza requisitos, pruebas y trazabilidad.
- Toda selección tecnológica actualiza el registro de decisiones.
- Toda prueba ejecutada incluye fecha, versión, responsable y evidencia.
- Los datos personales y las claves nunca se almacenan en el repositorio.
- Las fuentes externas se citan con título, entidad, URL y fecha de consulta.
