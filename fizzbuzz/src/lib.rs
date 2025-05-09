
pub fn fizzbuzz_match() {
    for i in 1..=100{
        match i {
            n if (n % 3 == 0 && n % 5 == 0) => println!("FizzBuzz"),
            b if b % 5 == 0 => println!("Buzz"),
            a if a % 3 == 0 => println!("Fizz"),
            _ => println!("{}", i),
        }   
    }
}

fn function_fizzbuzz(i: &i32) -> &str {
    if i % 3 == 0 && i % 5 == 0 {
        return "FizzBuzz";
    } else if i % 5 == 0 {
        return "Buzz";
    } else if i % 3 == 0 {
        return "Fizz";
    } else {
        return "";
    }

}

pub fn fizzbuzz(){
    for i in 1..=100{
         if function_fizzbuzz(&i).is_empty() {
            println!("{}", i);
        } else {
            println!("{}", function_fizzbuzz(&i));
        }
    }
}


pub fn basic_fizzbuzz(){

    for i in 1..=100{
        if i % 3 == 0 && i % 5 == 0{
            println!("FizzBuzz");
        }else if i % 3 == 0{
            println!("Fizz");
            
        }else if i % 5 == 0 {
            println!("Buzz");
        }else {
            println!("{}",i);
        }

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizzbuzz_match() {
        fizzbuzz_match();
    }

    #[test]
    fn test_fizzbuzz() {
        fizzbuzz();
    }

    #[test]
    fn test_basic_fizzbuzz() {
        basic_fizzbuzz();
    }
}
