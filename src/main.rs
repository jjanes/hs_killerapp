mod mixtape;

fn main() {
    let _input_file = std::env::args().nth(1)
        .expect(&show_usage("No input file given"));
    
    let _change_file = std::env::args().nth(2)
        .expect(&show_usage("No change file found"));
    
    let _output_file = std::env::args().nth(3)
        .expect(&show_usage("No ouput file param found"));

    let params: mixtape::MixtapeParams = mixtape::MixtapeParams {
        input_file: _input_file,
        change_file: _change_file,
        output_file: _output_file
    };
    
    mixtape::update(params);
        
    println!("[x] Wrote changes.");
}

fn show_usage(error: &str) -> String {
    format!("Usage: <input file> <change file> <output file>\n{}",error)
}
