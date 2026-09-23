# 05. Justificación de blockchain

**Estado:** justificación provisional  
**Última revisión:** septiembre de 2026

## Pregunta de decisión

¿Consensus necesita una blockchain o podría resolver el problema adecuadamente con una base de datos centralizada y un proceso de auditoría?

## Las tres señales

| Señal de la sesión 1 | Presencia en el caso piloto | Observación |
| --- | --- | --- |
| Varias partes que no confían plenamente entre sí necesitan el mismo registro. | Parcial | Comité, candidatos, votantes y observadores necesitan una versión común del proceso. Debe validarse el nivel real de desconfianza. |
| El histórico no debe alterarse ni siquiera por quien administra. | Sí, como requisito propuesto | Reglas, apertura, cierre y conteo deberían dejar evidencia permanente. |
| Existe un intermediario principalmente para concentrar la confianza. | Parcial | El comité seguirá siendo necesario para validar identidades, aunque no tendría que ser la única fuente del conteo. |

## Comparación de alternativas

| Criterio | Formulario convencional | Base central con auditoría | Stellar y contrato inteligente |
| --- | --- | --- | --- |
| Facilidad inicial | Alta | Media | Media o baja |
| Verificación pública | Limitada | Posible con acceso o publicación adicional | Nativa para los datos registrados |
| Modificación unilateral | Posible por el operador | Mitigable con controles y auditoría | Restringida por las reglas del contrato |
| Identidad del votante | Correo o sistema institucional | Sistema institucional | Sigue requiriendo un mecanismo externo de habilitación |
| Privacidad | Controlada por el operador | Configurable | El MVP es seudónimo, no secreto |
| Complejidad técnica | Baja | Media | Alta |
| Costo del piloto | Bajo | Medio | Sin costo de red en Testnet, pero con mayor costo de desarrollo |
| Continuidad sin la interfaz original | Baja | Depende del diseño | El estado puede consultarse desde otros clientes |

## Aporte esperado

Blockchain podría aportar:

- Un registro compartido de las acciones relevantes.
- Reglas ejecutadas de la misma forma para todos.
- Evidencia de apertura, voto, cierre y conteo.
- Menor dependencia de la base de datos del organizador.
- Posibilidad de verificar el resultado con herramientas distintas a la interfaz oficial.

## Lo que blockchain no resuelve

- Comprobar que una dirección corresponde a una persona autorizada.
- Evitar que una persona entregue o venda su acceso.
- Recuperar una clave perdida.
- Garantizar voto secreto en el diseño inicial.
- Evitar una mala configuración del administrador.
- Hacer que la experiencia sea automáticamente sencilla.
- Otorgar validez legal o institucional al resultado.

## Decisión provisional

Se utilizará Stellar Testnet y un contrato inteligente para construir una **prueba funcional de verificabilidad**, no para afirmar que el sistema está listo para elecciones oficiales.

La decisión se revisará después de las entrevistas y del piloto. Si una base central auditada ofrece un resultado suficiente con menor fricción, el equipo deberá documentarlo honestamente.

## Fuentes técnicas oficiales

- [Smart contracts en Stellar](https://developers.stellar.org/docs/build/smart-contracts/overview)
- [Escribir, probar y desplegar un contrato en Rust](https://developers.stellar.org/docs/build/smart-contracts/getting-started/hello-world)
- [Despliegue en Testnet](https://developers.stellar.org/docs/build/smart-contracts/getting-started/deploy-to-testnet)
- [Redes de Stellar](https://developers.stellar.org/docs/networks)
