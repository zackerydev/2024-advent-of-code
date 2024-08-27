use std::fs;

pub fn one() {
    // read in input file
    let content =
        fs::read_to_string("./src/one/input.txt").expect("Should have been able to read file");
    // split on new line
    let lines = content.split('\n');

    let mut sum: i32 = 0;

    for line in lines.into_iter() {
        let chars = line.split("");
        let first: i32;
        let last: i32;

        // handle words of numbers

        let line = line.to_lowercase();

        // match line.to_lowercase() {
        //     x if x.contains("one") => {
        //         // index of one
        //         let one_idx = line.find("one").map(|i| i + 1).unwrap();
        //         println!("one {}", line);
        //         println!("one_idx: {}", one_idx);
        //     }
        //     String { .. } => {
        //         // first = 0;
        //         // last = 0;
        //     }
        // }
        //
        let nums = chars
            .flat_map(|c| c.parse::<i32>().ok())
            .collect::<Vec<i32>>();

        if nums.len() == 0 {
            continue;
        }

        first = nums[0];
        last = nums.last().copied().unwrap();

        let first_str = first.to_string();
        let last_str = last.to_string();

        let full_num = format!("{}{}", first_str, last_str);

        sum += full_num.parse::<i32>().unwrap();
    }

    println!("Sum: {}", sum);
}
