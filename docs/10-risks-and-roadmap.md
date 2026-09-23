# 10. Riesgos y hoja de ruta

**Estado:** registro inicial  
**Última revisión:** septiembre de 2026

## Riesgos

| ID | Riesgo | Impacto | Mitigación inicial |
| --- | --- | --- | --- |
| R-001 | Curva de aprendizaje de Rust y Soroban. | Alto | Empezar con ejemplos oficiales, reducir funciones y probar desde la semana 1. |
| R-002 | La billetera impide que usuarios completen el flujo. | Alto | Guía breve, acompañamiento y medición de cada paso. |
| R-003 | El voto seudónimo permite relacionar dirección y opción. | Alto | Piloto simulado, advertencia explícita y ausencia de identidades reales. |
| R-004 | La habilitación de direcciones no evita por sí sola suplantación. | Medio | Verificación externa controlada para el piloto y documentación de la limitación. |
| R-005 | El administrador configura incorrectamente la elección. | Medio | Validaciones, estados irreversibles documentados y revisión previa a apertura. |
| R-006 | Reinicio de Stellar Testnet elimina datos del piloto. | Medio | Scripts y pasos reproducibles; redesplegar antes de la demostración. |
| R-007 | Expiración o archivo de datos del contrato. | Medio | Diseñar la duración de datos y probar su mantenimiento antes del piloto. |
| R-008 | El equipo amplía el alcance durante las cinco semanas. | Alto | Respetar el documento de alcance y registrar cualquier cambio. |
| R-009 | La evidencia pública no es comprensible para personas no técnicas. | Medio | Traducir estados y transacciones a explicaciones claras en la interfaz. |
| R-010 | El problema no resulta importante para los usuarios. | Alto | Entrevistas tempranas y criterio explícito para detener o pivotar. |

## Limitaciones aceptadas del MVP

- La elección no es secreta.
- La identidad se valida fuera de la cadena.
- Existe una cuenta administradora.
- La red utilizada es de pruebas.
- Solo se demuestra una elección a la vez.
- La experiencia con billetera puede requerir acompañamiento.

## Hoja de ruta posterior

El orden depende de la validación; no constituye un compromiso de construcción.

1. Esquema de voto secreto, comenzando por evaluar commit-reveal.
2. Administración multifirma.
3. Integración con identidad institucional.
4. Experiencia sin extensiones mediante passkeys o cuentas inteligentes.
5. Soporte a múltiples elecciones y organizaciones.
6. Evaluación de pruebas de conocimiento cero o encriptación homomórfica.
7. Despliegue en Mainnet después de auditoría, modelo de gobernanza y revisión legal.

## Condiciones antes de una elección real

No debe utilizarse Consensus en una elección vinculante hasta contar con:

- Privacidad adecuada al contexto.
- Auditoría independiente del contrato.
- Gestión segura de identidades y claves.
- Plan de incidentes y recuperación.
- Revisión institucional y legal.
- Pruebas de accesibilidad y usabilidad.
- Gobernanza sobre actualizaciones y administración.
