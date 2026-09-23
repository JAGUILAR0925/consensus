# 12. Plan de pruebas de aceptación

**Estado:** definido; ejecución pendiente de implementación

| ID | Escenario | Requisitos | Resultado esperado |
| --- | --- | --- | --- |
| PA-001 | Crear elección válida | RF-001 | Se crea en Configuración con identificador consultable. |
| PA-002 | Abrir configuración incompleta | RF-004 | La transición es rechazada. |
| PA-003 | Abrir elección válida | RF-002, RF-003, RF-005 | El estado cambia a Abierta. |
| PA-004 | Voto válido | RF-006, RF-007 | Se acepta, genera evidencia y aumenta una opción. |
| PA-005 | Votante no habilitado | RF-008 | Se rechaza sin modificar conteo. |
| PA-006 | Segundo voto | RF-009 | Se rechaza sin modificar conteo. |
| PA-007 | Cerrar elección | RF-010 | El estado cambia a Cerrada. |
| PA-008 | Votar antes de abrir | RF-011 | Se rechaza. |
| PA-009 | Votar después de cerrar | RF-011 | Se rechaza. |
| PA-010 | Consultar resultado | RF-012, RF-013 | Estado y conteo coinciden con operaciones aceptadas. |
| PA-011 | Verificación independiente | RF-014, RF-015 | Otra persona reproduce el resultado. |
| PA-012 | Recuperar escenario | RF-017 | El entorno se recrea desde cero. |
| PA-013 | Red incorrecta | RNF-007 | Se bloquea la operación y se explica el error. |
| PA-014 | Revisión de secretos | RNF-001 | No se encuentran secretos en repositorio ni logs. |
| PA-015 | Revisión de privacidad | RNF-002, RNF-012 | Solo hay datos ficticios y advertencia visible. |
| PA-016 | Navegación básica | RNF-006 | Los flujos críticos funcionan con teclado y texto. |
| PA-017 | Acción administrativa no autorizada | RF-001, RF-002, RF-003, RF-005, RF-010, AUT-001 | Una identidad no administradora intenta crear o modificar la elección, registrar opciones y participantes, abrirla y cerrarla; cada intento se rechaza sin alterar la configuración, el estado ni el conteo. |

## Niveles

### Unitarias

- Estados.
- Elegibilidad.
- Autorización.
- Unicidad.
- Conteo.
- Errores.

### Integración

- TypeScript con Stellar.
- Firma.
- Simulación.
- Envío.
- Confirmación.
- Consulta.

### Extremo a extremo

- Configuración.
- Apertura.
- Voto.
- Cierre.
- Resultado.
- Verificación.

## Evidencia requerida

Para cada ejecución:

- Commit.
- Fecha.
- Entorno.
- Datos ficticios.
- Resultado.
- Log o captura.
- Identificador de red cuando aplique.
- Responsable.
- Incidencia asociada si falla.

## Criterio de salida

El MVP no se declara completado hasta aprobar PA-001 a PA-017 con la evidencia requerida. Cualquier prueba fallida o bloqueada impide declarar el MVP terminado.
