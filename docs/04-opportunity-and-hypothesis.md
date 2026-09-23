# 04. Oportunidad e hipótesis

**Estado:** definido  
**Última revisión:** septiembre de 2026

## Oportunidad

Construir un laboratorio educativo donde reglas, operaciones y resultados de una votación simulada puedan inspeccionarse y reproducirse, permitiendo comparar un enfoque centralizado con uno apoyado en Stellar.

## Hipótesis principal

**H-001:** registrar reglas y evidencia verificable en Stellar permite que un tercero compruebe aspectos esenciales del proceso sin acceder a una base de datos privada.

## Hipótesis secundarias

| ID | Hipótesis | Evidencia requerida |
| --- | --- | --- |
| H-002 | TypeScript permite construir la integración necesaria con Stellar. | Prueba técnica que construya, firme, envíe y consulte operaciones. |
| H-003 | El sistema puede impedir dos votos válidos del mismo participante. | Prueba automatizada y ejecución controlada. |
| H-004 | El estado Cerrada impide nuevos votos. | Prueba automatizada y evidencia en red de pruebas. |
| H-005 | Un observador puede reproducir el conteo. | Guía ejecutada por una persona distinta al autor. |
| H-006 | La interfaz puede explicar la evidencia sin exigir conocimientos profundos de blockchain. | Prueba de comprensión con el equipo. |
| H-007 | El enfoque elegido puede implementarse en cinco semanas por cinco integrantes. | Seguimiento de hitos y alcance. |

## Supuestos de diseño

| ID | Supuesto | Tratamiento |
| --- | --- | --- |
| S-001 | El piloto usa identidades y datos ficticios. | Obligatorio. |
| S-002 | Solo existe una elección activa en el MVP. | Limita complejidad. |
| S-003 | El voto no es secreto en la primera versión. | Se informa antes de usar. |
| S-004 | La red de pruebas puede reiniciarse. | El despliegue debe ser reproducible. |
| S-005 | El equipo conoce TypeScript, pero no ha elegido framework. | La lógica de integración debe ser independiente del framework. |
| S-006 | La tecnología de reglas on-chain sigue en evaluación. | Se resuelve mediante una prueba comparativa. |

## Criterio de éxito educativo

El proyecto es exitoso si el equipo puede:

- Explicar el modelo de confianza.
- Ejecutar el flujo de extremo a extremo.
- Mostrar evidencia verificable.
- Reproducir las pruebas.
- Identificar con precisión qué propiedades no están garantizadas.
- Justificar cada decisión tecnológica.

## Criterio de fracaso útil

También se considera aprendizaje válido demostrar que una alternativa más simple resuelve mejor una parte del problema. El objetivo educativo no obliga a presentar blockchain como superior cuando la evidencia no lo sostiene.
