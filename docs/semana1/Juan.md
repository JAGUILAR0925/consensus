# Propuesta individual — Juan Carlos Aguilar Herrera

## El problema

Al terminar un contrato de arrendamiento de vivienda, el inquilino no sabe si recuperará su depósito en garantía, porque el arrendador custodia el dinero y decide por sí solo cuánto descontar.

## ¿Quién lo sufre?

El inquilino de vivienda en México, en el momento de entregar el inmueble. Durante todo el contrato ha dejado en manos del arrendador un depósito que suele equivaler a un mes de renta, y en algunos casos a dos (propiedades amuebladas, inquilinos sin aval o estancias cortas). Al salir, depende de la buena fe del arrendador: puede recibir el depósito completo, recibirlo con descuentos que no se le justifican o no recibirlo.

También lo sufre, en menor medida, el arrendador honesto, que no tiene cómo demostrar de forma confiable el estado del inmueble al inicio y al final del contrato cuando necesita justificar un descuento por daños.

## ¿Cómo se resuelve hoy y qué cuesta?

Hoy el inquilino se protege por su cuenta: pide un recibo del depósito, toma fotografías del inmueble al entrar y guarda sus comprobantes de pago. Si al final el arrendador no devuelve el depósito o descuenta sin justificar, el camino es:

1. Reclamar por escrito, con acuse de recibo, citando la cláusula del contrato.
2. Acudir a conciliación.
3. Demandar ante un juez civil.

**Costo:**

- **Dinero:** el valor del depósito (uno o dos meses de renta) queda en riesgo, y un proceso judicial puede costar más que el monto en disputa.
- **Tiempo:** semanas o meses de reclamos y trámites mientras el dinero sigue retenido.
- **Esfuerzo:** reunir pruebas que suelen ser informales (fotos en el celular, mensajes) y cuya fecha o autenticidad la contraparte puede cuestionar.

## ¿Por qué creo que blockchain podría aportar?

_Hipótesis personal, no certeza._

Creo que blockchain podría aportar porque el caso cumple dos criterios de la Sesión 1:

- **Eliminar un intermediario que concentra la confianza.** Hoy el arrendador es al mismo tiempo custodio del dinero y juez de los descuentos. Mi hipótesis es que un contrato inteligente podría custodiar el depósito de forma neutral: el arrendador propone un descuento, el inquilino lo acepta o lo disputa y, si el arrendador no reclama dentro del plazo acordado, el dinero vuelve automáticamente al inquilino. Esto se parece a lo que ya plantea el Código Civil Federal (art. 2422): el saldo se devuelve al terminar el contrato y, si el arrendador cree tener un derecho, debe depositarlo ante un juez en lugar de retenerlo por su cuenta. _(Pendiente: verificar el texto oficial del artículo.)_
- **Partes que no confían entre sí comparten un registro inalterable.** El hash de las fotos del inventario de entrada y de salida quedaría registrado con fecha en la red, así que ninguna de las partes podría alterar después la evidencia del estado del inmueble.

Lo que tendría que validar:

- Si los arrendadores estarían dispuestos a ceder la custodia del depósito.
- Si ambas partes aceptarían a un mediador para resolver disputas.

**Nota de contexto:** en Colombia esta propuesta no aplica para vivienda urbana, porque el artículo 16 de la Ley 820 de 2003 prohíbe exigir depósitos en dinero, incluso de forma indirecta o con otra denominación. Por eso la enfoco en México, donde el depósito es legal y de uso común.

## Fuentes

- Ley 820 de 2003, art. 16 (Función Pública, fuente oficial): https://www.funcionpublica.gov.co/eva/gestornormativo/norma.php?i=8738
- Práctica del depósito en México y proceso de reclamo (Century 21 México, fuente secundaria): https://blog.century21mexico.com/renta-de-inmuebles/deposito-de-renta/
- Art. 2422 del Código Civil Federal, devolución del saldo (City Laws, fuente secundaria; pendiente verificar en el texto oficial): https://citylaws.com.mx/abogado-inmobiliario/deposito-garantia-renta-cuando-devuelven/
  xxx

---
