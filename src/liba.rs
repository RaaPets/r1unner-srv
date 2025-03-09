use std::collections::HashMap;

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

    pub fn get_via_id(&self, id: usize) -> Option<String> {
        self.list.get(&id).cloned()
    }

    pub fn insert_via_id_info(&mut self, info: &str) -> usize {
        self.counter += 1;
        self.list.insert(self.counter, info.to_string());
        return self.counter
    }

    pub fn remove_via_id(&mut self, id: usize) -> Option<String> {
        self.list.remove(&id)
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
        new.insert_via_id_info("one");
        new.insert_via_id_info("two");
        new.remove_via_id(1);
        let id3 = new.insert_via_id_info("three");
        assert!(id3 == 3);
        let response = new.get_via_id(2);
        assert!(response == Some("two".to_string()));
        let response_none = new.get_via_id(1);
        assert!(response_none == None);
    }

    #[test]
    fn get_item() {
        let mut new = Runner::new();
        new.insert_via_id_info("one");
        new.insert_via_id_info("two");
        new.insert_via_id_info("three");
        let response = new.get_via_id(2);
        assert!(response == Some("two".to_string()));
    }

    #[test]
    fn insertion() {
        let mut new = Runner::new();
        new.insert_via_id_info("one");
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
        new.insert_via_id_info("one");
        new.insert_via_id_info("two");
        new.remove_via_id(3);
        assert!(new.list.len() == 2);
        assert!(new.counter == 2);
        new.remove_via_id(2);
        assert!(new.list.len() == 1);
        assert!(new.counter == 2);
    }

    #[test]
    fn insertion() {
        let mut new = Runner::new();
        let id1 = new.insert_via_id_info("one");
        assert!(id1 == 1);
        let id2 = new.insert_via_id_info("two");
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
