/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use pulldown_cmark::{Parser, Options, html};

#[lua_function]
pub unsafe fn to_html(lua: gmod::lua::State) -> i32 {
  let markdown = lua.check_string(1).to_string();

  let parser = Parser::new_ext(&markdown, Options::all());
  let mut html_buf = String::new();
  html::push_html(&mut html_buf, parser);

  lua.push_string(&html_buf);

  1
}