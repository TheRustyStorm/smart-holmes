use rand::Rng;

#[derive(Debug)]
pub struct MarkovState {
    pub transition_propabilities: Vec<usize>,
}

impl MarkovState {
    fn new(amount_states: usize) -> MarkovState {
        let mut transition_propabilities: Vec<usize> = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..amount_states {
            let mut probability = rng.gen_range(0..10);
            if probability < 6 {
                probability = 0;
            } else {
                probability -= 6;
            }
            transition_propabilities.push(probability);
        }
        let sum: usize = transition_propabilities.iter().sum();
        if sum == 0 {
            //if actually no transition occurs
            let max = transition_propabilities.len();
            transition_propabilities[rng.gen_range(0..max)] = 1;
        }
        MarkovState {
            transition_propabilities,
        }
    }
}

#[derive(Debug)]
pub struct MarkovModel {
    pub states: Vec<MarkovState>,
    pub current_state_index: usize,
}

impl MarkovModel {
    pub fn new(amount_states: usize) -> MarkovModel {
        let mut states = Vec::new();
        for _ in 0..amount_states {
            let state = MarkovState::new(amount_states);
            states.push(state);
        }
        let current_state_index = 0;
        MarkovModel {
            states,
            current_state_index,
        }
    }

    pub fn step(&mut self) {
        let transitions = &self
            .states
            .get(self.current_state_index)
            .unwrap()
            .transition_propabilities;

        let lotteryrange: usize = transitions.iter().sum();
        let mut rng = rand::thread_rng();
        let mut winning_ticket = rng.gen_range(0..lotteryrange);

        let transitions = &self
            .states
            .get(self.current_state_index)
            .unwrap()
            .transition_propabilities;
        for (index, value) in transitions.iter().enumerate() {
            if *value >= winning_ticket {
                self.current_state_index = index;
                break;
            }
            winning_ticket -= value;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::markov_model::*;

    #[test]
    fn test_init_markov_state() {
        let _ = MarkovState::new(10);
    }

    #[test]
    fn test_init_markov_model() {
        let mut markov_model = MarkovModel::new(4);
        for _ in 0..10 {
            markov_model.step();
        }
    }

    #[test]
    fn test_dependency_markov_model() {
        let mut markov_model = MarkovModel::new(6);
        markov_model.states[1].transition_propabilities[2] = 999;
        for _ in 0..100 {
            markov_model.step();
            println!("{}", markov_model.current_state_index);
        }
    }
}
