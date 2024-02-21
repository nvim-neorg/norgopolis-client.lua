# A Lua API for Norgopolis Clients

For information about Norgopolis, consult https://github.com/nvim-neorg/norgopolis.

This repository acts as a wrapper around https://github.com/nvim-neorg/norgopolis-client for Lua.
Any Lua application may install this repository as a dependency and communicate with Norgopolis
and its modules.

# Installation

```sh
luarocks --local install norgopolis-client.lua
```

# Usage

`norgopolis-client.lua` exposes a very simple yet extendable API:

```lua
-- Connect to Norgopolis. The port number may be omitted and will default to 62020.
local norgopolis = require("norgopolis").connect("localhost", "62020")

-- Invoke a function of a given module. The third argument is the parameters to send
-- over to the module. It may be nil or any inbuilt Lua datatype.
-- This will most commonly be a table.
norgopolis:invoke("module-name", "function-name", "hello!", function(ret)
    -- This function gets invoked every time a module responds back with a response
    -- packet. `ret` could be anything, so be careful!
    print(ret)
end)
```
