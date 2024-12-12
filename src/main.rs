use std::io;

fn main() {
    println!("Guess the number!");
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