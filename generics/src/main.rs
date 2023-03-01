struct ExcertoImportante<'a> {
    parte: &'a str,
}

fn main() {
    let romance = String::from("Chame-me Ishmael. Há alguns anos...");
    let primeira_sentenca = romance.split('.')
        .next()
        .expect("Não pôde achar um '.'");
    let i = ExcertoImportante { parte: primeira_sentenca };

    println!("Excerto: {}", i.parte);
}
