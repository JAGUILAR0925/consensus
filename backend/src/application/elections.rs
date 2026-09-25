use std::sync::{Arc, RwLock};
use crate::domain::{Election, Status};

#[derive(Clone, Default)]
pub struct ElectionService { state: Arc<RwLock<Option<Election>>> }
impl ElectionService {
    pub fn snapshot(&self) -> Option<Election> { self.state.read().unwrap().clone() }
    pub fn demo(&self) -> Election {
        let mut lock = self.state.write().unwrap();
        lock.get_or_insert_with(|| Election { id: "demo-001".into(), status: Status::Open, options: vec!["A".into(), "B".into()], votes: vec![] }).clone()
    }
}
