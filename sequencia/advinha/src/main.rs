use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let num_random:u32 = rand::thread_rng()
        .gen_range(1, 101);
    
    loop {
        let mut guess = String::new();
        println!("Chute um número de 1 a 100:");

        io::stdin()
            .read_line(&mut guess)
            .expect("Falha ao ler o palpite");

        let guess: u32 = match guess
            .trim()
            .parse()
            {
               Ok(num) => num,
               Err(_) => continue,
            };
        
        println!("Você palpitou: {}", guess);

        match guess.cmp(&num_random) {
            Ordering::Less => println!("O número é maior"),
            Ordering::Greater => println!("O número é menor"),
            Ordering::Equal => {
                println!("Parabéns, você tem muita sorte e acertou!");
                break;
            }
        }
    }
}
