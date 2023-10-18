use mlua::{Function, Lua, Result, String, Table, UserData, UserDataMethods};
use nc::MessagePack;
use norgopolis_client as nc;

mod msgpack_lua_transcoder;

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
                this.handle
                    .block_on(this.connection.invoke_raw_callback(
                        module_name.to_str()?.into(),
                        function_name.to_str()?.into(),
                        Some(msgpack_lua_transcoder::transcode_lua_msgpack(
                            parameters.unwrap(),
                        )),
                        |ret: MessagePack| {
                            let lua_value =
                                msgpack_lua_transcoder::transcode_msgpack_lua(lua, ret).unwrap();
                            callback.call::<_, ()>(lua_value).unwrap();
                        }, // TODO: Error handling
                    ))
                    .unwrap();
                Ok(())
            },
        );
    }
}

#[mlua::lua_module]
pub fn norgopolis(lua: &Lua) -> Result<Table> {
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
