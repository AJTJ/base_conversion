use std::io;

struct InputData {
    input_num_as_str: String,
    input_num: usize,
    input_base: usize,
    output_base: usize,
}

fn check_response(response: &str) -> Option<InputData> {
    let split_response = response.split(" ");
    let mut nums = vec![];
    let mut first = true;
    for s in split_response {
        match s.trim().parse::<usize>() {
            Ok(n) => nums.push(n),
            Err(e) => return None,
        }
    }
    if nums.len() != 3 {
        return None;
    }

    Some(InputData {
        input_num_as_str: nums[0].to_string(),
        input_num: nums[0],
        input_base: nums[1],
        output_base: nums[2],
    })
}

fn main() {
    println!("TEST HERE");

    let mut num = 12534;
    let base = 10;
    while num > 0 {
        let digit = num % base;
        num = num / base;
        println!("{}", digit);
    }

    loop {
        println!("Input format: input number base conversion base");
        println!("example: 11011 2 4");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("input Error");

        let mut val: usize = 0;
        if let Some(i) = check_response(&input) {
            if i.input_base > 10 || i.output_base > 10 {
                println!("base too large");
                break;
            }

            // confirm that input num is of the correct base
            for c in i.input_num_as_str.chars() {
                if !c.is_digit(i.input_base as u32) {
                    println!("not of base");
                    break;
                }
            }

            let mut working_num = i.input_num.clone();

            while working_num > 0 {
                let digit = working_num % 10;

                // the following is true, but we need to convert it to the new base

                let x = digit * i.input_base

                val = val + (digit * i.input_base);
                working_num = working_num / 10;
            }
        } else {
            println!("input error");
            break;
        }

        println!("the new number: {val}");
        break;
    }
}
