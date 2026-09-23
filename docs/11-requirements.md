# 11. Requisitos

**Estado:** definido  
**Prioridad:** MoSCoW

## Requisitos funcionales

| ID | Requisito | Prioridad |
| --- | --- | --- |
| RF-001 | El sistema debe permitir al administrador crear una elección de prueba con identificador, título y estado. | Must |
| RF-002 | Debe permitir al administrador registrar al menos dos opciones ficticias. | Must |
| RF-003 | Debe permitir al administrador registrar participantes ficticios habilitados. | Must |
| RF-004 | Debe impedir abrir una elección sin configuración válida. | Must |
| RF-005 | Debe permitir al administrador la transición Configuración → Abierta y rechazarla para otros actores. | Must |
| RF-006 | Debe autorizar la operación del participante. | Must |
| RF-007 | Debe aceptar un voto válido durante el estado Abierta. | Must |
| RF-008 | Debe rechazar participantes no habilitados. | Must |
| RF-009 | Debe impedir un segundo voto aceptado de la misma identidad de prueba. | Must |
| RF-010 | Debe permitir al administrador la transición Abierta → Cerrada y rechazarla para otros actores. | Must |
| RF-011 | Debe rechazar votos fuera del estado Abierta. | Must |
| RF-012 | Debe consultar el estado actual. | Must |
| RF-013 | Debe consultar el conteo por opción. | Must |
| RF-014 | Debe mostrar un identificador de evidencia asociado a operaciones confirmadas. | Must |
| RF-015 | Debe permitir verificar el resultado sin acceso administrativo. | Must |
| RF-016 | Debe mostrar errores comprensibles. | Should |
| RF-017 | Debe ofrecer un reinicio reproducible del escenario. | Should |
| RF-018 | Debe exportar un resumen de la demostración. | Could |

## Requisitos no funcionales

| ID | Requisito | Criterio |
| --- | --- | --- |
| RNF-001 | Seguridad de secretos | Ninguna clave privada o frase aparece en código, commits o logs. |
| RNF-002 | Privacidad | Solo se utilizan datos e identidades ficticias. |
| RNF-003 | Reproducibilidad | Una segunda persona puede preparar el entorno con la documentación. |
| RNF-004 | Trazabilidad | Cada requisito Must está relacionado con una prueba. |
| RNF-005 | Auditabilidad | Un observador consulta estado y resultado sin permisos de administrador. |
| RNF-006 | Accesibilidad básica | Flujos operables con teclado, etiquetas y mensajes textuales. |
| RNF-007 | Compatibilidad | La aplicación declara y valida la red utilizada. |
| RNF-008 | Mantenibilidad | La integración Stellar se desacopla del framework de interfaz. |
| RNF-009 | Testabilidad | Las reglas principales pueden probarse automáticamente. |
| RNF-010 | Observabilidad | Los errores conservan contexto sin revelar secretos. |
| RNF-011 | Rendimiento educativo | El flujo responde dentro de tiempos razonables de Testnet documentados. |
| RNF-012 | Honestidad del producto | La interfaz indica que es un prototipo educativo sin voto secreto. |

## Reglas de dominio

- EST-001: Configuración solo puede pasar a Abierta.
- EST-002: Abierta solo puede pasar a Cerrada.
- EST-003: Cerrada es final para el escenario.
- AUT-001: solo el administrador ejecuta configuración y transiciones; las solicitudes de otros actores se rechazan sin cambiar la configuración ni el estado.
- AUT-002: cada participante autoriza su voto.
- VOT-001: solo votan participantes habilitados.
- VOT-002: solo se acepta un voto por participante.
- VOT-003: la opción debe existir.
- VOT-004: el conteo cambia únicamente por un voto aceptado.
