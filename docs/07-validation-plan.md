# 07. Plan de investigación y evaluación

**Estado:** protocolo definido  
**Última revisión:** septiembre de 2026

## Enfoque

El proyecto no utilizará entrevistas inventadas. La fase de definición se apoya en:

1. Investigación documental.
2. Escenarios sintéticos.
3. Experimentos técnicos reproducibles.
4. Evaluaciones internas controladas.
5. Evidencia generada por el sistema durante la implementación.

## Trabajo completado

- Revisión de estándares y literatura sobre integridad y verificabilidad.
- Identificación de amenazas de sistemas electrónicos.
- Modelado de cinco actores sintéticos.
- Creación de un flujo centralizado de referencia.
- Delimitación de propiedades que blockchain no resuelve.
- Definición de hipótesis y pruebas necesarias.

## Evaluaciones a ejecutar

### EXP-001 — Integración básica con Stellar

**Objetivo:** demostrar que TypeScript puede construir, firmar, enviar y consultar una operación de prueba.

**Resultado esperado:** identificador verificable y script reproducible.

### EXP-002 — Comparación de arquitectura

**Objetivo:** comparar operaciones nativas contra contrato inteligente.

**Criterios:**

- Capacidad de expresar reglas.
- Pruebas.
- Seguridad.
- Complejidad.
- Tiempo de aprendizaje.
- Integración con TypeScript.
- Mantenimiento.

### EXP-003 — Reglas de votación

**Objetivo:** demostrar autorización, unicidad y control de estados.

**Resultado esperado:** pruebas automatizadas y evidencia en Testnet.

### EXP-004 — Verificación independiente

**Objetivo:** que un integrante distinto al autor reproduzca el resultado.

**Resultado esperado:** registro de pasos, tiempo, problemas y conclusión.

### EXP-005 — Comprensión de interfaz

**Objetivo:** evaluar si los cinco integrantes interpretan correctamente estado, confirmación y evidencia.

**Resultado esperado:** lista de ajustes de contenido y experiencia.

## Métricas

| Métrica | Cálculo |
| --- | --- |
| Cobertura de aceptación | Pruebas aprobadas / pruebas definidas |
| Reproducibilidad | Ejecuciones exitosas por una segunda persona |
| Tiempo de verificación | Minutos necesarios para reproducir un resultado |
| Errores de integración | Fallos por firma, envío, simulación o consulta |
| Comprensión | Respuestas correctas sobre qué demuestra la evidencia |
| Cumplimiento documental | Artefactos actualizados / artefactos afectados |

## Evidencia

Cada experimento genera un registro con:

- ID.
- Fecha.
- Versión o commit.
- Responsable.
- Entorno.
- Datos ficticios utilizados.
- Pasos.
- Resultado.
- Evidencia.
- Limitaciones.
- Decisión derivada.

Se utiliza [templates/validation-record.md](templates/validation-record.md).

## Interpretación responsable

- Un experimento exitoso demuestra únicamente su criterio.
- Una transacción confirmada no demuestra privacidad.
- Un doble voto rechazado no demuestra identidad única.
- Un resultado reproducible no convierte el MVP en sistema electoral real.
