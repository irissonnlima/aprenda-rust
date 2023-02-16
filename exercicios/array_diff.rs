fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let mut resp = Vec::new();
    for item in a{
        if !b.contains(&item){
            resp.push(item);
        }
    }
    return resp;
}

//funções da comunidade
fn array_diff_comunidade_1<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    a.into_iter().filter(|x| !b.contains(x)).collect()
}

fn array_diff_comunidade_2<T: PartialEq>(mut a: Vec<T>, b: Vec<T>) -> Vec<T> {
    a.retain(|x| !b.contains(x));
    a
}