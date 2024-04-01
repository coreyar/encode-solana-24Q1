fn main() {
    println!("Hello, world!");
    fizz_buzz();
}

fn fizz_buzz() -> u32 {
    let mut fizz_buzz = 0u32;
    for i in 1..302 {
        match i {
            i if (i % 15 == 0) => {
                println!("{}", "fizz buzz");
                fizz_buzz += 1;
            }
            i if (i % 3 == 0) => {
                println!("{}", "fizz")
            }
            i if (i % 5 == 0) => {
                println!("{}", "buzz")
            }
            _ => {}
        }
    }
    println!("fizz buzz occurred {} times", fizz_buzz);
    return fizz_buzz;
}

#[cfg(test)]
mod tests {
  use super::*;
    #[test]
    fn it_works() {
        assert_eq!(fizz_buzz(), 20);
    }
}
