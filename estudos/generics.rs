// TEMPLATES PARA RUST


enum Valor<T>{
    Some(T),
    None
}

fn exemplo_utilizacao() -> Valor<String>{
    Some(String::from("Algum conteúdo!"))
}