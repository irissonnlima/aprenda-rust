fn ler_arquivo(caminho_arquivo:String) -> Option<String>{
    Some(String::from("Conteudo arquivo"))
}

fn if_let_test(){
    if let Some(valor) = conteudo_arquivo{
        println!("Podemos ver o valor {}", valor);
    }
}

/*
Diferente do Match o if let permite que possamos tratar um único caso do retorno.
Sendo os trechos equivalentes:

match conteudo_arquivo{
    Some(valor) => println!("{}", valor),
    None => ()
};

if let Some(valor) = conteudo_arquivo{
    println!("Podemos ver o valor {}", valor);
}

*/