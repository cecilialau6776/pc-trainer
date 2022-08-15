use crate::solver::SolverState;

pub struct HistoryManager {
    states: Vec<SolverState>,
}

impl HistoryManager {
    pub fn new() -> HistoryManager {
        HistoryManager { states: Vec::new() }
    }
    
    pub fn add(&mut self, state: SolverState) {
        self.states.push(state);
    }
}
