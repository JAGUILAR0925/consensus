# 09. Plan de ejecución

**Estado:** definido  
**Horizonte inicial:** cinco semanas  
**Equipo:** cinco integrantes

## Frentes

| Frente | Responsabilidad |
| --- | --- |
| Producto e investigación | Alcance, evidencia, casos, requisitos y trazabilidad. |
| Stellar e integración | Pruebas con la red, operaciones, SDK y herramientas. |
| Frontend y experiencia | Interfaz, accesibilidad, estados y mensajes. |
| Calidad y seguridad | Pruebas, amenazas, revisión y criterios de aceptación. |
| Documentación y entrega | Guías, decisiones, despliegue y demostración. |

Los frentes no fijan la tecnología pendiente. Cada integrante debe contribuir mediante commits y revisar trabajo de otra persona.

## Semana 1 — Fundamentos y decisión técnica

- Completar prueba TypeScript con Stellar.
- Comparar solución con y sin contrato.
- Resolver OPEN-001 sobre arquitectura y registrar la decisión con el siguiente identificador disponible.
- Seleccionar framework y estrategia de firma.
- Crear el esqueleto mínimo del repositorio.

**Salida:** pruebas técnicas, decisiones registradas y entorno reproducible.

## Semana 2 — Reglas y dominio

- Modelar elección, opciones, participantes y estados.
- Implementar las reglas según la arquitectura elegida.
- Crear pruebas de autorización y unicidad.
- Definir errores de dominio.

**Salida:** núcleo de reglas probado.

## Semana 3 — Flujo completo

- Integrar interfaz, firma, envío y consulta.
- Implementar configuración, apertura, voto, cierre y resultados.
- Asociar evidencia de Stellar a cada operación.

**Salida:** flujo funcional de extremo a extremo.

## Semana 4 — Calidad y verificación

- Ejecutar pruebas de aceptación.
- Revisar seguridad, privacidad y accesibilidad.
- Ejecutar verificación independiente.
- Corregir errores críticos.

**Salida:** informe de calidad y matriz de pruebas.

## Semana 5 — Publicación educativa

- Automatizar despliegue de prueba.
- Completar guías.
- Preparar datos ficticios.
- Ejecutar la demostración.
- Documentar resultados y decisiones posteriores.

**Salida:** MVP desplegado, evidencia, documentación y demostración reproducible.

## Flujo de trabajo

- Issue con alcance y criterios.
- Rama por cambio.
- Pull request.
- Revisión de una segunda persona.
- Pruebas antes de fusionar.
- Actualización de trazabilidad.
- Commits identificables por integrante.

## Control de alcance

Una funcionalidad entra al MVP solo si:

- Corresponde a un requisito prioritario.
- Tiene prueba de aceptación.
- Puede completarse dentro del horizonte.
- No introduce datos reales.
- No contradice una decisión confirmada.
