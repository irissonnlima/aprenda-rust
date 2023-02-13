// Não funcionou, ajustar posteriormente

fn maskify(cc: &str)->String{

    let tam = cc.len()-4;
    let cc = cc[tam-1..];
    let rest_cc = String::from(cc);

    if tam <= 0{
        return str;
    }

    let mut response: String = "#".repeat(tam);
    response.push_str(&rest_cc);

    return &response;
}

// Soluções possíveis

fn maskify(cc: &str) -> String {
    let mask_length = cc.len().saturating_sub(4);
    "#".repeat(mask_length) + &cc[mask_length..]
}

fn maskify(cc: &str) -> String {
    if cc.len() > 4{
        "#".repeat(cc.len()-4) + &cc[cc.len() - 4..]
    }else {
        cc.to_string()
    }
}

// APRENDIZADOS DO EX
// uma str pode ser concatenada por meio de:
"Irisson".to_owned() + &" Lima"

// também temos que 
let mask_length = cc.len().saturating_sub(4)
// é equivalente a 
let mask_length = cc.len() - 4