# Sistema documental

Este directorio mantiene separadas las distintas capas de información del proyecto. El objetivo es evitar que una idea, una hipótesis y una decisión técnica se presenten como si tuvieran el mismo nivel de certeza.

## Orden de lectura

1. Problema.
2. Persona y actores.
3. Recorrido actual y flujo de valor.
4. Fricción seleccionada.
5. Oportunidad e hipótesis.
6. Supuestos y validación.
7. Justificación de blockchain.
8. Alcance del MVP.
9. Enfoque técnico.
10. Entrega, riesgos y hoja de ruta.

La solución se define después de entender el problema, no antes.

## Tipos de información

| Tipo | Identificador | Significado |
| --- | --- | --- |
| Hecho | `F-###` | Observación respaldada por evidencia consultable. |
| Hipótesis | `H-###` | Afirmación que el equipo debe comprobar. |
| Supuesto | `S-###` | Condición necesaria para que la propuesta funcione. |
| Decisión | `ADR-###` | Elección acordada, con contexto y consecuencias. |
| Evidencia | `EV-###` | Entrevista, prueba, métrica, transacción o documento verificable. |
| Riesgo | `R-###` | Evento incierto que podría afectar el resultado. |

No se convierte una hipótesis en hecho hasta enlazar evidencia suficiente.

## Estructura mínima de cada documento

Cada documento debe indicar:

- Estado: borrador, en validación, aceptado o reemplazado.
- Objetivo.
- Información conocida.
- Hipótesis o decisiones relacionadas.
- Evidencia disponible.
- Preguntas abiertas.
- Próximo paso.

## Reglas de mantenimiento

- El README funciona como portada, no como documento exhaustivo.
- Los detalles viven en `docs/`.
- Las decisiones técnicas importantes se discuten en un issue antes de implementarse.
- La evidencia no se reescribe: se referencia y se conserva.
- Los datos personales se anonimizan.
- Un cambio de alcance debe actualizar el documento afectado y explicar el motivo en el pull request.
- Si dos documentos se contradicen, prevalece el que tenga estado aceptado y fecha de revisión más reciente.

## Plantillas

- [Registro de decisión](templates/decision-record.md).
- [Registro de validación](templates/validation-record.md).
