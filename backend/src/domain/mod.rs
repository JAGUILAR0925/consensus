use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Election { pub id: String, pub status: Status, pub options: Vec<String>, pub votes: Vec<String> }
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Status { Draft, Open, Closed }
#[derive(Debug, Error)]
pub enum DomainError { #[error("election is not open")] NotOpen, #[error("voter already voted")] DuplicateVote, #[error("unknown option")] UnknownOption }

impl Election {
    pub fn cast(&mut self, voter: String, option: &str) -> Result<(), DomainError> {
        if self.status != Status::Open { return Err(DomainError::NotOpen); }
        if self.votes.iter().any(|v| v == &voter) { return Err(DomainError::DuplicateVote); }
        if !self.options.iter().any(|o| o == option) { return Err(DomainError::UnknownOption); }
        self.votes.push(voter); Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_duplicate_voter() {
        let mut e = Election { id: "t".into(), status: Status::Open, options: vec!["A".into()], votes: vec![] };
        assert!(e.cast("v1".into(), "A").is_ok());
        assert!(matches!(e.cast("v1".into(), "A"), Err(DomainError::DuplicateVote)));
    }
    #[test]
    fn rejects_vote_when_closed() {
        let mut e = Election { id: "t".into(), status: Status::Closed, options: vec!["A".into()], votes: vec![] };
        assert!(matches!(e.cast("v1".into(), "A"), Err(DomainError::NotOpen)));
    }
}
