use std::sync::RwLock;

//  //  //  //  //  //  //  //
pub struct AppState {
    pub runner: RwLock<crate::liba::Runner>,
}

impl AppState {
    pub fn new() -> Self {
        let mut runner = crate::liba::Runner::default();
        runner.insert("one-1").unwrap();
        runner.insert("two-2").unwrap();
        println!("\n[+]AppState\n");
        Self {
            runner: RwLock::new(runner),
        }
    }
}
