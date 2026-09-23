# Investigación documental

**Estado:** completada para la fase de definición  
**Fecha de consulta:** septiembre de 2026

## Objetivo

Identificar problemas, propiedades y limitaciones relevantes para un prototipo educativo de votación verificable.

## Fuentes

### SRC-001 — National Academies of Sciences, Engineering, and Medicine

- **Documento:** _Securing the Vote: Protecting American Democracy_
- **Año:** 2018
- **Enlace:** https://www.nationalacademies.org/read/25120/chapter/7
- **Hallazgo:** los registros, votos, conteos y evidencias pueden perderse, alterarse o reportarse incorrectamente; los sistemas electrónicos también enfrentan interrupciones y malware.
- **Aplicación:** Consensus debe preservar evidencia, definir amenazas y evitar afirmar que la red protege el dispositivo del usuario.

### SRC-002 — U.S. Election Assistance Commission

- **Documento:** _Voluntary Voting System Guidelines 2.0_
- **Enlace:** https://www.eac.gov/voting-equipment/voluntary-voting-system-guidelines
- **Hallazgo:** un sistema de votación serio debe evaluarse en funcionalidad, accesibilidad y seguridad.
- **Aplicación:** el MVP incluye requisitos no funcionales y pruebas más allá del conteo.

### SRC-003 — Benaloh, Rivest, Ryan, Stark, Teague y Vora

- **Documento:** _End-to-end verifiability_
- **Año:** 2015
- **Enlace:** https://arxiv.org/abs/1504.03778
- **Hallazgo:** la verificabilidad de extremo a extremo permite comprobar elementos importantes del resultado sin depender únicamente de software, hardware o autoridades.
- **Aplicación:** Consensus separa “transacción registrada” de “sistema electoral seguro” y define verificación independiente.

### SRC-004 — Consejo de Europa

- **Documento:** _Recommendation CM/Rec(2017)5 on standards for e-voting_
- **Año:** 2017
- **Enlace:** https://search.coe.int/cm?i=090000168071bc84
- **Hallazgo:** la votación electrónica debe preservar principios democráticos y estándares operativos, legales y técnicos.
- **Aplicación:** el proyecto se limita a una simulación educativa y no reclama validez electoral.

### SRC-005 — Liu y Wang

- **Documento:** _A Systematic Review of Challenges and Opportunities of Blockchain for E-Voting_
- **Año:** 2020
- **Enlace:** https://www.mdpi.com/2073-8994/12/8/1328
- **Hallazgo:** blockchain puede contribuir a integridad y verificabilidad, pero permanecen desafíos de privacidad, consenso y diseño general.
- **Aplicación:** el MVP declara explícitamente que no ofrece voto secreto.

### SRC-006 — Khan et al.

- **Documento:** _A Systematic Literature Review and Meta-Analysis on Scalable Blockchain-Based Electronic Voting Systems_
- **Año:** 2022
- **Enlace:** https://pmc.ncbi.nlm.nih.gov/articles/PMC9572428/
- **Hallazgo:** autenticación, privacidad, integridad, transparencia, verificabilidad y escalabilidad deben abordarse simultáneamente.
- **Aplicación:** Consensus no reduce el problema a almacenar votos en una cadena.

### SRC-007 — Stellar Development Foundation

- **Documento:** _Build on Stellar_
- **Enlace:** https://developers.stellar.org/docs/build
- **Hallazgo:** una aplicación puede construirse con o sin contratos inteligentes.
- **Aplicación:** el proyecto comparará ambas rutas.

### SRC-008 — Stellar Development Foundation

- **Documento:** _Smart Contracts Overview_
- **Enlace:** https://developers.stellar.org/docs/build/smart-contracts/overview
- **Hallazgo:** el soporte oficial actual para contratos utiliza Rust.
- **Aplicación:** Rust solo se seleccionará si el equipo decide utilizar un contrato.

### SRC-009 — Stellar Development Foundation

- **Documento:** _SDKs_
- **Enlace:** https://developers.stellar.org/docs/tools/sdks
- **Hallazgo:** los SDK de cliente permiten integrar aplicaciones mediante JavaScript y otros lenguajes.
- **Aplicación:** TypeScript es adecuado para la capa de aplicación e integración.

### SRC-010 — Stellar Development Foundation

- **Documento:** _Comprehensive frontend guide for Stellar dapps_
- **Enlace:** https://developers.stellar.org/docs/build/guides/dapps/frontend-guide
- **Hallazgo:** el SDK permite construir transacciones, consultar RPC y generar bindings TypeScript.
- **Aplicación:** la integración debe aislarse del framework frontend.

## Síntesis

1. La verificabilidad es un problema reconocido.
2. La evidencia pública no sustituye privacidad ni identidad.
3. Blockchain puede aportar trazabilidad, pero aumenta complejidad.
4. El MVP debe usar datos ficticios.
5. El proyecto debe comparar arquitectura con y sin contrato.
6. La seguridad debe evaluarse mediante requisitos y pruebas explícitas.
7. TypeScript es válido para la aplicación; el lenguaje de contrato es una decisión separada.

## Limitaciones de esta investigación

- No incluye trabajo de campo.
- No demuestra demanda de mercado.
- No valida usabilidad.
- Las normas citadas se orientan a sistemas electorales más exigentes que este MVP.
- Las conclusiones se utilizan como marco educativo, no como certificación.
