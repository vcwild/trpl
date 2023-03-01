#[derive(Debug)] // Para podermos ver qual é o estado com mais facilidade
enum Estado {
    Alabama,
    Alaska,
    // ... etc
}

enum Moeda {
    Penny,
    Nickel,
    Dime,
    Quarter(Estado),
}



fn main() {
    let mut contagem = 0;
    let moeda = Moeda::Quarter(Estado::Alabama);
    match moeda {
        Moeda::Quarter(estado) => println!("Quarter do estado {:?}!", estado),
        _ => contagem += 1,
    };

    println!("O valor da contagem é {}", contagem);
}
