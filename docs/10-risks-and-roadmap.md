# 10. Riesgos y evolución

**Estado:** definido  
**Última revisión:** septiembre de 2026

## Riesgos

| ID | Riesgo | Probabilidad | Impacto | Mitigación |
| --- | --- | --- | --- | --- |
| R-001 | Elegir arquitectura antes de realizar pruebas técnicas. | Media | Alto | Ejecutar EXP-001 y EXP-002 antes de desarrollar. |
| R-002 | La opción con contrato exige aprender Rust. | Alta si se elige contrato | Alto | Spike acotado, ejemplos oficiales y alcance pequeño. |
| R-003 | Elegir una alternativa comunitaria sin suficiente madurez. | Media | Alto | Evaluar mantenimiento, pruebas y soporte. |
| R-004 | Confundir evidencia documental con validación de usuarios reales. | Media | Medio | Etiquetar fuentes y escenarios sintéticos. |
| R-005 | Presentar el prototipo como apto para elecciones reales. | Media | Alto | Advertencias visibles y datos exclusivamente ficticios. |
| R-006 | Exponer la elección del votante. | Alta en el MVP | Alto | No usar procesos sensibles; documentar que no existe voto secreto. |
| R-007 | Doble voto o autorización incorrecta. | Media | Alto | Pruebas automatizadas y de aceptación. |
| R-008 | Pérdida o exposición de claves. | Media | Alto | Cuentas de prueba, secretos fuera del repositorio y rotación. |
| R-009 | Reinicio de Testnet. | Media | Medio | Automatizar preparación y despliegue. |
| R-010 | Framework frontend elegido por preferencia y no por requisitos. | Media | Medio | Matriz de decisión y prueba de integración. |
| R-011 | Dependencia excesiva de una billetera. | Media | Medio | Abstracción `WalletGateway`. |
| R-012 | Crecimiento de alcance. | Alta | Alto | Requisitos prioritarios y control de cambios. |
| R-013 | Resultado difícil de verificar para terceros. | Media | Alto | Guía independiente y EXP-004. |
| R-014 | Documentación desactualizada. | Media | Medio | Trazabilidad obligatoria en PR. |
| R-015 | Tratar AMB como si fuera una red Stellar o asumir gratuidad estudiantil. | Media | Alto | Usar AWS solo para auxiliares, Budgets y verificación de créditos antes de desplegar. |
| R-016 | Biometría o cifrado homomórfico amplían el alcance y exponen datos sensibles. | Alta | Alto | Datos sintéticos en MVP; investigación separada y revisión legal/privacidad. |
| R-017 | Kubernetes/Nginx añaden complejidad sin beneficio medido. | Alta | Medio | Mantener despliegue estático/serverless hasta tener métricas de carga. |

## Limitaciones aceptadas

- Proyecto educativo.
- Datos ficticios.
- Red de pruebas.
- Una elección.
- Sin voto secreto.
- Sin identidad real.
- Sin validez jurídica.
- Sin garantías de producción.

## Hoja de ruta

### Fase 1 — MVP educativo

- Flujo completo.
- Evidencia Stellar.
- Reglas básicas.
- Pruebas.
- Documentación reproducible.

### Fase 2 — Privacidad y gobernanza

- Evaluar commit-reveal.
- Evaluar cifrado o pruebas de conocimiento cero.
- Administración multifirma.
- Separación de identidad y voto.

### Fase 3 — Producto comunitario

- Múltiples elecciones.
- Organizaciones.
- Roles.
- Configuración reutilizable.
- Experiencia de incorporación.

### Fase 4 — Evaluación para producción

Solo se considera después de:

- Auditoría externa.
- Modelo de amenazas completo.
- Revisión legal.
- Accesibilidad.
- Gestión de identidad.
- Plan de incidentes.
- Gobernanza de actualizaciones.
