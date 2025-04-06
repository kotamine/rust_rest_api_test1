fn main() {
    println!("Hello, world!");
    let mut tokens: Vec<i32> = vec![1, 2, 3, 4, 5];
    let context_size = 10;
 
    let ctxt = &tokens[tokens.len().saturating_sub(context_size)..];
    println!("ctxt: {:?}", ctxt);

    let context_size2 = 3;
    let ctxt2 = &tokens[tokens.len().saturating_sub(context_size2)..];
    println!("ctxt2: {:?}", ctxt2);

    let mut tokens0: Vec<i32> = vec![6,7,8];
    let mut tokens2 = tokens.clone();
    tokens2.append(&mut tokens0);
    println!("tokens: {:?}", tokens);
    println!("tokens2: {:?}", tokens2);

    let mut tokens3: [&u32]= [&0, &1, &2];



}

