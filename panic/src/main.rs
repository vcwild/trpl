pub struct Palpite {
    valor: u32,
}

impl Palpite {
    pub fn new(valor: u32) -> Palpite {
        if valor < 1 || valor > 100 {
            panic!("Valor de chute deve ser entre 1 e 100, recebi {}.", valor);
        }

        Palpite {
            valor
        }
    }

    pub fn valor(&self) -> u32 {
        self.valor
    }
}

fn main() {
    let palpite = Palpite::new(200);
    println!("Palpite: {}", palpite.valor());
}
