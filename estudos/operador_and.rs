fn main() {
    let x = 10;
    let y = 0;
    if x > 0 && y / x > 0 {
        println!("Esta mensagem não será exibida, pois y / x não será avaliado");
    }
    if x > 0 & y / x > 0 {
        println!("Esta mensagem não será exibida, pois y / x resultará em um erro de divisão por zero");
    }
}

// O operador && é conhecido como "and de curto-circuito" (short-circuit and), o que significa que se a primeira expressão for falsa,
// a segunda expressão não será avaliada. Isso ocorre porque, se a primeira expressão é falsa, a expressão inteira já é falsa, 
// independentemente do valor da segunda expressão. Isso pode ajudar a melhorar o desempenho e evitar possíveis erros.

// Já o operador & é um "and lógico" (logical and) padrão, que sempre avalia as duas expressões, independentemente de a primeira expressão
// ser verdadeira ou falsa. Isso pode ser útil em algumas situações, mas em outras pode levar a resultados inesperados.