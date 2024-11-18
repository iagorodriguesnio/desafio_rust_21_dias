use std::io;

fn main() {
    println!("Adivinhe o número!");

    println!("Digite o seu palpite");

    let mut palavra = String::new();

    io::stdin()
        .read_line(&mut palavra)
        .expect("Algo deu errado.");

    println!("Você digitou {}", palavra);
}
