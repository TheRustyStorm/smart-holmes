use rand::Rng;

#[derive(Debug)]
struct MarkovState{
    pub transition_propabilities: Vec<usize>,
}

impl MarkovState{
    fn new(amount_states: usize) -> MarkovState{
        let mut transition_propabilities: Vec<usize> = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..amount_states{
            transition_propabilities.push(rng.gen_range(0..10)*rng.gen_range(0..10));
        }
        MarkovState{transition_propabilities}
    }
}

#[derive(Debug)]
struct MarkovModel{
    states: Vec<MarkovState>,
    current_state_index: usize,
}

impl MarkovModel{
    pub fn new(amount_states: usize) -> MarkovModel{
        let mut states = Vec::new();
        for _ in 0..amount_states{
            let state = MarkovState::new(amount_states);
            states.push(state);
        } 
        let current_state_index = 0;
        MarkovModel{states, current_state_index}
    }

    pub fn step(&mut self){
        let transitions = &self.states.get(self.current_state_index).unwrap().transition_propabilities;
        let mut sum:usize = transitions.iter().sum();
        let mut rng = rand::thread_rng();
        let new_index = rng.gen_range(0..sum);
        let transitions = &self.states.get(self.current_state_index).unwrap().transition_propabilities;
        for (index, value) in transitions.iter().enumerate(){
            if *value >= new_index {
                self.current_state_index = index;
                break;
            }
            sum -= value;
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::markov_model::*;
   
    #[test]
    fn test_init_markov_state(){
        let _ = MarkovState::new(10);
    }

    #[test]
    fn test_init_markov_model(){
        let mut markov_model = MarkovModel::new(4);
        for _ in 0..100{
            markov_model.step();
            println!("{}",markov_model.current_state_index);
        }
    }
}