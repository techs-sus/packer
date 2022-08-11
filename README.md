# packer

Packer is a module bundler created for Void Script Builder.

It uses [full-moon](https://crates.io/crate/full-moon) to parse an AST from your lua code and recurse for `import` calls and `nls_import` calls.

# roadmap

- development server likely using localtunnel (or alternatives)
- recursive module detection (right now it overflows the stack)
- code minfication / obfuscation
- full protection from tusk logging & other loggers
- better configuration with toml (right now its really bad)

# q/a

Q: What happened to pack & rpack?

A: Both projects were incredibly slow, and I wanted to rebuild the whole project.

# code examples

Here is how to import a module in your code.

```lua
-- the @module is for roblox LSP
---@module file
local file = import("file.lua")
```

If you want to import a localscript please use

```lua
nls_import("local.lua")
```
