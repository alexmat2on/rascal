use std::fs::File;
use std::io::Read;

pub fn scanfile(filename : &String) {
    let mut buffer = vec![];

    let mut f = File::open(filename).unwrap();
    f.read_to_end(&mut buffer);

    for b in buffer {
        println!("{}\n", b);
    }

    // while let Ok(bytes_read) = io::stdin().read(&mut buffer) {
    //     if bytes_read == 0 { break; }
    //     process(&buffer[..bytes_read]).unwrap();
    // }
    // Ok(());
}
