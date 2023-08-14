use rand::Rng;
use std::fmt::Debug;

//Sampling from a discrete distribution.
//Read more here:
//https://www.keithschwarz.com/darts-dice-coins/
//
//The alias method has substantial initialization cost according to
//https://bugs.python.org/msg197540, so we use "Roulette Wheel",
//i.e. binary search on cumulative sums of the weights.
#[derive(Debug)]
pub struct Lottery<'a, T: Copy> {
    rng: &'a mut rand::rngs::ThreadRng,
    possible_outcomes: Vec<T>,
    //A weight is the ratio of an outcome's chance to be chosen to another outcome's chance to be
    //chosen. E.g. if pumpkin has weight 3 and lemon has weight 1, pumpkin is 3x more likely to be
    //chosen over lemon. An outcome's weight is never 0.
    weights: Vec<usize>,
    //An internal value that is never made known to the user for the sake of simplicity.
    //Denotes whether cumulative_weights can be used for choose_with_bias(), that is,
    //successive calls to choose_with_bias() will use the same cumulative_weights.
    is_compiled: bool,
    cumulative_weights: Vec<usize>,
}

impl<T: Copy + Debug> Lottery<'_, T> {
    pub fn build(rng: &mut rand::rngs::ThreadRng) -> Lottery<T> {
        Lottery {
            rng,
            possible_outcomes: Vec::new(),
            weights: Vec::new(),
            is_compiled: false,
            cumulative_weights: Vec::new(),
        }
    }

    //Add a T to the lottery.
    pub fn add(&mut self, outcome: T) {
        self.is_compiled = false;
        self.possible_outcomes.push(outcome);

        let default_bias: usize = 1;
        self.weights.push(default_bias);
    }

    fn is_unbiased(&self) -> bool {
        self.weights.len() == 0
    }

    //weights must be empty.
    fn gen_default_bias(&mut self) {
        let default_bias: usize = 1;

        self.weights.resize(self.possible_outcomes.len(), default_bias);
    }

    pub fn add_with_bias(&mut self, outcome: T, bias: usize) {
        self.is_compiled = false;
        if self.is_unbiased() {
            self.gen_default_bias();
        }

        if bias == 0 {
            panic!("Bias must never be zero.");
        }

        self.possible_outcomes.push(outcome);
        self.weights.push(bias);

        //At this point, there may be a common denominator between each weight in weights,
        //but we choose not to care about that.
    }

    pub fn choose_without_bias(&mut self) -> T {
        //Technically we are unbiased if len == 0.
        //Unbiased Lotteries always use choose_without_bias, so always panics if len == 0.
        if self.possible_outcomes.len() == 0 {
            panic!("Attempted to choose() a Lottery without any possible outcomes.");
        }

        let num = self.rng.gen_range(0..self.possible_outcomes.len());

        return self.possible_outcomes[num];
    }

    fn compile_cumulative_weights(&mut self) {
        self.is_compiled = true;

        self.cumulative_weights.resize(self.weights.len(), 0);

        //Here we would check if weights.len() == 0,
        //but we know we are biased already.
        self.cumulative_weights[0] = self.weights[0];
        let mut sum = self.cumulative_weights[0];

        for i in 1..self.weights.len() {
            sum += self.weights[i];
            self.cumulative_weights[i] = sum;
        }

        //Already sorted.
        //self.cumulative_weights.sort();
    }

    pub fn choose(&mut self) -> T {
        if self.is_unbiased() {
            return self.choose_without_bias();
        }

        if !self.is_compiled {
            self.compile_cumulative_weights();
        }

        //Binary search to the upper-bound.
        let target = self.rng.gen_range(0..*self.cumulative_weights.last().expect("Should be unbiased, hence len > 0.")) + 1;
        let mut left = 0;
        let mut right = self.cumulative_weights.len() - 1;
        let mut mid;

        while left < right {
            mid = left + (right - left) / 2;

            //Leave off this bit can't be bothered to time efficiency.
            //if self.cumulative_weights[mid] == target
            if self.cumulative_weights[mid] < target {
                left = mid + 1;
            }
            else {
                right = mid;
            }
        }

        return self.possible_outcomes[left];
    }
}

#[cfg(test)]
mod choose_amongst_tests {
    use super::*;

    #[test]
    #[should_panic]
    fn choosing_none() {
        let mut rng = rand::thread_rng();
        let mut lot: Lottery<'_, f32> = Lottery::build(&mut rng);
        let num: f32 = lot.choose();

        assert!(true);
    }
}
