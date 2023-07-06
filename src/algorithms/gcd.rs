
pub mod euclid{
    fn find_gcd(p_u : &i32, p_v : &i32) -> Result<i32, &'static str>{
            let mut t : i32;
            let mut v : i32 = *p_v;
            let mut u : i32 = *p_u;
            while u > 0 {
                if u < v {
                    t = u;
                    u = v;
                    v = t;
                }
                u = u - v;
            }

            Ok(v)      
    }
    pub fn run(contents : &str) -> Result<(), &'static str>{
        for line in contents.lines(){
            let parts : Vec<i32> = line.split("/")
                .map(|s_num| {
                    let r_num = s_num.parse::<i32>();
                    match r_num {
                        Ok(num) => num,
                        Err(error) => panic!(
                            "could not parse number {} in file due to: {:?}",
                            s_num,
                            error
                        )
                    }
                }).collect();

            let r_gcd : Result<i32, &str> = find_gcd(
                &parts[0], 
                &parts[1]
            );

            let gcd = match r_gcd {
                Ok(res) => res,
                Err(err) => panic!(
                    "could not find lcd for line due to: {:?}",
                    err
                )
            };
            println!("-------------------");
            println!(
                "fraction: {} / {}",
                parts[0],
                parts[1]
            );
            println!("GCD: {}", gcd);
            println!(
                "simplified fraction: {} / {} ",
                parts[0]/gcd,
                parts[1]/gcd
            );
        }

        println!("-------------------");
        Ok(())
    }

}
