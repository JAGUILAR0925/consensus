# 01. Problem brief

**Estado:** definido  
**Última revisión:** septiembre de 2026  
**Caso de estudio:** elección simulada de representantes estudiantiles

## Problema

En una votación digital centralizada, los participantes pueden depender de la misma entidad para habilitar votantes, recibir votos, calcular resultados y presentar evidencia. Cuando el sistema no ofrece mecanismos de verificación independientes, el resultado exige confianza en el operador y en su infraestructura.

Consensus estudia cómo proporcionar evidencia verificable sin afirmar que blockchain resuelve por sí sola todos los riesgos electorales.

## Evidencia pública

- **SRC-001 — National Academies, _Securing the Vote_ (2018):** identifica riesgos de alteración, pérdida o reporte incorrecto de votos y evidencia electoral, además de ataques contra sistemas electrónicos.
- **SRC-002 — U.S. Election Assistance Commission, VVSG 2.0:** evalúa sistemas de votación mediante requisitos de funcionalidad, accesibilidad y seguridad.
- **SRC-003 — Benaloh et al., _End-to-end verifiability_ (2015):** explica que la verificabilidad de extremo a extremo permite comprobar elementos esenciales del resultado sin confiar únicamente en software, hardware o funcionarios.
- **SRC-004 — Consejo de Europa, estándares de voto electrónico (2017):** establece principios para procesos electrónicos compatibles con elecciones democráticas.
- **SRC-005 — Revisiones sistemáticas de votación blockchain:** identifican privacidad, autenticación, integridad, escalabilidad y verificabilidad como problemas simultáneos; blockchain no elimina estas tensiones.

Las fuentes completas y su interpretación están en [research/01-desk-research.md](research/01-desk-research.md).

## Persona afectada

Para el caso de estudio, la persona principal es un estudiante habilitado que quiere:

- Emitir un voto válido.
- Saber que fue registrado.
- Comprender el resultado.
- No revelar información personal innecesaria.
- No depender de conocimientos técnicos avanzados.

## Fricción seleccionada

El participante no dispone de evidencia independiente y comprensible para comprobar que el proceso siguió sus reglas y que el resultado corresponde a los votos aceptados.

## Límites del problema

Consensus no pretende resolver en el MVP:

- Identidad legal o institucional.
- Coerción o compra de votos.
- Seguridad completa del dispositivo del usuario.
- Recuperación de credenciales.
- Validez jurídica.
- Voto secreto criptográficamente robusto.
- Disponibilidad nacional o gran escala.

## Pregunta de investigación

¿Cómo puede un equipo construir sobre Stellar un prototipo educativo que permita registrar y verificar reglas y resultados de una votación simulada, manteniendo visibles sus limitaciones de privacidad, identidad y seguridad?

## Resultado esperado

Una implementación reproducible que:

- Demuestre qué aporta un registro compartido.
- Permita verificar las reglas principales.
- Exponga claramente lo que no resuelve.
- Produzca evidencia técnica revisable por terceros.
