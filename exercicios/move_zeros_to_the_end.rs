// Write an algorithm that takes an array and moves all of the zeros to the end, preserving the order of the other elements.

// moveZeros([false,1,0,1,2,0,1,3,"a"]) // returns[false,1,1,2,1,3,"a",0,0]

fn move_zeros(arr: &[u8]) -> Vec<u8> {
    let vec = arr.iter().cloned()
                .filter(|&x| x!=0)
                .collect::<Vec<u8>>();
    let n0 = arr.len()-vec.len();
    
    vec.into_iter().chain(vec![0; n0]).collect()
}

fn main(){
    println!("{:?}", move_zeros(&[1,0,2,0,3,0]))
}

// comunidade

fn move_zeros_comunidade(xs: &[u8]) -> Vec<u8> {
    let mut ys = Vec::with_capacity(xs.len());
    ys.extend(xs.iter().filter(|&&x| x != 0));
    ys.resize(xs.len(), 0);
    ys
}

// A diferença entre iter() e into_iter() é que o primeiro retorna um iterador que produz referências para os elementos da coleção,
// enquanto o segundo retorna um iterador que produz os próprios elementos. Portanto, o iter() é usado para iterar sobre uma coleção
// sem modificar seus elementos, enquanto o into_iter() é usado para consumir a coleção e produzir seus elementos diretamente.