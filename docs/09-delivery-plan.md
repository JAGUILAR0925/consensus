# 09. Plan de entrega

**Estado:** propuesta  
**Duración:** cinco semanas  
**Equipo supuesto:** tres integrantes

## Roles iniciales

| Rol | Responsabilidad principal |
| --- | --- |
| Contrato e infraestructura blockchain | Contrato Soroban, pruebas, despliegue, bindings y soporte técnico. |
| Frontend e integración | Interfaz web, billetera, transacciones, resultados y experiencia de uso. |
| Producto, QA y comunicación | Entrevistas, documentación, pruebas manuales, piloto y presentación. |

Los roles organizan el trabajo, pero las decisiones, revisiones y aprendizajes son compartidos.

## Cronograma

| Semana | Pregunta principal | Trabajo | Evidencia de salida |
| --- | --- | --- | --- |
| 1. Identificar | ¿El problema existe y para quién? | Entrevistas, actores, recorrido, fricción, instalación de herramientas y prueba técnica mínima. | Problem brief con evidencia y contrato Hello World desplegado en Testnet. |
| 2. Diseñar | ¿Qué reglas mínimas necesita el experimento? | Alcance, flujos, criterios de aceptación, contrato v1 y pruebas unitarias. | Contrato desplegable, pruebas en verde y prototipo de interfaz. |
| 3. Construir | ¿Podemos completar un voto real de extremo a extremo? | Integración de billetera, votación, lectura de estado y resultados. | Transacción de prueba, hash verificable y video corto del flujo. |
| 4. Integrar | ¿Personas reales pueden usarlo y entenderlo? | Administración mínima, manejo de errores, piloto y registro de métricas. | Reporte del piloto y lista priorizada de problemas. |
| 5. Demostrar | ¿Qué aprendimos y qué podemos probar? | Correcciones críticas, publicación, documentación, demo y hoja de ruta. | Repositorio, demo, video de máximo tres minutos y presentación. |

## Forma de trabajo

- Cada tarea parte de un issue con criterios de aceptación.
- Cada integrante trabaja en su propia rama.
- Cada cambio se integra mediante pull request.
- Al menos otra persona revisa antes de fusionar.
- Los commits individuales deben permitir observar la contribución de cada integrante.
- Las demostraciones se respaldan con evidencia reproducible.

## Entregables acumulativos

1. **Problem brief:** problema, persona, recorrido, fricción, oportunidad, hipótesis y supuestos.
2. **Product blueprint:** alcance, flujos, reglas y criterios de aceptación.
3. **Functional proof:** contrato y flujo real en Testnet.
4. **Validated MVP:** piloto, métricas y aprendizajes.
5. **Demo package:** repositorio, presentación, video y hoja de ruta de treinta días.

## Definición de avance

Una semana no se considera completa únicamente por haber escrito código. Debe existir evidencia verificable del aprendizaje o resultado esperado.
