# Propuesta individual

**\*\*Nombre:\*\*** Daniela Osorio Rivera  

**\*\*Usuario de GitHub:\*\*** [@DanoRiv](https://github.com/danoriv)

## El problema

En trabajos freelance desarrollados por etapas, clientes y profesionales pueden tener dificultades para garantizar que el pago de cada entregable se realice una vez cumplidas las condiciones acordadas, sin que una de las partes tenga que asumir todo el riesgo.

## ¿Quién lo sufre?

Lo sufren tanto **freelancers como clientes**, especialmente cuando no existe una relación previa de confianza y el proyecto tiene varios entregables o pagos parciales.

Por ejemplo, un desarrollador puede acordar con un cliente un proyecto de $1.000.000 dividido en cuatro hitos. El cliente puede tener dudas sobre pagar por adelantado sin haber recibido el trabajo, mientras que el freelancer puede tener dudas sobre invertir tiempo y entregar un resultado sin tener garantías suficientes de que recibirá el pago correspondiente.

El problema también puede presentarse en trabajos de diseño, desarrollo de software, consultoría, edición audiovisual, marketing y otros servicios que puedan dividirse en entregables verificables.

No afirmo que todos los clientes incumplan sus pagos ni que todos los freelancers incumplan sus entregas: el problema es que, cuando las partes no tienen una relación de confianza previa, ambas necesitan mecanismos para reducir el riesgo de que la otra parte no cumpla lo acordado.

Para explorar el caso sin involucrar contratos reales ni dinero real, propongo comenzar con un proyecto freelance simulado dividido en varios entregables.

## ¿Cómo se resuelve hoy y qué cuesta?

Actualmente, el cliente y el freelancer suelen acordar el precio, los entregables y las fechas mediante contratos, mensajes o plataformas intermediarias. El pago puede realizarse por adelantado, después de cada entrega o una vez terminado todo el proyecto.

También existen plataformas que pueden actuar como intermediarias y mantener los fondos hasta que se cumplan determinadas condiciones. Cuando las partes trabajan directamente, suelen utilizar transferencias bancarias, billeteras digitales u otros medios de pago y conservar comprobantes de las operaciones.

Esto puede generar costos de **dinero, tiempo y esfuerzo**. Las plataformas intermediarias pueden cobrar comisiones, mientras que en acuerdos directos las partes deben hacer seguimiento manual de los pagos, entregables y fechas establecidas. Si aparece un desacuerdo, puede ser necesario revisar conversaciones, contratos, comprobantes y archivos para determinar qué se había acordado y qué parte cumplió.

Además, el cliente puede asumir el riesgo de pagar antes de recibir un entregable, mientras que el freelancer puede asumir el riesgo contrario: completar y entregar un trabajo sin tener certeza de que recibirá el pago.

El problema no es necesariamente la ausencia de mecanismos de pago, sino la dificultad de **coordinar el pago con el cumplimiento de cada etapa del trabajo** cuando las partes no tienen suficiente confianza entre sí.

## ¿Por qué creo que blockchain podría aportar?

*Esta es una hipótesis personal, no una solución demostrada.*

Creo que blockchain podría aportar mediante un sistema de **fondos reservados y pagos por hitos**, en el que el dinero destinado al proyecto quede comprometido desde el inicio y se vaya liberando a medida que se cumplan las condiciones acordadas.

El criterio que considero más relevante es que **dos actores con intereses distintos necesitan compartir un registro de los compromisos y transferencias sin que una sola de las partes controle completamente los fondos o el historial de operaciones**.

Por ejemplo, un cliente podría reservar el valor total del proyecto y dividirlo en varios pagos:

```text
Cliente
   │
   │ reserva $1.000.000
   ▼
Fondos del proyecto
   │
   ├── Hito 1 → $200.000
   ├── Hito 2 → $300.000
   ├── Hito 3 → $300.000
   └── Hito 4 → $200.000
                       │
                       ▼
                   Freelancer