use std::env;
use std::fs;
use std::sync::Arc;

fn euclid(mut p_u: Arc<i32>, mut p_v: Arc<i32>) -> Result<i32, &'static str> {
    let mut t : i32;

    let mut u : i32 = *(match Arc::get_mut(&mut p_u) {
        Some(i_u) => i_u,
        None => return Err("could not find denomenator")
    });

    let mut v : i32 = *(match Arc::get_mut(&mut p_v) {
        Some(i_v) => i_v,
        None => return Err("could not find numerator")
    });

    while u > 0 {
        if u < v {
            t = u;
            u = v;
            v = t;
        }
        u = u-v;
    }

    Ok(v)
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


    for line in contents.lines(){
        let parts : Vec<i32> = line.split("/")
            .map(
                |s_num| {
                    let r_num = s_num.parse::<i32>();
                    match r_num {
                        Ok(num) => num,
                        Err(error) => panic!(
                            "could not parse number {} in file due to: {:?}",
                            s_num,
                            error
                        )
                    }
                }
            ).collect();
        //dbg!(parts);
        let r_gcd : Result<i32, &str> = euclid(
            Arc::<i32>::new(parts[0]), 
            Arc::<i32>::new(parts[1])
        );
        let gcd = match r_gcd {
            Ok(res) => res,
            Err(err) => panic!(
                "could not find lcd for line due to: {:?}",
                err
            )
        };
        dbg!(parts, gcd);
    }

    Ok(())

}
