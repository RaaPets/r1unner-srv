use mlua::prelude::*;

//  //  //  //  //  //  //  //
pub struct TaskHolder {
    //lua: mlua::Lua,
    info: String,
}
impl TaskHolder {
    pub fn new(info: &str) -> mlua::Result<Self> {
        let lua = init_lua(info)?;

        let instance = Self {
            //lua,
            info: info.to_string(),
        };

        Ok(instance)
    }

    pub fn info(&self) -> String {
        self.info.clone()
    }
}

//  //  //  //  //  //  //  //
fn init_lua(code: &str) -> mlua::Result<mlua::Lua> {
    let lua = Lua::new();
    lua.load(code).exec()?;

    Ok(lua)
}

// // // // // // // //
/*
fn work_lua(lua: &Lua, level_lua_code: &str) -> mlua::Result<()> {
    let globals = lua.globals();
    globals.set("alfa", "omega")?;
    let rst_fn = lua.create_function(|_, ()| {
        println!("from LUAAAAAAAA");
        Ok(())
    })?;
    globals.set("invokeRust", rst_fn)?;

    lua.load(level_lua_code).exec()?;
    let lua_b: String = globals.get("b")?;
    println!("lua B = {}", lua_b);

    let call_from_rust: Function = globals.get("fromRust")?;
    call_from_rust.call::<_>(())?;

    let call_from_rust_2: Function = globals.get("fromRust2")?;
    let res: u32 = call_from_rust_2.call::<_>("MegaKey")?;
    println!("from lua u32 = {}", res);

    Ok(())
}
*/
