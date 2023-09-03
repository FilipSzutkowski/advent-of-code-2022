use std::fs::read_to_string;

struct TopList {
    list: Vec<usize>,
}

impl TopList {
    fn try_inserting(&mut self, number: usize) {
        if let Some(pos) = self.list.iter().position(|nr| number >= *nr) {
            self.list.insert(pos, number);
            self.list.pop();
        }
    }

    fn get_total(&mut self) -> usize {
        self.list.iter().fold(0, |acc, elm| acc + elm)
    }
}

impl Default for TopList {
    fn default() -> Self {
        Self { list: vec![0; 3] }
    }
}

pub fn main() {
    const FILE_PATH: &str = "./inputs/day1";

    match read_to_string(FILE_PATH) {
        Ok(text) => handle_input(text),
        Err(err) => println!("Panicked lolx /n{err}"),
    }
}

fn handle_input(text: String) {
    let mut top_list = TopList::default();
    let mut temp_cals: usize = 0;

    text.lines().enumerate().for_each(|(line_nr, line_txt)| {
        if line_txt != "" {
            temp_cals = match line_txt.parse::<usize>() {
                Ok(calories) => temp_cals + calories,
                Err(_) => panic!("Could not parse line nr {line_nr}"),
            }
        } else {
            top_list.try_inserting(temp_cals);
            temp_cals = 0;
        }
    });

    println!("calories: {:?}", top_list.get_total());
}
