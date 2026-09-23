# 09. Plan de entrega

**Estado:** propuesta  
**Horizonte inicial:** cinco semanas  
**Equipo:** cinco integrantes

## Frentes de responsabilidad

| Frente | Responsabilidad principal |
| --- | --- |
| Producto e investigación | Descubrimiento, entrevistas, definición del problema, alcance, métricas y priorización. |
| Contratos inteligentes | Diseño del contrato Soroban, autorización, almacenamiento, pruebas y despliegue. |
| Integración blockchain | Billetera, SDK, bindings, transacciones, lectura de estado y herramientas de despliegue. |
| Frontend y experiencia | Interfaz de administración, votación, resultados, accesibilidad y manejo de errores. |
| Calidad y documentación | Estrategia de pruebas, validación funcional, riesgos, evidencia, documentación y demostración. |

Cada frente tiene una persona responsable, pero las decisiones críticas y las revisiones son compartidas. La asignación nominal se registrará cuando el equipo confirme responsables.

## Cronograma

| Semana | Objetivo | Trabajo | Resultado verificable |
| --- | --- | --- | --- |
| 1. Descubrimiento | Confirmar el problema y el contexto de uso. | Entrevistas, actores, recorrido, fricción y prueba técnica inicial. | Problem brief respaldado por evidencia y prueba técnica desplegada en Testnet. |
| 2. Diseño | Definir las reglas mínimas del MVP. | Alcance, flujos, criterios de aceptación, contrato v1 y pruebas unitarias. | Contrato desplegable, pruebas automatizadas y prototipo de interfaz. |
| 3. Construcción | Completar el flujo de voto de extremo a extremo. | Integración de billetera, votación, lectura de estado y resultados. | Transacción de prueba reproducible y flujo funcional documentado. |
| 4. Validación | Comprobar que personas reales pueden utilizarlo y comprenderlo. | Administración mínima, manejo de errores, prueba piloto y registro de métricas. | Informe del piloto y lista priorizada de problemas. |
| 5. Lanzamiento del MVP | Consolidar una versión demostrable y auditable. | Correcciones críticas, publicación, documentación técnica y hoja de ruta. | Repositorio, MVP publicado, demostración y plan de evolución. |

## Forma de trabajo

- Cada tarea parte de un issue con criterios de aceptación.
- Cada integrante trabaja en una rama propia o de funcionalidad.
- Cada cambio se integra mediante pull request.
- Al menos otra persona revisa antes de fusionar.
- Los commits permiten identificar la contribución de cada integrante.
- Los resultados técnicos y de producto se respaldan con evidencia reproducible.

## Hitos

1. **Definición del problema:** contexto, persona, recorrido, fricción, oportunidad, hipótesis y supuestos.
2. **Definición del producto:** alcance, flujos, reglas y criterios de aceptación.
3. **Prueba funcional:** contrato y flujo real en Testnet.
4. **MVP validado:** piloto, métricas y aprendizajes.
5. **Versión demostrable:** repositorio, despliegue, documentación y hoja de ruta.

## Criterio de avance

Un hito se considera completo cuando su resultado puede revisarse, reproducirse y relacionarse con evidencia. La cantidad de código no se utiliza como medida principal de progreso.
