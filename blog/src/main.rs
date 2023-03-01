extern crate blog;
use blog::Postagem;

fn main() {
    let mut post = Postagem::new();

    post.add_texto("Eu comi uma salada no almoço de hoje");

    let post = post.solicitar_revisao();

    let post = post.aprovar();

    assert_eq!("Eu comi uma salada no almoço de hoje", post.conteudo());
}
