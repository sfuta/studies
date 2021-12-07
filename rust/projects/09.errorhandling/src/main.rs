use std::io;
use std::io::Read;
use std::fs::File;

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


fn _use_unwrap () {
    let _f = File::open("hello.txt").unwrap();
}

fn _use_expect() {
    let _f = File::open("hello.txt").expect("Failed open file. [hello.txt]");
}

fn _error_transfer() {
    fn read_file_h_txt() -> Result<String, io::Error> {
        let f = File::open("h.txt");
        let mut f = match f {
            Ok(file) => file, Err(e) => return Err(e),
        };
        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s), Err(e) => Err(e)
        }
    }

    fn read_file_a_txt() -> Result<String, io::Error> {
        let mut f = File::open("a.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }

    fn read_file_b_txt() -> Result<String, io::Error> {
        let mut s = String::new();
        File::open("b.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }

    let _f = read_file_b_txt().expect("Failed read_file_b_txt");
    let _f = read_file_a_txt().expect("Failed read_file_a_txt");
    let _f = read_file_h_txt().unwrap();

    // NG: '?' is available, when function returns 'Result'
    // let _f = File::open("c.txt")?;
}

fn main() {
    // _out_of_bound();
    // _panic();
    // _use_result();
    // _errors_match();
    // _use_unwrap();
    // _use_expect();
    _error_transfer();
}
