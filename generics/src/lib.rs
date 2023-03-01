pub trait Resumir {
  fn resumo(&self) -> String;
}

pub struct ArtigoDeNoticia {
  pub titulo: String,
  pub local: String,
  pub autor: String,
  pub conteudo: String,
}

impl Resumir for ArtigoDeNoticia {
  fn resumo(&self) -> String {
      format!("{}, by {} ({})", self.titulo, self.autor, self.local)
  }
}

pub struct Tweet {
  pub nomeusuario: String,
  pub conteudo: String,
  pub resposta: bool,
  pub retweet: bool,
}

impl Resumir for Tweet {
  fn resumo(&self) -> String {
      format!("{}: {}", self.nomeusuario, self.conteudo)
  }
}
