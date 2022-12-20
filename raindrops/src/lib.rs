pub fn raindrops(n: u32) -> String {
     let factors = get_factors(n);
     let mut result = String::from("");
     let mut count = 0;
     for factor in factors {
        if factor == 3 {
           result = result + "Pling";
           count=count+1;
        } else if factor == 5  {
            result = result + "Plang";
            count=count+1;
        } else if factor == 7 {
            result = result + "Plong";
            count=count+1;
        } else {
            
        }
     }
     if count ==0 {
        return n.to_string();
     }
     return result;
}

fn get_factors(n:u32) -> Vec<u32> {
    (1..n+1).into_iter().filter(|&x| n%x==0).collect()
}
