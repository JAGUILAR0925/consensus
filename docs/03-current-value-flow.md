# 03. Flujo de valor

**Estado:** definido como modelo sintético  
**Última revisión:** septiembre de 2026

> Este flujo es un modelo creado por el equipo para analizar el problema. No describe una institución específica.

## Flujo centralizado de referencia — SCN-006

| Paso | Responsable | Acción | Evidencia disponible para terceros |
| --- | --- | --- | --- |
| 1. Configuración | Organizador | Define reglas, opciones y fechas. | Documento o interfaz controlada por el organizador. |
| 2. Habilitación | Organizador | Registra quién puede participar. | Lista privada o acceso restringido. |
| 3. Emisión | Votante | Envía su elección. | Confirmación del mismo sistema. |
| 4. Almacenamiento | Plataforma | Guarda el voto. | Base de datos no accesible. |
| 5. Conteo | Plataforma u organizador | Calcula totales. | Resultado publicado. |
| 6. Revisión | Candidato u observador | Solicita explicaciones o evidencia. | Depende del acceso concedido. |
| 7. Archivo | Organizador | Conserva el historial. | Copias internas o acta final. |

## Fricciones

| ID | Fricción | Impacto |
| --- | --- | --- |
| FRC-001 | La confirmación y el conteo dependen del mismo sistema. | El usuario no tiene verificación independiente. |
| FRC-002 | Las reglas pueden no estar vinculadas técnicamente al conteo. | Es difícil comprobar que se aplicaron de forma uniforme. |
| FRC-003 | La evidencia puede no ser pública o reproducible. | La auditoría requiere permisos. |
| FRC-004 | La identidad y el voto pueden almacenarse juntos. | Aumenta el riesgo de privacidad. |
| FRC-005 | Los errores del cliente o dispositivo pueden pasar inadvertidos. | La cadena no garantiza que la intención se capture correctamente. |

## Flujo objetivo del MVP

| Paso | Acción | Evidencia esperada |
| --- | --- | --- |
| 1. Configurar | Crear opciones, participantes y reglas del escenario. | Estado inicial verificable. |
| 2. Revisar | Confirmar la configuración antes de abrir. | Resumen reproducible. |
| 3. Abrir | Cambiar el estado de la elección. | Operación registrada en Stellar. |
| 4. Autorizar | Comprobar que el participante está habilitado. | Regla ejecutada consistentemente. |
| 5. Votar | Firmar y enviar una operación. | Identificador y resultado de la operación. |
| 6. Cerrar | Impedir votos posteriores. | Cambio de estado verificable. |
| 7. Contar | Consultar el resultado. | Estado o evidencia pública. |
| 8. Auditar | Repetir consultas y pruebas. | Procedimiento documentado. |

## Valor transferido

El proyecto traslada parte de la capacidad de verificación desde el operador hacia reglas y evidencia que otros actores pueden consultar.

No elimina al administrador: la habilitación inicial y la preparación del escenario siguen requiriendo responsabilidades claramente asignadas.
