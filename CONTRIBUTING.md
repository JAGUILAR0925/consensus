# Guía de contribución

Consensus es un proyecto educativo desarrollado con prácticas de ingeniería reproducibles. Cada contribución debe ser comprensible, revisable y trazable.

## Flujo

1. Selecciona o crea un issue.
2. Relaciona requisitos, pruebas o decisiones afectadas.
3. Crea una rama desde `main`.
4. Implementa un cambio acotado.
5. Ejecuta las validaciones aplicables.
6. Actualiza documentación y trazabilidad.
7. Abre un pull request.
8. Obtén al menos una revisión.
9. Resuelve comentarios antes de fusionar.

No se realizan cambios directos en `main`.

## Ramas

- `feature/...`
- `fix/...`
- `docs/...`
- `test/...`
- `research/...`
- `chore/...`

## Commits

Se utiliza Conventional Commits:

- `feat:`
- `fix:`
- `docs:`
- `test:`
- `refactor:`
- `chore:`
- `research:`

Cada commit representa una unidad coherente y debe poder atribuirse a su autor.

## Issues

Un issue debe incluir:

- Problema o resultado.
- Alcance.
- Criterios de aceptación.
- Identificadores relacionados.
- Dependencias.
- Responsable.
- Riesgos conocidos.

## Decisiones

Una tecnología no se incorpora porque sea popular o conocida. Las decisiones que afecten arquitectura, seguridad, privacidad, Stellar, frontend, firma, almacenamiento o despliegue requieren:

1. Issue de decisión.
2. Opciones.
3. Criterios.
4. Evidencia o spike.
5. ADR.
6. Revisión del equipo.
7. Actualización de `docs/13-decision-log.md`.

## Investigación

Las fuentes públicas deben indicar título, entidad, URL y fecha. Los ejemplos creados por el equipo se etiquetan como sintéticos. No se inventan entrevistas, usuarios ni resultados.

## Pull requests

Todo PR debe:

- Resolver un objetivo.
- Enlazar su issue.
- Identificar requisitos y pruebas afectadas.
- Incluir instrucciones de validación.
- Adjuntar evidencia.
- Actualizar documentación.
- Mantener secretos y datos personales fuera del repositorio.
- Estar actualizado con `main`.

## Seguridad

Está prohibido publicar:

- Claves privadas.
- Frases de recuperación.
- Tokens.
- Credenciales.
- Datos personales.
- Correspondencias entre personas reales y direcciones.
- Votos reales.

## Definición de terminado

Una tarea está terminada cuando:

- Cumple criterios.
- Tiene pruebas.
- Fue revisada.
- Actualiza trazabilidad.
- Documenta limitaciones.
- No introduce secretos.
- Puede reproducirse por otra persona.
