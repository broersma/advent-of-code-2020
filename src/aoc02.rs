use std::str::FromStr;
use regex::Regex;

#[derive(Debug, PartialEq)]
struct PolicyPassword {
    min: u8,
    max: u8,
    letter: String,
    password: String,
}

impl FromStr for PolicyPassword {
    type Err = String;

    fn from_str(policy_password_str: &str) -> Result<Self,Self::Err> {
        let re = Regex::new(
            r"(\d+)\-(\d+) ([a-z]): ([a-z]+)",
        ).unwrap();
    
        re.captures(policy_password_str).and_then(|cap| {
            Some(PolicyPassword {
                min: cap.get(1).unwrap().as_str().parse::<u8>().unwrap(),
                max: cap.get(2).unwrap().as_str().parse::<u8>().unwrap(),
                letter: cap.get(3).unwrap().as_str().to_string(),
                password: cap.get(4).unwrap().as_str().to_string()
            })
        }).ok_or("Not good.".to_string())
    }
}

fn part1(input : &Vec<PolicyPassword>) {
    for x in input {
        println!("{:?}", x);
    }
}

fn part2(input : &Vec<PolicyPassword>) {
    for x in input {
        println!("{:?}", x);
    }
}

pub fn main() {
    let input: Vec<PolicyPassword> = "1-3 a: abcde
    1-3 b: cdefg
    2-9 c: ccccccccc".split("\n").map(|x| x.trim()).map(|x| x.parse::<PolicyPassword>().unwrap()).collect();

    part1(&input);
    part2(&input);
}
