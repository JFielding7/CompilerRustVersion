mod tokenizer;
mod ast;
mod line;
mod compiler_error;
mod ast_node;
mod data_type;

fn main() {
    const MIN_ARG_COUNT: usize = 2;
    let args = std::env::args().collect::<Vec<_>>();

    println!("args: {:?}", args);

    if args.len() < MIN_ARG_COUNT {
        panic!("{}: ERROR: No input files", args[0]);
    }

    for arg in args {

    }
}
