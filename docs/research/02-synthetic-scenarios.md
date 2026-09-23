# Escenarios sintéticos

**Estado:** definido  
**Propósito:** diseño, discusión y pruebas

> Todos los nombres, cuentas, organizaciones y resultados son ficticios.

## SCN-101 — Elección base

Una asociación estudiantil simula la elección de un representante entre tres opciones. Doce cuentas de Testnet están habilitadas. La elección comienza en Configuración, pasa a Abierta y termina Cerrada.

**Resultado esperado:**

- Solo las cuentas habilitadas participan.
- Cada cuenta genera como máximo un voto aceptado.
- El conteo puede verificarse.
- No se publican identidades reales.

## SCN-102 — Segundo voto

Una cuenta que ya votó intenta enviar otra elección.

**Resultado esperado:** la operación no modifica el conteo y devuelve un error identificable.

## SCN-103 — Participante no habilitado

Una cuenta válida de Stellar que no aparece en el escenario intenta votar.

**Resultado esperado:** el voto es rechazado.

## SCN-104 — Elección cerrada

Un participante intenta votar después del cierre.

**Resultado esperado:** no cambia el estado ni el conteo.

## SCN-105 — Configuración incompleta

El administrador intenta abrir sin opciones o sin participantes.

**Resultado esperado:** el sistema impide la transición.

## SCN-106 — Acción administrativa no autorizada

Un participante intenta registrar opciones o cerrar la elección.

**Resultado esperado:** la operación es rechazada.

## SCN-107 — Operación enviada pero no confirmada

La aplicación construye una operación, pero la red no la confirma.

**Resultado esperado:** la interfaz no presenta el voto como aceptado y permite consultar el estado.

## SCN-108 — Red incorrecta

La aplicación está configurada para una red distinta a la cuenta.

**Resultado esperado:** se bloquea el flujo y se explica la red requerida.

## SCN-109 — Reinicio de Testnet

Los datos de prueba dejan de estar disponibles.

**Resultado esperado:** el equipo puede recrear cuentas, despliegue y escenario siguiendo una guía.

## SCN-110 — Verificación externa

Un integrante que no desarrolló la funcionalidad recibe únicamente la URL, los identificadores y la guía.

**Resultado esperado:** reproduce estado y conteo sin acceder a almacenamiento privado.

## SCN-111 — Privacidad

Un observador relaciona una cuenta de prueba con su elección.

**Resultado esperado:** el equipo reconoce que el MVP no ofrece secreto del voto y no utiliza el sistema con datos reales.

## SCN-112 — Comparación centralizada

El equipo implementa o describe el mismo conteo con una estructura TypeScript local.

**Resultado esperado:** documenta diferencias de confianza, complejidad, evidencia y mantenimiento sin declarar un ganador de antemano.

## Uso

Estos escenarios alimentan:

- Requisitos.
- Casos de prueba.
- Demostración.
- Modelo de amenazas.
- Decisiones técnicas.
