# 02. Personas y actores

**Estado:** definido mediante escenarios sintéticos  
**Última revisión:** septiembre de 2026

> Los perfiles siguientes fueron creados por el equipo para diseñar y probar el MVP. No representan entrevistas ni personas reales.

## Persona principal — SCN-001

### Votante habilitado

**Objetivo:** participar una vez y comprobar que el sistema aceptó su voto.

**Necesidades:**

- Instrucciones comprensibles.
- Confirmación inequívoca.
- Protección de información personal.
- Explicación simple de la evidencia en Stellar.
- Mensajes útiles ante errores.

**Riesgos:**

- No comprender una billetera o firma.
- Confundir una transacción enviada con una confirmada.
- Exponer su elección si el diseño no ofrece privacidad.
- Perder acceso a sus credenciales.

## Actor administrador — SCN-002

### Organizador de la elección

**Objetivo:** configurar el proceso, habilitar participantes y controlar sus estados.

**Necesidades:**

- Evitar configuraciones inválidas.
- Revisar candidatos y elegibilidad antes de abrir.
- Contar con evidencia de cada acción administrativa.
- No poder alterar silenciosamente el resultado.

**Riesgos:**

- Habilitar una dirección incorrecta.
- Abrir antes de completar la configuración.
- Perder la clave administrativa.
- Concentrar demasiado poder.

## Actor candidato — SCN-003

### Persona incluida como opción

**Objetivo:** consultar el resultado y verificar que el conteo cumple las reglas.

**Necesidades:**

- Resultado público.
- Explicación de cómo se obtiene.
- Evidencia que no dependa del panel administrativo.

## Actor observador — SCN-004

### Auditor técnico o miembro de la comunidad

**Objetivo:** reproducir la verificación.

**Necesidades:**

- Identificadores públicos de red.
- Reglas documentadas.
- Código y pruebas accesibles.
- Instrucciones de verificación independientes.

## Actor facilitador — SCN-005

### Integrante que acompaña el piloto educativo

**Objetivo:** ayudar a completar el ejercicio sin intervenir en el resultado.

**Necesidades:**

- Guía de preparación.
- Datos de prueba.
- Protocolo para reiniciar el escenario.
- Lista de limitaciones que debe explicar.

## Matriz de responsabilidades

| Acción | Votante | Administrador | Candidato | Observador | Facilitador |
| --- | --- | --- | --- | --- | --- |
| Consultar reglas | Sí | Sí | Sí | Sí | Sí |
| Configurar elección | No | Sí | No | No | Apoya |
| Habilitar participante | No | Sí | No | No | Prepara datos |
| Emitir voto | Sí | No | No | No | No |
| Consultar resultado | Sí | Sí | Sí | Sí | Sí |
| Verificar evidencia | Opcional | Sí | Sí | Sí | Sí |
| Modificar votos | No | No | No | No | No |

## Fuente de los escenarios

Los perfiles se derivan de funciones comunes en sistemas de votación verificable y de los requisitos de integridad, seguridad, accesibilidad y auditoría identificados en la investigación documental.
