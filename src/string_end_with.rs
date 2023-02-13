fn solution (word: &str, ending: &str)->bool{

    let e_len = ending.len();
    let w_len = word.len();

    if e_len > w_len{
        return false;
    }

    let compatible = &word[w_len-e_len..];

    if compatible == ending{
        return true;
    }
    return false;
}

// Soluções mais elegantes 

fn solution_comunidade_1(word: &str, ending: &str) -> bool {
    if word.len() < ending.len() {
        return false;
    }
    &word[word.len()-ending.len()..] == ending

}

fn solution_comunidade_2(word: &str, ending: &str) -> bool {
    let res = word.ends_with(ending);
    res
  }