use mlua::{Function, Lua, LuaSerdeExt, Result, String, Table, UserData, UserDataMethods};
use nc::MessagePack;
use norgopolis_client as nc;
use std::io::Write;

struct Connection {
    handle: tokio::runtime::Handle,
    connection: nc::ConnectionHandle,
}

impl Connection {
    pub fn new(connection: nc::ConnectionHandle, handle: tokio::runtime::Handle) -> Connection {
        Connection { connection, handle }
    }
}

impl UserData for Connection {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut(
            "invoke",
            |lua,
             this,
             (module_name, function_name, parameters, callback): (
                String,
                String,
                Option<mlua::Value>,
                Function,
            )| {
                Ok(this
                    .handle
                    .block_on(this.connection.invoke(
                        module_name.to_str()?.into(),
                        function_name.to_str()?.into(),
                        // Some(MessagePack::encode(parameters).expect("Encoding failed")),
                        Some(MessagePack::encode("hello").unwrap()),
                        |ret: std::string::String /*: TODO*/| callback.call(ret).unwrap(), // TODO: Error handling
                    ))
                    .unwrap())
            },
        );
    }
}

#[mlua::lua_module]
pub fn norgopolis_client(lua: &Lua) -> Result<Table> {
    let client = lua.create_table()?;
    let rt = tokio::runtime::Runtime::new().unwrap();
    let _guard = rt.enter();

    client.set(
        "connect",
        lua.create_function(move |_, (ip, port): (String, Option<String>)| {
            rt.block_on(async {
                let handle = rt.handle().to_owned();
                Ok(Connection::new(
                    nc::connect(
                        &ip.to_str().unwrap().to_string(),
                        &port
                            .map(|val| val.to_str().unwrap().to_string())
                            .unwrap_or_else(|| "62020".to_string()),
                    )
                    .await
                    .unwrap(),
                    handle,
                ))
            })
        })?,
    )?;

    Ok(client)
}
