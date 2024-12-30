use risc0_zkvm::guest::env;

fn main() {
    // TODO: Implement your guest code here

    // read the input
    // let input: u32 = env::read();

    // TODO: do something with the input

    // write public output to the journal
    // env::commit(&input);


    // let n = env::read();
    let n = 10;

    // Write n to public input
    // env::commit(&n);

    // Compute the n'th fibonacci number, using normal Rust code.
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let mut c = a + b;
        c %= 7919; // Modulus to prevent overflow.
        a = b;
        b = c;
    }

    // Write the output of the program.
    //
    // Behind the scenes, this also compiles down to a system call which handles writing
    // outputs to the prover.
    // env::commit(&a);
    // env::commit(&b);
    let result = format!("{}:{}:{}", n, a, b);
    env::commit(&result);
}
