use std::str::FromStr;


struct Equation {
    left: String,
    rigth: String,
    is_equal: bool,
}

struct ParseEquationError;

impl FromStr for Equation {
    type Err = ParseEquationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once("=").and_then(|(l, r)| {
            let letf = sum(l);
            let right = sum(r);

            Some(Equation {
                left: l.trim().to_string(),
                rigth: r.trim().to_string(),
                is_equal: letf == right,
            })
        }).ok_or(ParseEquationError)
    }
}

fn test_eq(s: &str) {
    match s.parse::<Equation>() {
        Ok(Equation { left, rigth, is_equal }) => {
            if is_equal {
                println!("{} does equal {}", left, rigth);
            } else {
                println!("{} does not equal {}", left, rigth);
            }
        },
        Err(_) => println!("Boom"),
    }
}


fn sum (s: &str) -> usize {
    s.split("+").map(|x| x.trim().parse::<usize>().unwrap_or(0)).sum()
}

fn get_number(s: &str) -> Result<u32, String> {
 return match  s.parse::<u32>() {
       Ok(n) => Ok(n),
       Err(_) => Err("Not a number".to_string()),
   };

}

fn main() {
    test_eq("1 + 1 = 2");
    test_eq("3 + 2 = 4 + 1");
    let array = [1, 2, 3];

    let hello = 123;

    if hello == 123 {
        println!("Hello");
    }

    let increment_array = array.map(|x| {
        match x {
            1 => 9,
            2 => 5,
            3 => 0,
            _ => 0,
            
        }
    });

    println!("{:?}", increment_array);

    let result = get_number("1");
    match result {
        Ok(n) => println!("Number: {}", n),
        Err(e) => println!("Error: {}", e),
    }
}


