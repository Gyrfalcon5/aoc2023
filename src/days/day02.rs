use std::fs::read_to_string;
use std::cmp::max;

struct Contents {
    red: usize,
    green: usize,
    blue: usize,
}

struct Game {
    id: usize,
    rounds: Vec<Contents>,
}

impl Contents {
    fn possible(&self, compare: &Contents) -> bool {
        self.red <= compare.red
            && self.green <= compare.green
            && self.blue <= compare.blue
    }
}

impl Game {
    fn possible(&self, compare: &Contents) -> usize {
        for round in self.rounds.iter() {
            if !round.possible(&compare) {
                return 0
            }
        }
        self.id
    }

    fn power(&self) -> usize {
        let (mut max_red, mut max_blue, mut max_green) = (0, 0, 0);
        for round in self.rounds.iter() {
            max_red = max(max_red, round.red);
            max_blue = max(max_blue, round.blue);
            max_green = max(max_green, round.green);
        }
        max_red * max_blue * max_green
    }
}

pub fn solve(input_file: String) -> (String, String) {
    
    let mut result01 = 0;
    let mut result02 = 0;
    let standard = Contents {
        red: 12,
        green: 13,
        blue: 14,
    };
    let mut games: Vec<Game> = Vec::new();

    // This is a let binding to avoid weird lifetime issues
    let data = read_to_string(input_file).unwrap();
    let input = data.lines();
    for line in input {

        let mut rounds: Vec<Contents> = Vec::new();


        // By all rights this should be a function but I just don't feel like it
        let first_split: Vec<_> = line.split(": ").collect();
        let id: usize = first_split[0][5..].parse().unwrap();
        let second_split: Vec<_> = first_split[1].split("; ").collect();
        for split in second_split.iter() {
            let (mut red, mut green, mut blue) = (0, 0, 0);
            let items: Vec<_> = split.split(", ").collect();
            for item in items.iter() {
                let colors: Vec<_> = item.split(" ").collect();
                match colors[1] {
                    "green" => green = colors[0].parse().unwrap(),
                    "red" => red = colors[0].parse().unwrap(),
                    "blue" => blue = colors[0].parse().unwrap(),
                    _ => panic!("This shouldn't happen"),
                }
            }

            rounds.push(Contents {
                red: red,
                green: green,
                blue: blue,
            });
        }
        games.push(Game {
            id: id,
            rounds: rounds,
        });
    }

    for game in games.iter() {
        result01 += game.possible(&standard);
        result02 += game.power();
    }

    (format!("{result01}").to_string(), format!("{result02}").to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_01 () {
        let (result1, result2) = solve("input/test01_02.txt".to_string());
        assert_eq!(result1, "8");
        assert_eq!(result2, "2286");
    }
}
