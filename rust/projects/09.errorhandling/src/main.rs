fn _panic() {
    panic!("Call panic!!");
}

fn _out_of_bound() {
    let v = vec![1, 2, 3];
    v[9];
}

fn _use_result() {
    let f = std::fs::File::open("hello.txt");
    let _f = match f {
        Ok(file) => file, Err(error) => { panic!("Failed open file: {:?}", error) }
    };
}

fn _errors_match() {
    let f = std::fs::File::open("hello.txt");
    let _f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == std::io::ErrorKind::NotFound => {
                match std::fs::File::create("Hello.txt") {
                    Ok(fc) => fc, Err(e) => { panic!("Faile create file: {:?}", e) }
                }
            },
        Err(error) => { panic!("Failed open file: {:?}", error) }
        
    };
}

fn main() {
    // _out_of_bound();
    // _panic();
    // _use_result();
    _errors_match();
}
