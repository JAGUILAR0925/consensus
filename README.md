# Consensus

Plataforma de votaciones comunitarias verificables construida sobre Stellar y Soroban.

## La propuesta

Muchas comunidades, universidades, equipos, fundaciones, colectivos y organizaciones necesitan tomar decisiones mediante votaciones. Sin embargo, normalmente deben confiar en una sola persona o plataforma para registrar los votos, realizar el conteo, publicar los resultados y conservar el historial.

**Consensus** propone un sistema reutilizable de votaciones en el que las reglas, los votos y los resultados relevantes puedan verificarse de forma independiente. La meta no es utilizar blockchain por moda, sino aplicarla donde aporte transparencia, trazabilidad e integridad al proceso de decisión.

## Problema

En una votación administrada de forma centralizada:

- La comunidad depende de quien controla el sistema.
- Los participantes no pueden comprobar fácilmente que su voto fue incluido.
- El historial y los resultados podrían modificarse sin dejar evidencia.
- La confianza se concentra en un intermediario.
- Diferentes organizaciones terminan construyendo soluciones aisladas para necesidades similares.

## Solución propuesta

Consensus permitirá crear y participar en votaciones configurables para distintos tipos de comunidades. Cada proceso tendrá reglas claras, un periodo definido y resultados verificables.

La blockchain conservará únicamente la información que necesite ser pública y verificable. Los datos personales, perfiles, comunicaciones y demás información sensible permanecerán fuera de la cadena.

## Usuarios potenciales

- Comunidades y colectivos.
- Universidades y grupos estudiantiles.
- Fundaciones y organizaciones sociales.
- Equipos de trabajo.
- Eventos y comunidades digitales.
- Organizaciones autónomas descentralizadas (DAO).

## MVP

La primera versión busca demostrar el flujo principal:

1. Crear una votación con título, opciones y fechas.
2. Definir qué cuentas o wallets pueden participar.
3. Conectar una wallet compatible con Stellar.
4. Registrar un único voto por wallet.
5. Cerrar la votación según las reglas establecidas.
6. Consultar el resultado y su evidencia verificable en Stellar Testnet.

## Uso de blockchain

Consensus plantea utilizar:

- **Stellar** como red blockchain.
- **Soroban** para ejecutar contratos inteligentes.
- **Rust/Wasm** para implementar las reglas del contrato.
- **Stellar Testnet** durante el desarrollo y la validación inicial.

El contrato inteligente será responsable de aplicar reglas como la vigencia de la votación, la elegibilidad de una wallet y la prevención de votos duplicados.

## Información on-chain y off-chain

### On-chain

- Identificador de la votación.
- Opciones o referencias verificables.
- Fechas y estado del proceso.
- Registro necesario para validar que una wallet votó.
- Conteo o evidencia del resultado.

### Off-chain

- Datos personales.
- Perfiles y autenticación convencional.
- Descripciones extensas y contenido multimedia.
- Notificaciones.
- Información privada de las organizaciones.
- Analítica que no necesite consenso público.

## Principios del proyecto

- Transparencia verificable.
- Privacidad desde el diseño.
- Reutilización en diferentes comunidades.
- Reglas simples y comprensibles.
- Código abierto.
- Uso justificado de blockchain.
- Desarrollo incremental y validación temprana.

## Estado

El proyecto se encuentra en etapa inicial de definición y validación. Este repositorio comenzará con la propuesta, las decisiones del equipo y la evidencia del avance. La arquitectura y el código se incorporarán cuando el alcance del MVP esté suficientemente claro.

## Colaboración

Las contribuciones se realizan mediante ramas y pull requests. Consulta [CONTRIBUTING.md](CONTRIBUTING.md) antes de comenzar una tarea.

## Equipo

Proyecto desarrollado de forma colaborativa como parte de un proceso de aprendizaje y construcción de soluciones con blockchain.
