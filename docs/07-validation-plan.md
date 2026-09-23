# 07. Plan de validación

**Estado:** pendiente de ejecución  
**Última revisión:** septiembre de 2026

## Objetivo

Validar primero el problema y después la solución. Una demostración técnica exitosa no demuestra que el producto sea necesario o usable.

## Fase 1: entrevistas de problema

### Muestra mínima

- Cinco estudiantes.
- Un candidato o excandidato.
- Una persona que haya organizado o acompañado una elección.

### Preguntas orientadoras

Las entrevistas deben explorar experiencias pasadas, no vender la solución.

- Cuéntame cómo fue la última votación en la que participaste.
- ¿Cómo supiste que tu voto había sido recibido?
- ¿Cómo se comunicó el resultado?
- ¿Tuviste alguna duda sobre el conteo?
- ¿Qué hiciste o qué habrías hecho para resolverla?
- ¿Qué información necesitarías para confiar en el resultado?
- ¿Qué información de tu voto consideras privada?
- ¿Has usado alguna vez una billetera digital?

### Evitar

- “¿Usarías una app con blockchain?”
- “¿Te parece buena esta idea?”
- Explicar la solución antes de entender la experiencia.
- Registrar nombres, documentos o información innecesaria.

## Fase 2: prueba del prototipo

Probar con al menos cinco personas:

- Comprensión de las instrucciones.
- Conexión de la billetera.
- Selección y firma.
- Confirmación del registro.
- Interpretación del resultado y su evidencia.

## Fase 3: piloto

Ejecutar una votación simulada con diez a quince participantes.

### Métricas

| Métrica | Cómo se calcula |
| --- | --- |
| Finalización sin ayuda | Participantes que completan el voto sin asistencia / participantes que lo intentan. |
| Tiempo de voto | Tiempo desde abrir la aplicación hasta confirmar la transacción. |
| Abandono | Intentos que no llegan a confirmación / intentos iniciados. |
| Errores por paso | Cantidad de errores en conexión, firma, envío o confirmación. |
| Comprensión | Participantes que explican correctamente qué pudieron verificar. |
| Cambio de confianza | Diferencia entre una pregunta breve antes y después del piloto. |

No se fijan metas numéricas hasta obtener una línea base.

## Registro de evidencia

Cada hallazgo debe utilizar la [plantilla de validación](templates/validation-record.md) y asignar un identificador `EV-###`.

La evidencia debe incluir:

- Fecha.
- Método.
- Perfil anonimizado.
- Observación literal o resultado medido.
- Hipótesis relacionada.
- Interpretación del equipo.
- Decisión o siguiente experimento.

## Criterios de decisión

- **Continuar:** el problema aparece de manera consistente y la solución mejora verificabilidad sin una fricción desproporcionada.
- **Ajustar:** el problema existe, pero billetera, privacidad o comprensión bloquean el flujo.
- **Detener o pivotar:** el problema no es relevante o una alternativa centralizada auditada lo resuelve mejor.
