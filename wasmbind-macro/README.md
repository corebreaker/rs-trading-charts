[![Crates.io]](https://crates.io/crates/wasmbind-js-file-macro)
[![Docs.rs]](https://docs.rs/wasmbind-js-file-macro/)
# Wasmbind JS File Macro

Provide a macro to generate JavaScript bindings from an external JS file.

The procedural macro will generate the `#[wasm_bindgen]` attribute with parameter `inline_js`.
The parameter `inline_js` will be filled by this macro with the content of the file
that the path is passed as the argument `path` of this macro.
The path can contain `${outDir}` placeholders which is the value of the `OUT_DIR` environment variable.

## Pre-requisites
The crate `wasm-bindgen` must be in the dependencies of the project.

## Example
```rust
use wasmbind_js_macro::wasmbind_js;

#[wasmbind_dump_js_file_as_inline(path = "${outDir}/js/external.js")]
extern "C" {
    fn external_function();
}
```

This will generate the following code:
```rust
#[wasm_bindgen(inline_js = r###" ... here is the content of the file external.js ... "###)]
extern "C" {
    fn external_function();
}
```

[Crates.io]: https://img.shields.io/crates/v/wasmbind-js-file-macro?style=for-the-badge
[Docs.rs]: https://img.shields.io/docsrs/wasmbind-js-file-macro?style=for-the-badge
