//ENTRADA STD
fn main() {
    println!("Advinhe o número!");

    println!("Digite o seu palpite.");

    let mut palpite = String::new();

    std::io::stdin().read_line(&mut palpite) 
        .expect("Falha ao ler entrada");

    println!("Você disse: {}", palpite);
}