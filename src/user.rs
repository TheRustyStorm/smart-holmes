use super::markov_model::MarkovModel;
use super::smart_home::SmartHome;
use rand::Rng;

pub struct User{
    pub model: MarkovModel
}

impl User{
    pub fn new(smart_home: &SmartHome, amount_implicit_dependencies: usize) -> User{
        assert!(amount_implicit_dependencies < (smart_home.devices.len()/2));
        let mut model = MarkovModel::new(smart_home.devices.len());
        for i in 1..amount_implicit_dependencies+1{
            model.states[2*i].transition_propabilities[2*i+1] = 999999;
        }
        User{model}
    }

    pub fn choose_random_new_device(&mut self){
        let mut rng = rand::thread_rng();
        self.model.current_state_index = rng.gen_range(0..self.model.states.len());
    }

    pub fn step(&mut self){
        self.model.step();
    }

    pub fn currently_used_device(&self) -> usize{
        self.model.current_state_index
    }
}