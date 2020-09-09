//CONDIÇÃO COM LOOP
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Jogo!!! - Advinhe o número entre 1 a 20!");
    println!("Voce tem três tentativas");

    let numero_secreto = rand::thread_rng().gen_range(1, 20);

    let mut counter = 0;

    //println!("O número secreto é: {}", numero_secreto);

    loop {
       

        println!("Digite o seu palpite.");

        let mut palpite = String::new();

        io::stdin().read_line(&mut palpite)
            .expect("Falha ao ler entrada");
        
        let palpite: u32 = match palpite.trim().parse() {  //tratamento de erro
            Ok(num) => num,
            Err(_) => continue,
           // .expect("Por favor, digite um número!");
        };

        println!("Você disse: {}", palpite);

        if counter == 2{
            println!("Limite 3 tentativas", ); break;
        }

        match palpite.cmp(&numero_secreto) {
            Ordering::Less => {
                println!("Muito baixo!");
                counter += 1;               
            },
            Ordering::Greater => {
                println!("Muito alto!");
                counter += 1; 
            }
            Ordering::Equal => {
                println!("Você acertou!");
                break;
            },
        } 
    }
}