/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use ammonia::Builder;

#[magic_static::magic_static]
pub static BUILDER: Builder = {
  let mut b: Builder = Builder::default();
  b.add_allowed_classes("span", &["large", "big", "small", "square", "blue", "red", "green"]);
  b.add_generic_attributes(&["color"]);

  b
};

#[lua_function]
pub unsafe fn sanitize(lua: gmod::lua::State) -> i32 {
  let html = lua.check_string(1).to_string();

  let sanitized = BUILDER.clean(&html).to_string();

  lua.push_string(&sanitized);

  1
}