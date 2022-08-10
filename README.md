# writing-rs
Rust module for writing plugin

## Content

### lz4
lz4_flex wrapper for lua.
##### lz4.compress(data: `string`): `string`
Compress data with lz4.
```lua
local bin = lz4.compress "string"
```

##### lz4.decompress(data: `string`): `string`
Decompress data with lz4.
```lua
local str = lz4.decompress(lz4.compress"string")

assert(str=="string")
```

### markdown
CommonMark parser for lua.

##### markdown.to_html(data: `string`): `string`
Parse markdown to html.
```lua
local html = markdown.to_html "## Hello"

assert(html == "<h2>Hello</h2>\n")
```

### html

#### html.sanitize(data: `string`): `string`
Sanitize html.
```lua
local html = html.sanitize "<script>alert('xss')</script>"

assert(html == "")
```

