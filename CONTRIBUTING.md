# Guía de contribución

Gracias por contribuir a Consensus. En esta etapa buscamos que el trabajo del equipo sea visible, ordenado y fácil de revisar.

## Flujo de trabajo

1. Elige o crea un issue que describa la tarea.
2. Crea una rama desde `main`.
3. Realiza cambios pequeños y enfocados.
4. Escribe commits claros.
5. Abre un pull request relacionado con el issue.
6. Solicita al menos una revisión antes de integrar el cambio.
7. Atiende los comentarios y mantén la conversación técnica dentro del pull request.

No hagas push directo a `main` salvo para una corrección administrativa acordada por el equipo.

## Nombres de ramas

Usa nombres cortos en minúsculas y separados por guiones:

- `feature/nombre-de-la-funcionalidad`
- `fix/nombre-del-error`
- `docs/tema-documentado`
- `chore/tarea-interna`
- `research/tema-investigado`

Ejemplo: `docs/definir-flujo-de-votacion`.

## Commits

Utilizamos una versión sencilla de Conventional Commits:

- `feat: ...` para una funcionalidad.
- `fix: ...` para una corrección.
- `docs: ...` para documentación.
- `test: ...` para pruebas.
- `refactor: ...` para una mejora interna sin cambiar el comportamiento.
- `chore: ...` para mantenimiento.
- `research: ...` para hallazgos o experimentos.

Procura que cada commit represente una unidad de trabajo comprensible.

## Issues

Antes de iniciar una tarea:

- Comprueba que no exista un issue equivalente.
- Describe el problema o resultado esperado.
- Incluye criterios de aceptación verificables.
- Indica dependencias o bloqueos conocidos.
- Asigna la tarea a la persona que la realizará.

## Pull requests

Un pull request debe:

- Resolver una sola tarea o cambio relacionado.
- Explicar qué cambia y por qué.
- Enlazar el issue correspondiente con `Closes #número` cuando aplique.
- Incluir instrucciones de validación.
- Mantener fuera cambios no relacionados.
- Estar actualizado con `main` antes de integrarse.

## Decisiones técnicas

Las decisiones que afecten el alcance, la privacidad, el modelo on-chain/off-chain o las reglas de votación deben discutirse en un issue antes de implementarse. La conclusión debe quedar documentada en el repositorio.

## Definición de terminado

Una tarea está terminada cuando:

- Cumple sus criterios de aceptación.
- Fue validada por quien la desarrolló.
- Tiene documentación suficiente.
- Recibió revisión de otra persona.
- No incorpora información sensible ni secretos.
