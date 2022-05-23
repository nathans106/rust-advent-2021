use crate::card::{self, Card};

pub fn play(card_nums: Vec<card::Nums>, calls: &Vec<card::Num>) -> Vec<(usize, card::Num)> {
    let mut cards: Vec<Card> = card_nums.into_iter().map(|nums| Card::new(nums)).collect();
    let mut winners: Vec<(usize, card::Num)> = vec![];

    for call in calls {
        for (i, card_) in cards.iter_mut().enumerate() {
            if winners.iter().any(|(win_i, _score)| win_i == &i) {
                continue;
            }

            let has_won = card_.call(call.clone());
            if has_won {
                let score = card_.score();
                winners.push((i, score));
            }
        }
    }

    return winners;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let calls = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];

        let cards = vec![
            vec![
                vec![22, 13, 17, 11, 0],
                vec![8, 2, 23, 4, 24],
                vec![21, 9, 14, 16, 7],
                vec![6, 10, 3, 18, 5],
                vec![1, 12, 20, 15, 19],
            ],
            vec![
                vec![3, 15, 0, 2, 22],
                vec![9, 18, 13, 17, 5],
                vec![19, 8, 7, 25, 23],
                vec![20, 11, 10, 24, 4],
                vec![14, 21, 16, 12, 6],
            ],
            vec![
                vec![14, 21, 17, 24, 4],
                vec![10, 16, 15, 9, 19],
                vec![18, 8, 23, 26, 20],
                vec![22, 11, 13, 6, 5],
                vec![2, 0, 12, 3, 7],
            ],
        ];

        let (_winner, score) = play(cards, &calls).unwrap();
        assert_eq!(score, 4512);
    }
}
