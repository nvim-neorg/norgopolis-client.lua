# Changelog

## [0.2.0](https://github.com/nvim-neorg/norgopolis-client.lua/compare/v0.1.4...v0.2.0) (2024-02-17)


### Features

* use newer version of norgopolis client ([f8c86ba](https://github.com/nvim-neorg/norgopolis-client.lua/commit/f8c86ba40fb24bc4e5fdef7d4fbf03bedbb1acf7))

## [0.1.4](https://github.com/nvim-neorg/norgopolis-client.lua/compare/v0.1.3...v0.1.4) (2024-02-17)


### Bug Fixes

* do not try to autostart the server automatically ([0e2bdb0](https://github.com/nvim-neorg/norgopolis-client.lua/commit/0e2bdb0e8fc6fa4c24403bf7b0f3c39128b85cb2))

## [0.1.3](https://github.com/nvim-neorg/norgopolis-client.lua/compare/v0.1.2...v0.1.3) (2024-01-03)


### Bug Fixes

* **ci/luarocks:** use `sudo` ([4a8d7e5](https://github.com/nvim-neorg/norgopolis-client.lua/commit/4a8d7e54dc455c7ca7efbd0e4c30db847fffe65e))

## [0.1.2](https://github.com/nvim-neorg/norgopolis-client.lua/compare/v0.1.1...v0.1.2) (2024-01-03)


### Bug Fixes

* **ci/luarocks:** install protoc before publishing to luarocks ([11a577d](https://github.com/nvim-neorg/norgopolis-client.lua/commit/11a577d904d828c23f5a8131b3e55ae770f961f1))

## [0.1.1](https://github.com/nvim-neorg/norgopolis-client.lua/compare/v0.1.0...v0.1.1) (2024-01-03)


### Bug Fixes

* **ci/luarocks:** typo in path to template ([2d3bebe](https://github.com/nvim-neorg/norgopolis-client.lua/commit/2d3bebeb6b964fe6b4846c36f6861428e1ce7aaa))

## 0.1.0 (2024-01-03)


### ⚠ BREAKING CHANGES

* change `require("norgopolis_client")` -> `require("norgopolis")`

### ref

* change `require("norgopolis_client")` -&gt; `require("norgopolis")` ([ba8dc3a](https://github.com/nvim-neorg/norgopolis-client.lua/commit/ba8dc3ac4399d949b429900fbdcd96c823fc3466))


### Features

* add conversion between lua types and msgpack types (excluding userdata) ([3cf5bd5](https://github.com/nvim-neorg/norgopolis-client.lua/commit/3cf5bd5e3d283834392fbfb3ed0a210212340a28))
* add luarocks workflow ([cab8c5a](https://github.com/nvim-neorg/norgopolis-client.lua/commit/cab8c5ac1c1fbbfbdd73bca85c8c7a87f0e348fe))
* first working version ([19362fd](https://github.com/nvim-neorg/norgopolis-client.lua/commit/19362fdc013a9aa21154ea1fc76157a1d07a59c0))
* initial commit ([712bd51](https://github.com/nvim-neorg/norgopolis-client.lua/commit/712bd51cd10dba672f602c29b3eb5b0d873a8e16))


### Bug Fixes

* make new name change work with `luarocks build` ([8c0e3b6](https://github.com/nvim-neorg/norgopolis-client.lua/commit/8c0e3b68de0d76671ce0d9b59b57a94a26b3272c))
