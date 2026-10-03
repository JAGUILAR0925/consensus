# Propuesta individual — Luis Martínez

**Nombre:** Luis Martínez  
**Usuario de GitHub:** [@MrLuis-WebMaster](https://github.com/MrLuis-WebMaster)

## El problema

En votaciones comunitarias digitales, los participantes no siempre pueden comprobar por sí mismos que se respetaron las reglas de participación y que el resultado publicado corresponde a los votos aceptados.

## ¿Quién lo sufre?

Lo puede sufrir una persona habilitada para votar en una elección de una asociación, colectivo o comunidad cuando la entidad organizadora también administra el padrón, recibe los votos, realiza el conteo y publica el resultado. Aunque haya emitido su voto, quizá solo reciba una confirmación de pantalla o el resultado final, sin evidencia que pueda revisar de manera independiente.

También afecta a los candidatos y observadores que necesitan comprobar si la votación abrió y cerró según lo anunciado, si se rechazaron votos no habilitados o duplicados y si el conteo coincide con las operaciones aceptadas. No afirmo que todas las organizaciones manipulen resultados: el problema es que, cuando el proceso carece de mecanismos de verificación independientes, los demás actores deben confiar en lo que informa el operador.

Para explorar el caso sin utilizar datos personales ni procesos reales, propongo comenzar con una elección simulada de representantes estudiantiles.

## ¿Cómo se resuelve hoy y qué cuesta?

En un flujo digital centralizado, la organización registra a las personas habilitadas, recibe sus votos en una aplicación, calcula el conteo y comunica un resultado. Los participantes pueden conservar una captura de la confirmación y los observadores pueden solicitar actas, reportes o registros al administrador. En algunos casos hay testigos o una revisión posterior, pero el acceso a la evidencia depende de la misma entidad que operó el sistema.

El costo principal es de **esfuerzo y tiempo**: pedir información, contrastar versiones de los reportes y reconstruir qué pasó si se cuestiona un resultado. Para la organización, preparar evidencias adicionales o atender reclamaciones también requiere trabajo. No dispongo de una medición de costos monetarios para este caso; esa parte habría que validarla con organizaciones y participantes reales.

Una confirmación de que la aplicación recibió un voto tampoco demuestra por sí sola que este se contó correctamente ni que el resultado respete todas las reglas. La literatura sobre verificabilidad de extremo a extremo distingue estas comprobaciones y motiva el problema que quiero investigar.

## ¿Por qué creo que blockchain podría aportar?

*Esta es una hipótesis personal, no una solución demostrada.*

Creo que un registro compartido e históricamente verificable podría reducir la dependencia de un único operador para consultar la secuencia de operaciones y contrastar el resultado anunciado. El criterio que considero más relevante es que **actores con intereses distintos necesitan revisar un mismo registro** sin que una sola parte controle toda la evidencia. Si las reglas de apertura, cierre, autorización y conteo pueden aplicarse y verificarse de forma independiente, un participante u observador tendría una manera adicional de comprobar el proceso.

Esto solo aportaría valor si esa verificación fuese comprensible y si el sistema demostrara algo más que la existencia de transacciones. Registrar votos en una red no resuelve identidad, voto secreto, coerción, seguridad del dispositivo ni posibles errores al habilitar participantes. Tampoco he demostrado que un registro distribuido sea mejor que una base de datos con auditoría independiente para este contexto; la comparación debe formar parte de la investigación del equipo.

**Fuentes de contexto:**

- Benaloh et al., [*End-to-end verifiability*](https://arxiv.org/abs/1504.03778) (2015): marco para distinguir el registro de un voto de la comprobación del resultado.
- National Academies, [*Securing the Vote*](https://www.nationalacademies.org/read/25120/chapter/7) (2018): riesgos y necesidad de evidencia en procesos de votación electrónica.
