use std::fs;

fn meets_criteria(pulledRed: i32, pulledGreen: i32, pulledBlue: i32) -> bool {
    let red = 12;
    let green = 13;
    let blue = 14;

    if pulledRed < red && pulledGreen < green && pulledBlue < blue {
        // println!("All colors valid");
        return true;
    } else {
        // println!("All colors not valid");
        return false;
    }
}

pub fn two() {
    let mut game_sum = 0;
    let content =
        fs::read_to_string("./src/two/input.txt").expect("Should have been able to read file");

    let games = content.split('\n');

    for game in games.into_iter() {
        println!("Game: {}", game);

        let mut game_is_valid = true;
        if game.len() == 0 {
            continue;
        }
        // remove "Game: " prefix
        let game = game.trim_start_matches("Game");
        // get game number
        let mut game_meta = game.split(":");
        let game_num = game_meta.next().unwrap().trim();
        let game_data = game_meta.next().unwrap().trim();
        // println!("Game: {}", game_num);
        // println!("Game Data: {}", game_data);

        let rolls = game_data.split(";");

        for roll in rolls.into_iter() {
            if game_is_valid == false {
                break;
            }

            // println!("Roll: {}", roll);
            let mut pulled_green = 0;
            let mut pulled_red = 0;
            let mut pulled_blue = 0;

            let pulls = roll.split(",");

            for pull in pulls.into_iter() {
                // println!("Pull: {}", pull);

                let mut pull_meta = pull.trim().split(" ");
                let pull_num = pull_meta.next().unwrap().trim();
                let pull_color = pull_meta.next().unwrap().trim();

                // println!("Pull Color: {}", pull_color);
                // println!("Pull Num: {}", pull_num);

                match pull_color {
                    "red" => pulled_red = pull_num.parse::<i32>().unwrap(),
                    "green" => pulled_green = pull_num.parse::<i32>().unwrap(),
                    "blue" => pulled_blue = pull_num.parse::<i32>().unwrap(),
                    _ => println!("Invalid color"),
                }

                // println!("Pulled Red: {}", pulled_red);
            }

            // println!("pull red: {}", pulled_red);
            // println!("pull green: {}", pulled_green);
            // println!("pull blue: {}", pulled_blue);

            if !meets_criteria(pulled_red, pulled_green, pulled_blue) && game_is_valid {
                println!("Game {} does not meet criteria", game_num);
                game_is_valid = false;
            }
        }

        if game_is_valid {
            game_sum += game_num.parse::<i32>().unwrap();
            println!(
                "All rolls valid Game {} meets criteria new sum {}",
                game_num, game_sum
            );
        }
    }

    println!("Game Sum: {}", game_sum);
}
