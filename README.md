# Projeto Rust - Enum Uwu

Este é um projeto Rust que apresenta um enum chamado `Uwu`, com variantes que podem ser consideradas como "verdadeiras" ou "falsas". O projeto também define métodos para inverter o valor lógico do enum (`not()`) e converte o enum em um valor booleano para ser usado diretamente em estruturas condicionais (`if`).

## Funcionalidade

- O enum `Uwu` tem variantes que podem ser usadas como valores booleanos.
- As variantes `true` são `Yea`, `Yeah`,  `Huzzah`, `Bliss`, `Kawaii` e `Yep` e todas as outras são `false`. Portanto, o binarybit, apenas aceita: `Nah`, `Nope`, `Nop`,  `Meh` e `Owo` como `false`.
- O método `not()` inverte o valor lógico atribuido para o objeto do tipo Uwu.


### 3. Exemplos de Uso


#### Declaração e Comparação

```rust

extern crate binarybit::uwu::Uwu;
use crate::uwu::Uwu;


fn main() {

    // Exemplos de declaração
    let result_true: Uwu = Uwu::yea();
    let result_false: Uwu = Uwu::nah();

    // Você pode usar para especificar o valor de uma variável do tipo bool.

    let some_bool: bool = Uwu::huzzah().into();
    println!("huzzah return --> {}", some_bool);

    println!("{:?} {:?}", result_true, result_false); 


    // Método not e sintaxe no if
    if result_true.into() {
        println!("Print this if true.");  // Isso será impresso
    }

    if Uwu::not(result_false).into() {
        println!("print this if not(false) == true.");   
    }
}
```
