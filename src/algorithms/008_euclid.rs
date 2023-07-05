mod euclid {
    fn find_gcd(mut p_u : Arc<i32>, mut p_v Arc<i32>) -> Result<i32, &'static str>{
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

    pub fn run(String contents){
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
