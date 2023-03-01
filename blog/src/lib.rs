pub struct Postagem {
    conteudo: String,
}

pub struct RascunhoPostagem {
    conteudo: String,
}

pub struct RevisaoPendentePostagem {
    conteudo: String,
}

impl RascunhoPostagem {
    pub fn solicitar_revisao(self) -> RevisaoPendentePostagem {
        RevisaoPendentePostagem {
            conteudo: self.conteudo,
        }
    }
}

impl Postagem {
    pub fn new() -> RascunhoPostagem {
        RascunhoPostagem {
            conteudo: String::new(),
        }
    }

    pub fn conteudo(&self) -> &str {
        &self.conteudo
    }

    pub fn solicitar_revisao(self) -> RevisaoPendentePostagem {
        RevisaoPendentePostagem {
            conteudo: self.conteudo,
        }
    }
}

impl RevisaoPendentePostagem {
    pub fn aprovar(self) -> Postagem {
        Postagem {
            conteudo: self.conteudo,
        }
    }
}

impl RascunhoPostagem {
    pub fn add_texto(&mut self, text: &str) {
        self.conteudo.push_str(text);
    }
}
