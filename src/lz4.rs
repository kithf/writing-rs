/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use lz4_flex::{compress_prepend_size, decompress_size_prepended};

#[lua_function]
pub unsafe fn compress(lua: gmod::lua::State) -> i32 {
  let data = lua.check_binary_string(1);
  let comp = compress_prepend_size(data);

  lua.push_binary_string(&comp);

  1
}

#[lua_function]
pub unsafe fn decompress(lua: gmod::lua::State) -> i32 {
  let data = lua.check_binary_string(1);
  let decomp = match decompress_size_prepended(data) {
    Ok(d) => d,
    Err(err) => lua.error(format!("Decompression error: {}", err)),
  };

  lua.push_binary_string(&decomp);

  1
}