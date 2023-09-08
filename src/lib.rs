use mlua::{Function, Lua, LuaSerdeExt, Result, String, Table, UserData, UserDataMethods};
use nc::MessagePack;
use norgopolis_client as nc;

struct Connection(nc::ConnectionHandle);

impl UserData for Connection {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_method_mut(
            "invoke",
            |lua,
             this,
             (module_name, function_name, parameters, callback): (
                String,
                String,
                Table,
                Function,
            )| async move {
                Ok(this
                    .0
                    .invoke::<(), _>(
                        module_name.to_str()?.into(),
                        function_name.to_str()?.into(),
                        Some(MessagePack::encode(parameters).expect("Encoding failed")),
                        |_| callback.call(lua.to_value(&"Hello")).unwrap(), // TODO:
                                                                            // Allow
                                                                            // a
                                                                            // raw
                                                                            // response.
                    )
                    .await
                    .unwrap())
            },
        );
    }
}

#[mlua::lua_module]
pub fn norgopolis_client(lua: &Lua) -> Result<Table> {
    let client = lua.create_table()?;

    client.set(
        "connect",
        lua.create_async_function(|_, (ip, port): (String, Option<String>)| async move {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let _guard = rt.enter();

            // NOTE: This seems to run the future off-thread which makes the service "not ready"
            // according to the rust side.
            Ok(Connection(
                rt.block_on(nc::connect(
                    &ip.to_str().unwrap().to_string(),
                    &port
                        .map(|val| val.to_str().unwrap().to_string())
                        .unwrap_or("62020".to_string()),
                ))
                .unwrap(),
            ))
        })?,
    )?;

    Ok(client)
}
