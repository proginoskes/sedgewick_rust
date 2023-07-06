use std::env;
use std::fs;

mod algorithms{
    pub mod gcd;
}

fn main() -> Result<(),&'static str> {
    //println!("Hello, world!");
    let args : Vec<String> = env::args().collect();
    let file_flag = String::from("-f");
    //dbg!(args);

    let file_path : &String = args.iter()
        .skip_while(|arg| **arg != file_flag)
        .skip_while(|arg| **arg == file_flag)
        .next()
        .unwrap();


    let mut rel_path = env::current_dir()
        .expect("REASON");

    rel_path.push(file_path);

    
    println!("File path: {}", rel_path.display());

    let path_exists : bool = rel_path.exists();

    if !path_exists {
        println!("path dne");
        return Err("missing path");
    }

    let r_contents = fs::read_to_string(rel_path);
    
    let contents = match r_contents {
        Ok(file) => file,
        Err(error) => panic!(
            "could not open the file at .\\{} due to: {:?}", 
            file_path,
            error
        )
    };

    algorithms::gcd::euclid::run(&contents)
}
