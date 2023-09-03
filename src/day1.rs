use std::fs::read_to_string;

pub fn main() {
    const FILE_PATH: &str = "./inputs/day1";

    match read_to_string(FILE_PATH) {
        Ok(text) => handle_input(text),
        Err(err) => println!("Panicked lolx /n{err}"),
    }
}

fn handle_input(text: String) {
    let mut top1: usize = 0;
    let mut top2: usize = 0;
    let mut top3: usize = 0;
    let mut temp_cals: usize = 0;

    text.lines().enumerate().for_each(|(line_nr, line_txt)| {
        if line_txt != "" {
            temp_cals = match line_txt.parse::<usize>() {
                Ok(calories) => temp_cals + calories,
                Err(_) => panic!("Could not parse line nr {line_nr}"),
            }
        } else {
            match temp_cals >= top3 {
                true => match temp_cals >= top2 {
                    true => match temp_cals >= top1 {
                        true => {
                            top2 = top1;
                            top1 = temp_cals;
                        }
                        false => {
                            top3 = top2;
                            top2 = temp_cals;
                        }
                    },
                    false => top3 = temp_cals,
                },
                _ => (),
            }

            temp_cals = 0;
        }
    });

    println!("Highest calories: {top1}");
    println!("top2: {top2}");
    println!("top3: {top3}");
    println!("Sum top3 calories: {:?}", top1 + top2 + top3);
}
