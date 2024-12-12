use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // Gerando um numero aleatorio
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess");

    let mut guess = String::new();
    // Variaveis sao imutaveis por padrao
    // e necessario declarar como mutavel (mut)
    // para alterar seu valor

    // Usando a instancia de Stdin fornecida por
    // std::io::stdin() para ler dados inseridos no terminal
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    // O metodo expect finaliza o programa e exibe a
    // mensagem de erro informada caso o resultado
    // retornado seja o enum Result<Error>

    println!("You guessed: {}", guess);
}