// tests/tests.rs
use binarybit::uwu::Uwu;  // Importando a enum Uwu do seu crate

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = binarybit::add(2, 2);
        println!("OOOOOO");
        assert_eq!(result, 4);
    }

    #[test]
    fn test_uwu_enum() {
        // Criando os valores usando as funções do Uwu
        let result_true = Uwu::yea();
        let result_false = Uwu::nah();

        // Usando println! para imprimir os resultados
        println!("Resultado verdadeiro: {:?}", result_true);  // Imprime o valor verdadeiro
        println!("Resultado falso: {:?}", result_false);  // Imprime o valor falso

        // Testando a comparação
        assert_eq!(result_true, Uwu::Yea);  // Testa se o valor 'true' é correto
        assert_eq!(result_false, Uwu::Nah);  // Testa se o valor 'false' é correto
    }
}
