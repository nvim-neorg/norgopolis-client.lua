use mlua::{Lua, LuaSerdeExt};
use norgopolis_client::MessagePack;

///! TODO: This whole file requires implementing error handling.

/// Transcodes from messagepack to a lua value
///
/// * `value`:
pub fn transcode_msgpack_lua(
    lua: &Lua,
    data: MessagePack,
) -> Result<mlua::Value<'_>, rmpv::decode::Error> {
    let mut string = data.data.as_slice();

    let value = rmpv::decode::read_value(&mut string)?;

    Ok(transcode_msgpack_lua_value(lua, value))
}

fn transcode_msgpack_lua_value(lua: &Lua, value: rmpv::Value) -> mlua::Value<'_> {
    match value {
        rmpv::Value::Nil => mlua::Value::Nil,
        rmpv::Value::Boolean(bool) => mlua::Value::Boolean(bool),
        rmpv::Value::Integer(int) => mlua::Value::Integer(int.as_i64().unwrap()),
        rmpv::Value::F32(f32) => mlua::Value::Number(f32.into()),
        rmpv::Value::F64(f64) => mlua::Value::Number(f64),
        rmpv::Value::String(str) => lua.to_value(&str.into_str().unwrap()).unwrap(),
        rmpv::Value::Binary(raw) => lua.to_value(&raw.as_slice()).unwrap(),
        rmpv::Value::Array(array) => {
            let ret = lua.create_table().unwrap();

            for value in array.into_iter() {
                ret.push(transcode_msgpack_lua_value(lua, value)).unwrap();
            }

            mlua::Value::Table(ret)
        }
        rmpv::Value::Map(map) => {
            let ret = lua.create_table().unwrap();

            for (key, value) in map.into_iter() {
                ret.set(
                    transcode_msgpack_lua_value(lua, key),
                    transcode_msgpack_lua_value(lua, value),
                )
                .unwrap();
            }

            mlua::Value::Table(ret)
        }
        rmpv::Value::Ext(_, _) => todo!(),
    }
}

pub fn transcode_lua_msgpack(value: mlua::Value) -> MessagePack {
    let mut bytes = Vec::new();

    let transcoded = transcode_lua_msgpack_value(value);
    rmpv::encode::write_value(&mut bytes, &transcoded).unwrap();

    MessagePack { data: bytes }
}

pub fn transcode_lua_msgpack_value(value: mlua::Value) -> rmpv::Value {
    match value {
        mlua::Value::Nil => rmpv::Value::Nil,
        mlua::Value::Boolean(bool) => rmpv::Value::Boolean(bool),
        mlua::Value::LightUserData(_userdata) => unimplemented!(),
        mlua::Value::Integer(int) => rmpv::Value::Integer(int.into()),
        mlua::Value::Number(num) => rmpv::Value::F64(num),
        mlua::Value::String(str) => {
            rmpv::Value::String(rmpv::Utf8String::from(str.to_str().unwrap()))
        }
        mlua::Value::Table(tbl) => {
            // If true, we are dealing with a map
            if tbl.raw_len() == 0 {
                let mut ret: Vec<(rmpv::Value, rmpv::Value)> = Vec::new();

                for kv in tbl.pairs() {
                    let (key, value) = kv.unwrap();
                    ret.push((
                        transcode_lua_msgpack_value(key),
                        transcode_lua_msgpack_value(value),
                    ));
                }

                rmpv::Value::Map(ret)
            } else {
                let mut ret: Vec<rmpv::Value> = Vec::new();

                for value in tbl.sequence_values() {
                    ret.push(transcode_lua_msgpack_value(value.unwrap()));
                }

                rmpv::Value::Array(ret)
            }
        }
        mlua::Value::Function(_) => panic!("TODO. Functions cannot be encoded in messagepack!"),
        mlua::Value::Thread(_) => unimplemented!(),
        mlua::Value::UserData(_) => unimplemented!(),
        mlua::Value::Error(_) => unimplemented!(),
    }
}
