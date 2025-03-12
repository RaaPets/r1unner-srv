use std::sync::RwLock;

//  //  //  //  //  //  //  //
pub struct AppState {
    pub runner: RwLock<crate::liba::Runner>,
}

impl AppState {
    pub fn new() -> Self {
        let mut runner = crate::liba::Runner::new();
        runner.insert("one-1");
        runner.insert("two-2");
        println!("\n[+]AppState\n");
        Self {
            runner: RwLock::new(runner),
        }
    }
}
