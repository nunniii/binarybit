mod uwu;  // Importando o módulo uwu.rs

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

