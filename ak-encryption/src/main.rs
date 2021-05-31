mod rot13;

use std::fs::File;
use std::io::{Read, Write};
use std::env;

/*
& vs ref
- & denotes that your pattern expects a reference to an object. Hence & is a part of said pattern: &Foo matches different objects than Foo does.
- ref indicates that you want a reference to an unpacked value. It is not matched against: Foo(ref foo) matches the same objects as Foo(foo).
*/

fn main() {
    // acces the command line arguments and puts them in a vector for then iterate throw them
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Provide a file to convert.\nUsage {} filename", args[0]);
    } else {
        // making it "ref" so that we can acces it after being in a match statement (when using strings)
        let ref filename = args[1];

        match File::open(filename) {
            Ok(mut file) => {

                let mut content = String::new();
                file.read_to_string(&mut content).unwrap();

                let converted = rot13::rot13(&content);

                // create opens file as write-only, deletes existing content
                match File::create(filename) {
                    Err(e) => println!("Couldn't write to {}: {}", filename, e),

                    Ok(mut file) => {
                        match file.write_all(converted.as_bytes()) {
                            Err(e) => println!("Couldn't write to {}, {}", filename, e),
                            Ok(_) => println!("Converted {}", filename) 
                        };
                    }
                };
                
            },
            Err(e) =>println!("Error opening file {}: {}", filename, e)
        };
    }

}
