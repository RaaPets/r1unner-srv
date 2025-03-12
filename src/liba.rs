use std::collections::HashMap;
use crate::error::RunnerError;

//  //  //  //  //  //  //  //
pub struct Runner {
    counter: usize,
    list: HashMap<usize, String>,
}

impl Runner {
    pub fn new() -> Self {
        let list = HashMap::new();
        Self {
            counter: 0,
            list,
        }
    }

    pub fn get(&self, id: usize) -> Result<&String, RunnerError > {
        let Some(res) = self.list.get(&id) else {
            return Err(RunnerError::WrongId);
        };

        Ok(res)
    }

    pub fn insert(&mut self, info: &str) -> usize {
        self.counter += 1;
        self.list.insert(self.counter, info.to_string());
        return self.counter
    }

    pub fn remove(&mut self, id: usize) -> Result<String, RunnerError> {
        let Some(res) = self.list.remove(&id) else {
            return Err(RunnerError::WrongId);
        };

        Ok(res)
    }

    pub fn get_list(&self) -> String {
        let mut text = String::new();
        for (id, info) in &self.list {
            text += &format!("id({}) <{}>\n", id, info);
        }

        text
    }
}

//  //  //  //  //  //  //  //
//        TEST              //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod runner_response_tests {
    use super::*;

    #[test]
    fn deletion() {
        let mut new = Runner::new();
        new.insert("one");
        new.insert("two");
        new.remove(1).unwrap();
        let id3 = new.insert("three");
        assert!(id3 == 3);
        let response = new.get(2).unwrap();
        assert!(response == "two");
        let response_none = new.get(1);
        assert!(response_none.is_err());
    }

    #[test]
    fn get_item() {
        let mut new = Runner::new();
        new.insert("one");
        new.insert("two");
        new.insert("three");
        let response = new.get(2).unwrap();
        assert!(response == "two");
    }

    #[test]
    fn insertion() {
        let mut new = Runner::new();
        new.insert("one");
        let response = new.get_list();
        assert!(response == "id(1) <one>\n");
    }

    #[test]
    fn create_empty() {
        let new = Runner::new();
        let response = new.get_list();
        assert!(response == "");
    }
}

#[cfg(test)]
mod runner_basic_tests {
    use super::*;

    #[test]
    fn deletion() {
        let mut new = Runner::new();
        new.insert("one");
        new.insert("two");
        assert!(new.remove(3).is_err());
        assert!(new.list.len() == 2);
        assert!(new.counter == 2);
        new.remove(2).unwrap();
        assert!(new.list.len() == 1);
        assert!(new.counter == 2);
    }

    #[test]
    fn insertion() {
        let mut new = Runner::new();
        let id1 = new.insert("one");
        assert!(id1 == 1);
        let id2 = new.insert("two");
        assert!(id2 == 2);
        assert!(new.list.len() == 2);
        assert!(new.counter == 2);
    }

    #[test]
    fn create_empty() {
        let new = Runner::new();
        assert!(new.list.len() == 0);
        assert!(new.counter == 0);
    }
}
