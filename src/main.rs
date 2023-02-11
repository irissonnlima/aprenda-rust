fn main() {
    // Print usual no rust, sendo uma macro!!
    println!("Hello, world!");
    // variáveis
    let total = 30; // as variáveis precisam ser definidas
                    // dentro de um escopo definido
    println!("Trabalhou {} horas", total);

    let _nome_var: i32 = 30;    // não é nescessário declarar o tipo
                                // a menos que o compilador solicite
                                // variáveis com _ significa para 
                                //ignorar o warning

    // para declarar variáveis mutaveis teremos que fazer:
    let mut _val = 30;
    _val = 40;
    //Rust não tem coerção automática de tipo então mesmo não declarando
    //explicitamente você não pode reatribuir o valor para outro tipo.
    //Porém, você pode redeclarar o tipo da variável
    let val = "irisson";
    println!("olá {}!", val);
    
    {
        let total = total + 50;
        println!("{}", total);
    }

    println!("{}",total);
}
