# 15. Matriz de trazabilidad

**Estado:** definida

| Problema o fricción | Hipótesis | Requisitos | Pruebas | Riesgos |
| --- | --- | --- | --- | --- |
| FRC-001 Dependencia del mismo sistema | H-001 | RF-014, RF-015, RNF-005 | PA-011 | R-013 |
| FRC-002 Reglas no verificables | H-003, H-004 | RF-004 a RF-011, RNF-009 | PA-002 a PA-009 | R-007 |
| Cambios administrativos no autorizados | H-003, H-004 | RF-001 a RF-003, RF-005, RF-010, AUT-001 | PA-017 | R-007 |
| FRC-003 Evidencia no reproducible | H-005 | RF-012 a RF-015, RNF-003 | PA-010, PA-011 | R-013 |
| FRC-004 Riesgo de privacidad | S-001, S-003 | RNF-002, RNF-012 | PA-015 | R-005, R-006 |
| FRC-005 Fallos del cliente | H-002, H-006 | RF-016, RNF-007, RNF-010 | PA-013 | R-011 |
| Reinicio de red | S-004 | RF-017, RNF-003 | PA-012 | R-009 |
| Tecnología sin decidir | S-005, S-006 | RNF-008, RNF-009 | EXP-001, EXP-002 | R-001, R-002, R-003, R-010 |

## Cobertura

- Todos los requisitos Must tienen una prueba asociada.
- Toda decisión abierta tiene criterios de evaluación.
- Todo riesgo alto tiene mitigación.
- Toda afirmación de investigación tiene una fuente.
- Todo escenario creado por el equipo se etiqueta como sintético.

## Mantenimiento

La matriz debe actualizarse cuando:

- Se agrega o elimina un requisito.
- Cambia el alcance.
- Se selecciona arquitectura.
- Se ejecuta una prueba.
- Aparece un riesgo.
- Se reemplaza una fuente.
