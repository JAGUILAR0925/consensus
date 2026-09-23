# 08. Enfoque técnico

**Estado:** propuesta provisional  
**Última revisión:** septiembre de 2026

## Objetivo técnico

Construir la prueba funcional más pequeña que permita validar las reglas de una votación y su verificabilidad en Stellar Testnet.

## Componentes propuestos

| Componente | Tecnología propuesta | Responsabilidad |
| --- | --- | --- |
| Contrato inteligente | Soroban con Rust | Estados, autorización, elegibilidad, prevención de doble voto y conteo. |
| Red | Stellar Testnet | Ejecución y evidencia pública para el piloto. |
| Herramientas | Stellar CLI | Compilación, despliegue, invocación y operación administrativa de respaldo. |
| Aplicación web | React, Vite y TypeScript | Flujos de administrador, votante, resultados y mensajes de error. |
| Cliente | SDK oficial de Stellar y bindings del contrato | Simulación, construcción y envío de transacciones. |
| Billetera | Freighter | Firma y autorización del usuario. |
| Consulta externa | Explorador compatible con Stellar | Comprobación independiente de transacciones. |
| Publicación | Hosting estático | Acceso al piloto. |

## Decisión de arquitectura inicial

No se propone un backend propio para el MVP. El contrato será la fuente de verdad y la aplicación web leerá el estado y enviará transacciones.

Esta decisión reduce alcance, pero deberá revisarse si aparecen necesidades off-chain como invitaciones, notificaciones, contenido extenso o integración de identidad.

## Responsabilidades del contrato

| Operación | Actor autorizado | Regla |
| --- | --- | --- |
| Inicializar | Despliegue | Se ejecuta una sola vez y define al administrador. |
| Registrar candidato | Administrador | Solo durante Configuración. |
| Habilitar dirección | Administrador | Solo durante Configuración. |
| Abrir elección | Administrador | Transición de Configuración a Abierta. |
| Cerrar elección | Administrador | Transición de Abierta a Cerrada. |
| Votar | Votante | Requiere firma, habilitación, elección abierta, candidato válido y no haber votado. |
| Consultar | Cualquiera | Lectura de estado, conteo y participación registrada. |

Los nombres definitivos de las funciones se decidirán al implementar y probar el contrato.

## Datos on-chain

- Estado de la elección.
- Candidatos o identificadores compactos.
- Direcciones habilitadas para el piloto.
- Marca de participación por dirección.
- Conteo.
- Eventos o referencias necesarias para auditoría.

## Datos que no deben publicarse

- Nombre del estudiante.
- Documento o código institucional.
- Correo, teléfono o información académica.
- Clave privada o frase de recuperación.
- Correspondencia entre identidad real y dirección.
- Respuestas de entrevistas sin anonimizar.

## Seguridad mínima

- Autorización explícita del administrador.
- Autorización explícita del votante.
- Transiciones de estado válidas.
- Protección contra inicialización repetida.
- Validación de candidatos.
- Prevención de doble voto.
- Pruebas para cada regla crítica.
- Manejo documentado de expiración o archivo de datos del contrato.
- Proceso reproducible después de un reinicio de Testnet.

## Pruebas mínimas

- Voto válido.
- Doble voto rechazado.
- Dirección no habilitada rechazada.
- Voto fuera del estado Abierta rechazado.
- Operación administrativa sin autorización rechazada.
- Candidato inválido rechazado.
- Transiciones de estado inválidas rechazadas.

## Decisiones pendientes

- Representación de candidatos y límites de almacenamiento.
- Uso de eventos y nivel de detalle publicado.
- Estrategia de duración de datos.
- Experiencia para habilitar varias direcciones.
- Herramienta externa de exploración.
- Framework frontend definitivo.

## Referencias oficiales

- [Descripción de contratos inteligentes](https://developers.stellar.org/docs/build/smart-contracts/overview)
- [Contrato Hello World y pruebas](https://developers.stellar.org/docs/build/smart-contracts/getting-started/hello-world)
- [Despliegue en Testnet con Stellar CLI](https://developers.stellar.org/docs/build/smart-contracts/getting-started/deploy-to-testnet)
- [Integración con Freighter](https://developers.stellar.org/docs/build/guides/freighter)
- [Automatización tras reinicios de Testnet](https://developers.stellar.org/docs/build/guides/basics/automate-reset-data)
