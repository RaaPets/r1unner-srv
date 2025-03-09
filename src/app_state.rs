use std::sync::RwLock;

//  //  //  //  //  //  //  //
pub struct AppState {
    pub runner: RwLock<crate::liba::Runner>,
}

impl AppState {
    pub fn new() -> Self {
        let mut runner = crate::liba::Runner::new();
        runner.insert_via_id_info("one-1");
        runner.insert_via_id_info("two-2");
        println!("\n[+]AppState\n");
        Self {
            runner: RwLock::new(runner),
        }
    }
}
