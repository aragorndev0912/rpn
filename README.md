# Evaluación aritmética para un compilador o intérprete (_RPN_).

El intérprete del lenguaje de programación __Amaia__ (_próximamente  se explicará_), internamente utilizará la [Notación Polaca Inversa](https://es.wikipedia.org/wiki/Notaci%C3%B3n_polaca_inversa). para la evaluación de las expresiones aritméticas. Se desarrolló un módulo escrito en el lenguaje [Rust](https://www.rust-lang.org/) que permite evaluar expresiones definidas dentro de un fichero de texto con extensión `.ope`, escritas utilizando la [Notación Algebraica](https://es.wikipedia.org/wiki/Notaci%C3%B3n_algebraica), para luego ser convertidas a expresiones definidas bajo la [Notación Polaca Inversa](https://es.wikipedia.org/wiki/Notaci%C3%B3n_polaca_inversa) y al final evaluarlas y obtener el resultado de la operaciones.

## Conceptos.

La [Notación Polaca Inversa](https://es.wikipedia.org/wiki/Notaci%C3%B3n_polaca_inversa) o por sus siglas en ingles RPN (Reverse Polish Notation), es un método de cálculo de operaciones aritméticas que es utilizada comúnmente por los desarrolladores de compiladores e interpretes, ya que simplifica el manejo de los operandos y operadores a través del uso de estructuras de datos de tipo LIFO ([Pilas](https://es.wikipedia.org/wiki/Pila_(inform%C3%A1tica))).

La Notación Polaca fue desarrollada por un matemático polaco llamado [Jan Łukasiewicz](https://es.wikipedia.org/wiki/Jan_%C5%81ukasiewicz) en el año 1920, donde el operador se introducía antes que los operandos, es decir, si se quiere sumar dos numero `8 + 12`, por dar un ejemplo, se tendría que escribir de la siguiente manera: `+ 8 12`, siendo el resultado de la expresión `20`. Esto permitió eliminar el uso de parentesis en las expresiones aritméticas, lo que facilita, en gran medida el manejo de las reglas de precedencia entre operaciones.

La Notación Polaca Inversa se aplica al revés de lo definido por [Jan Łukasiewicz](https://es.wikipedia.org/wiki/Jan_%C5%81ukasiewicz), ya que para hacer el mismo calculo anterior `8 + 12`, la expresión se escribiría de la siguiente manera: `8 12 +`, arrojando como resultado  `20`.

La Notación Polaca Inversa nace de la necesidad de simplificar el diseño de las máquinas de computo, como *compiladores e interpretes* mediante el uso de las [Pilas](https://es.wikipedia.org/wiki/Pila_(inform%C3%A1tica)), ya que el uso de estas junto con el método **RPN** permite el análisis de expresiones complejas sin necesidad de evaluar la precedencia entre operadores o paréntesis, lo que reduce y simplifica en gran medida la programación de máquinas de computo. 


## Ventajas & Desventajas el uso de RPN.

### Ventajas:
+ La evaluación de las operaciones se van ejecutando a medida que se introducen los datos.


+ No necesita el uso de reglas de precedencia ni paréntesis.

+ Permite almacenar datos a través de la estructura LIFO (Pila) para su uso posterior, lo que permite el computo de expresiones con una complejidad alta.

### Desventajas:

+ La adopción universal de la notación algebraica como vía para realizar las evaluaciones de las expresiones aritméticas.

## Ejemplo

A continuación, se muestra un conjunto de operaciones aritméticas escritas tanto en infijo y su equivalente en postfijo. Estas operaciones solo contemplan los siguientes operadores aritméticos: `+ - * /` y un único operador de agrupación `()`.

### Expresiones aritmeticas Infijo.


```
6 + 2 * 5 - 8 / 4

(6 + 2) * 5 - 8 / 4

((6 + 2) * 5) * 8 - 4

(((5 + 9) * 2) + (6 * 5))

8 + 5 * 4 - 12 / 68

5 + ((1 + 2) * 4) - 3
```

### Equivalencia en Postfijo (__RPN__).

```
6 2 5 *+8 4/-

6 2+ 5 *8 4/-

6 2+ 5* 8 *4-

5 9+ 2* 6 5*+

8 5 4 *+12 68/-

5 1 2+ 4* +3-
```