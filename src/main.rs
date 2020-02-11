mod mixtape;

fn main() {
    let _input_file = std::env::args().nth(1).expect("No input file given");
    let _change_file = std::env::args().nth(2).expect("No change file found");
    let _output_file = std::env::args().nth(3).expect("No ouput file param found");

    let params: mixtape::MixtapeParams = mixtape::MixtapeParams {
        input_file: _input_file,
        change_file: _change_file,
        output_file: _output_file
    };
    
    mixtape::update(params);
        
    println!("[x] Wrote changes.");
}

fn arg_get(arg: u8, error: String) {
}
