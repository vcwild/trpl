pub struct ColecaoDeMedia {
    lista: Vec<i32>,
    media: f64,
}

impl ColecaoDeMedia {
    pub fn new() -> ColecaoDeMedia {
        ColecaoDeMedia {
            lista: Vec::new(),
            media: 0.0,
        }
    }

    pub fn adicionar(&mut self, valor: i32) {
        self.lista.push(valor);
        self.atualizar_media();
    }

    pub fn remover(&mut self) -> Option<i32> {
        let resultado = self.lista.pop();
        match resultado {
            Some(valor) => {
                self.atualizar_media();
                Some(valor)
            }
            None => None,
        }
    }

    pub fn media(&self) -> f64 {
        self.media
    }

    pub fn tamanho(&self) -> usize {
        self.lista.len()
    }

    pub fn total(&self) -> i32 {
        self.lista.iter().sum()
    }

    fn atualizar_media(&mut self) {
        let total: i32 = self.total();
        self.media = total as f64 / self.lista.len() as f64;
    }
}

fn main() {
    let mut media = ColecaoDeMedia::new();

    media.adicionar(10);
    media.adicionar(20);
    media.adicionar(30);
    media.adicionar(40);

    println!(
        "Tamanho: {},Total: {}, Média: {}",
        media.tamanho(),
        media.total(),
        media.media(),
    );

    media.remover();
    media.remover();

    println!(
        "Tamanho: {},Total: {}, Média: {}",
        media.tamanho(),
        media.total(),
        media.media(),
    );
}
