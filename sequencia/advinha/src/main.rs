use std::io;
use rand::Rng;

fn main() {
    println!("Advinhe o número, Digite o seu palpite:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Falha ao ler o palpite");
    
    println!("Você palpitou: {}", guess);
}
