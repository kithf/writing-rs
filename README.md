# writing-rs
Rust module for writing plugin

## Installation
1. Download binaries from [releases](https://github.com/kithf/writing-rs/releases) or compile by yourself.
2. Put binaries into `{your SRCDS folder}/garrysmod/lua/bin`(If there is no bin folder, create one)
3. Now you can install [writing](https://github.com/kithf/writing/) plugin

## Compile by yourself
1. Install rust from https://www.rust-lang.org
2. Clone this repository `git clone https://github.com/kithf/writing-rs`
3. Navigate into repository `cd writing-rs`
4. Compile binary for your SRCDS with cargo `cargo build -r --target *your-target*`(See [targets](https://doc.rust-lang.org/nightly/rustc/platform-support.html#tier-1-with-host-tools), first you may need to install target)
5. Go to `target/*your-target*/release/`
6. Rename your to `gmsv_writingrs_*your-target*.dll`(See [wiki](https://wiki.facepunch.com/gmod/Creating_Binary_Modules#naminglocation))
7. Put binaries into `{your SRCDS folder}/garrysmod/lua/bin`(If there is no bin folder, create one)
8. Now you can install [writing](https://github.com/kithf/writing/) plugin

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

