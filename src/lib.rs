/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
#![feature(c_unwind)]

#[macro_use] extern crate gmod;

use gmod::lua::State;

mod lz4;
mod markdown;
mod sanitize;

#[gmod13_open]
unsafe fn gmod13_open(lua: State) -> i32 {
  println!("Loading Writing.rs");
  
  magic_static::init! {sanitize::BUILDER};

  // lz4
  lua.new_table();

  lua.push_function(lz4::compress);
  lua.set_field(-2, lua_string!("compress"));

  lua.push_function(lz4::decompress);
  lua.set_field(-2, lua_string!("decompress"));

  lua.set_global(lua_string!("lz4"));

  // markdown
  lua.new_table();

  lua.push_function(markdown::to_html);
  lua.set_field(-2, lua_string!("to_html"));

  lua.set_global(lua_string!("markdown"));

  // sanitizer
  lua.new_table();

  lua.push_function(sanitize::sanitize);
  lua.set_field(-2, lua_string!("sanitize"));

  lua.set_global(lua_string!("html"));

  println!("Loaded Writing.rs");

  0
}

#[gmod13_close]
unsafe fn gmod13_close(_lua: State) -> i32 {
  0
}