use std::fs::File;
use std::future::pending;
use std::io::read_to_string;
use regex::Regex;

fn main() {
    let reg = Regex::new(r"(?<firstnum>[0-9]+)-(?<secondnum>[0-9]+)").unwrap();
    let mut sum = 0;


    let file = File::open("input.txt").unwrap();
    let new_string = read_to_string(file).unwrap();
    let matches = reg.captures_iter(new_string.as_str());

    let _ = matches.for_each(|caps| {
        let first_num = &caps["firstnum"].parse::<u64>().unwrap();
        let second_num = &caps["secondnum"].parse::<u64>().unwrap();
        iterate_over_each_value_part2(first_num, second_num, &mut sum);
    }
    );
    // println!("{}", new_string);
    println!("New Sum: {}", sum);

}

fn iterate_over_each_value(first_num: &u64, second_num: &u64, sum: &mut u64) {
    let mut counter = first_num.clone();
    while counter <= *second_num {
        let string_value = counter.to_string();
        let length = string_value.len();
        if &length % 2 != 0 {
            counter += 1;
            continue;
        }
        let first_half = &string_value[0..length/2].parse::<i32>().unwrap();
        let second_half = &string_value[length/2..length].parse::<i32>().unwrap();
        // println!("Counter: {} | Length: {} | Halves: {} {}", counter, length, first_half, second_half);
        if first_half == second_half {
            *sum += counter;
        }
        counter += 1;
    }
}

fn iterate_over_each_value_part2(first_num: &u64, second_num: &u64, sum: &mut u64) {
    let mut counter = *first_num;

    while counter <= *second_num {
        let string_value = counter.to_string();
        let length = string_value.len();
        let mut starting_counter = 1;


        while starting_counter <= length/2 {
            let mut is_added_to_sum = true;
            if length % starting_counter != 0 {
                starting_counter += 1;
                continue;
            }

            let first_part = &string_value[0..starting_counter];

            let mut iteration = 1;
            while iteration*starting_counter < length {
                let sub_part = &string_value[iteration*starting_counter..starting_counter*(iteration+1)];


                // println!("{} | {} | {}", counter, first_part, sub_part);
                if first_part != sub_part {
                    is_added_to_sum = false;
                    break;
                }
                iteration += 1;


            }
            if is_added_to_sum {
                // println!("VALID {}", counter);
                *sum += counter;
                break;
            }



            starting_counter += 1;
        }


        counter += 1;
    }
}
