extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Advinhe o número!");

    let numero_secreto = rand::thread_rng().gen_range(1, 101);

    println!("O número secreto é: {}", numero_secreto);

    println!("Digite o seu palpite.");

    let mut palpite = String::new();

    io::stdin().read_line(&mut palpite)
        .expect("Falha ao ler entrada");

    let palpite: u32 = palpite.trim().parse()
        .expect("Por favor, digite um número!");
 
    println!("Você disse: {}", palpite);

    match palpite.cmp(&numero_secreto) {
        Ordering::Less => println!("Muito baixo!"),
        Ordering::Greater => println!("Muito alto!"),
        Ordering::Equal => println!("Você acertou!"),
    }   

    match palpite { 
        // Match a single value 
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("dois a onze"),
        // Match an inclusive range
        13..=19 => println!("treza a dezenove"),
        // Handle the rest of cases
        _ => println!("acima de dezenove"),
    }
}