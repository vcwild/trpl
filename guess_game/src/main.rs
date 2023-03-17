use std::io;

fn main() {
    println!("adivinhe o numero:");
    println!("Digite  o seu palpite:");

    let mut palpite = String::new();
    io::stdin()
        .read_line(&mut palpite)
        .expect("falha ao ler a entrada");

    println!("voce disse:{}", palpite);
}
