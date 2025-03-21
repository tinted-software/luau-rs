# tinted_luau_sys

Rust bindings for the [Luau Engine](https://github.com/luau-lang/luau).
This crate currently depends on the CMake Improvements PR that adds pkg-config support.

You can compile the Luau Engine with the following commands on a Linux machine:

```shell
git clone https://github.com/theoparis/luau
cd luau
cmake -B build -G Ninja -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/usr/local
cmake --build build
sudo cmake --install build
```
