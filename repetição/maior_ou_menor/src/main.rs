use std::io;
use std::cmp::Ordering;

fn main() {
    loop {
        let mut n1 = String::new();
        println!("Insira o primeiro valor:");

        io::stdin()
            .read_line(&mut n1)
            .expect("Falha ao ler o valor");

        let n1: u32 = match n1
            .trim()
            .parse()
            {
               Ok(num) => num,
               Err(_) => continue,
            };

        
        let mut n2 = String::new();
            println!("Insira o segundo valor:");    

        io::stdin()
            .read_line(&mut n2)
            .expect("Falha ao ler o valor");   

        let n2: u32 = match n2
            .trim()
            .parse()
            {
               Ok(num) => num,
               Err(_) => continue,
            };

        match n1.cmp(&n2) {
            Ordering::Less => println!("O número {} é menor que {}", n1,n2),
            Ordering::Greater => println!("O número {} é maior que {}", n1,n2),
            Ordering::Equal => println!("O número {} é igual a {}", n1, n2),
        }
    }
}
