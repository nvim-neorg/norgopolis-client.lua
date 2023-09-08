package = "norgopolis-client"
version = "dev-1"
source = {
   url = "git+https://github.com/nvim-neorg/norgopolis-client.lua"
}
description = {
   homepage = "https://github.com/vhyrro/norgopolis-client.lua",
   license = "MIT"
}

dependencies = {
    "lua >= 5.1",
    "luarocks-build-rust-mlua",
}

build = {
    type = "rust-mlua",
    modules = {
        "norgopolis_client"
    },
}
