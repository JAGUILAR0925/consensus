# 05. Evaluación del uso de blockchain

**Estado:** definido  
**Última revisión:** septiembre de 2026

## Decisión de contexto

Consensus utilizará Stellar porque el objetivo educativo incluye aprender a diseñar, integrar y evaluar una solución sobre esta red.

Esta decisión no equivale a afirmar que blockchain sea la única o la mejor solución para todas las votaciones.

## Criterios de adecuación

| Criterio | Evaluación para el MVP |
| --- | --- |
| Varias partes necesitan consultar el mismo estado. | Sí: administrador, votantes, candidatos y observadores. |
| El historial debe ser difícil de modificar unilateralmente. | Sí: reglas, estados y resultados requieren trazabilidad. |
| La evidencia debe poder consultarse fuera de la aplicación oficial. | Sí: es uno de los objetivos educativos. |
| Existe valor en ejecutar reglas compartidas. | Potencial: debe demostrarse con el prototipo. |
| Los datos son apropiados para publicación. | Solo datos ficticios y no sensibles. |
| El beneficio compensa la complejidad. | Debe evaluarse al finalizar el MVP. |

## Comparación de enfoques

| Aspecto | Sistema centralizado | Stellar sin contrato personalizado | Stellar con contrato inteligente |
| --- | --- | --- | --- |
| Desarrollo inicial | Más simple | Intermedio | Más complejo |
| Reglas de negocio | Servidor o aplicación | Aplicación y operaciones nativas | Programa desplegado en la red |
| Auditoría externa | Requiere exponer datos | Operaciones visibles | Estado y reglas invocables |
| Prevención de doble voto | Base de datos | Diseño adicional | Regla programable |
| Actualización | Controlada por operador | Aplicación cliente | Requiere estrategia de contrato |
| Privacidad | Configurable | Los datos publicados son visibles | Los datos publicados son visibles |
| Valor educativo | Arquitectura tradicional | Integración Stellar | Integración, autorización y contratos |

## Aportes esperados de Stellar

- Registro compartido de operaciones.
- Firmas y autorización.
- Identificadores verificables.
- Consulta independiente del historial.
- Posibilidad de utilizar contratos inteligentes.
- Entorno de pruebas sin activos reales.

## Límites

Stellar o una blockchain no resuelven automáticamente:

- Identidad única por persona.
- Voto secreto.
- Coerción.
- Malware en el dispositivo.
- Pérdida de claves.
- Mala configuración.
- Accesibilidad.
- Comprensión de la evidencia.
- Validez legal.

## Conclusión

Stellar está justificado como plataforma del proyecto educativo. La arquitectura concreta seguirá comparando dos alternativas:

1. Solución basada principalmente en operaciones y datos de Stellar.
2. Solución con contrato inteligente para ejecutar las reglas.

La selección debe basarse en una prueba técnica, no en preferencia personal.

## Referencias

- [National Academies — Securing the Vote](https://www.nationalacademies.org/read/25120/chapter/7)
- [U.S. EAC — Voluntary Voting System Guidelines](https://www.eac.gov/voting-equipment/voluntary-voting-system-guidelines)
- [Benaloh et al. — End-to-end verifiability](https://arxiv.org/abs/1504.03778)
- [Stellar — Applications with and without smart contracts](https://developers.stellar.org/docs/build)
- [Stellar — Smart contracts overview](https://developers.stellar.org/docs/build/smart-contracts/overview)
