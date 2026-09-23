# 13. Registro de decisiones

**Estado:** actualizado a septiembre de 2026

## Decisiones confirmadas

| ID | Decisión | Motivo |
| --- | --- | --- |
| DEC-001 | Utilizar Stellar. | Es el objeto técnico principal del proyecto educativo. |
| DEC-002 | Utilizar TypeScript en aplicación e integración. | Conocimiento del equipo y SDK disponible. |
| DEC-003 | Trabajar inicialmente en una red de pruebas. | Evitar activos y datos reales. |
| DEC-004 | Utilizar exclusivamente escenarios e identidades ficticias. | Privacidad y alcance educativo. |
| DEC-005 | Equipo de cinco integrantes. | Composición real del equipo. |
| DEC-006 | Una elección activa en el MVP. | Control de alcance. |
| DEC-007 | El MVP no ofrece voto secreto. | La privacidad avanzada queda fuera del horizonte inicial. |
| DEC-008 | El repositorio y la documentación son públicos. | Aprendizaje, revisión y trazabilidad. |

## Decisiones abiertas

### OPEN-001 — Modelo de ejecución de reglas

**Opciones:** operaciones nativas de Stellar o contrato inteligente.

**Criterios:** expresividad, pruebas, seguridad, aprendizaje, tiempo y verificabilidad.

**Evidencia:** EXP-001 y EXP-002.

### OPEN-002 — Lenguaje del contrato

Solo aplica si se selecciona contrato.

**Opciones consideradas:** Rust oficial o alternativa comunitaria compatible.

**Restricción:** TypeScript permanece como lenguaje de cliente; no se presenta como SDK oficial de contratos.

### OPEN-003 — Framework frontend

**Criterios:** TypeScript, experiencia, integración, accesibilidad, pruebas y despliegue.

### OPEN-004 — Estrategia de firma

**Opciones:** billetera, kit de billeteras o cuentas controladas para demostración.

**Criterios:** seguridad, reproducibilidad y facilidad educativa.

### OPEN-005 — Backend

**Regla:** no incorporarlo hasta que un requisito lo justifique.

### OPEN-006 — Hosting y CI

**Criterios:** costo, simplicidad, seguridad y reproducibilidad.

## Proceso

1. Crear un ADR desde la plantilla.
2. Documentar opciones.
3. Ejecutar la prueba necesaria.
4. Revisar en equipo.
5. Aprobar mediante pull request.
6. Actualizar esta tabla y la trazabilidad.
