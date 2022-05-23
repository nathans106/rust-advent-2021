pub type Num = i32;
pub(crate) type Nums = Vec<Vec<Num>>;

pub struct Card {
    nums: Nums,
    marked: Vec<Vec<bool>>,
    last_called: Num,
}

impl Card {
    pub fn new(nums: Nums) -> Self {
        Self {
            marked: vec![vec![false; nums[0].len()]; nums.len()],
            nums: nums,
            last_called: 0,
        }
    }

    pub fn score(&self) -> Num {
        let mut score = 0;

        for (c_i, col) in self.marked.iter().enumerate() {
            for (r_i, is_marked) in col.iter().enumerate() {
                if !is_marked {
                    score += self.nums[c_i][r_i];
                }
            }
        }

        score = score * self.last_called;
        return score;
    }

    pub fn call(&mut self, called: Num) -> bool {
        let marked = self.mark(&called);
        if !marked {
            return false;
        }

        let has_bingo = self.bingo();
        self.last_called = called;
        return has_bingo;
    }

    fn bingo(&self) -> bool {
        for col in &self.marked {
            if !col.contains(&false) {
                return true;
            }
        }

        let num_rows = self.marked.first().unwrap().len();
        for r in 0..num_rows {
            if self.row_bingo(&r) {
                return true;
            }
        }

        return false;
    }

    fn row_bingo(&self, row_num: &usize) -> bool {
        for col in &self.marked {
            if col[*row_num] == false {
                return false;
            }
        }

        return true;
    }

    fn mark(&mut self, called: &Num) -> bool {
        for (r_i, col) in self.nums.iter().enumerate() {
            for (c_i, num) in col.iter().enumerate() {
                if num == called {
                    self.marked[r_i][c_i] = true;
                    return true;
                }
            }
        }

        return false;
    }
}
