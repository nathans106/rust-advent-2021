use rust_advent_2021::{day::Day, input, problem::Problem};

mod bingo;
mod card;

struct BingoInput {
    card_nums: Vec<card::Nums>,
    calls: Vec<card::Num>,
}

struct BingoProblem {}
impl Problem<BingoInput> for BingoProblem {
    fn solve(
        &mut self,
        input: &BingoInput,
    ) -> Result<i32, rust_advent_2021::problem::ProblemError> {
        let result = bingo::play(input.card_nums.clone(), &input.calls);
        return Ok(result.unwrap().1.into());
    }
}

fn main() {
    let lines = input::parse_strs(&4);
    let input = parse_input(&lines);
    let mut day = Day::new(4, input);
    day.add_problem(BingoProblem {});
    day.solve();
}

fn parse_input(lines: &Vec<String>) -> BingoInput {
    let calls = lines[0]
        .split(',')
        .map(|num_str| num_str.parse::<card::Num>().unwrap())
        .collect();

    let mut card_nums: Vec<card::Nums> = vec![];

    let mut cur_card: card::Nums = vec![];
    for line in &lines[2..] {
        if line.is_empty() {
            card_nums.push(cur_card);
            cur_card = vec![];
        } else {
            let nums: Vec<card::Num> = line
                .split_ascii_whitespace()
                .map(|num_str| num_str.parse::<card::Num>().unwrap())
                .collect();
            cur_card.push(nums);
        }
    }

    BingoInput {
        card_nums: card_nums,
        calls: calls,
    }
}
