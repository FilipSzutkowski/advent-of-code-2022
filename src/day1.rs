use std::fs::read_to_string;

pub fn main() {
    const FILE_PATH: &str = "./inputs/day1";

    match read_to_string(FILE_PATH) {
        Ok(text) => handle_input(text),
        Err(err) => println!("Panicked lolx /n{err}"),
    }
}

fn handle_input(text: String) {
    let mut top_list: Vec<usize> = vec![0, 0, 0];
    let mut temp_cals: usize = 0;

    text.lines().enumerate().for_each(|(line_nr, line_txt)| {
        if line_txt != "" {
            temp_cals = match line_txt.parse::<usize>() {
                Ok(calories) => temp_cals + calories,
                Err(_) => panic!("Could not parse line nr {line_nr}"),
            }
        } else {
            if let Some(pos) = top_list.iter().position(|nr| temp_cals >= *nr) {
                top_list.insert(pos, temp_cals);
                top_list.pop();
            }

            temp_cals = 0;
        }
    });

    println!(
        "calories: {:?}",
        top_list.iter().fold(0, |acc, elm| acc + elm)
    );

    // println!("Highest calories: {top1}");
    // println!("top2: {top2}");
    // println!("top3: {top3}");
    // println!("Sum top3 calories: {:?}", top1 + top2 + top3);
}
