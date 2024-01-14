use rlua::Lua;

fn main() {
    let lua = unsafe { Lua::unsafe_new_with_flags(rlua::StdLib::ALL, rlua::InitFlags::NONE) };
    lua.context(|lua| {
        lua.load(r#"
            local lrand = require('lrand')
            local state = lrand.new()
            print(state:randu64())"#)
            .exec().unwrap();
    });
}
