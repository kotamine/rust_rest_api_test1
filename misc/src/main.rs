fn main() {
    println!("Hello, world!");
    let mut tokens = vec![1, 2, 3, 4, 5];
    let context_size = 10;
 
    let ctxt = &tokens[tokens.len().saturating_sub(context_size)..];
    println!("ctxt: {:?}", ctxt);

    let context_size2 = 3;
    let ctxt2 = &tokens[tokens.len().saturating_sub(context_size2)..];
    println!("ctxt2: {:?}", ctxt2);
}

