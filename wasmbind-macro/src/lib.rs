use quote::quote;
use proc_macro2::Span;
use proc_macro::TokenStream;
use syn::{
    parse::{Parse, ParseStream},
    Ident,
    Token,
    LitStr,
    Result as SynResult,
    parse_macro_input,
};

use std::{path::PathBuf, fs::read_to_string, env};

#[derive(Clone)]
struct PathOption {
    span: Span,
    path: PathBuf,
}

impl PathOption {
    fn contents(&self) -> LitStr {
        let contents = read_to_string(&self.path).unwrap_or_else(|err| {
            if cfg!(any(doc, docsrs)) {
                String::from("(() => {})();")
            } else {
                panic!("Failed to read file {:?}: {}", self.path, err);
            }
        });

        LitStr::new(&contents, self.span.clone())
    }
}

impl Parse for PathOption {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let span = input.span();
        let option_name = input.parse::<Ident>()?;
        if option_name != "path" {
            return Err(input.error("Expected `path` option"));
        }

        input.parse::<Token![=]>()?;

        let path_str = input.parse::<LitStr>()?.value();
        let mut path = PathBuf::from(&path_str);

        const PREFIX: &'static str = "${outDir}/";
        if path_str.starts_with(PREFIX) {
            let out_dir = env::var("OUT_DIR").map_err(|_| input.error("OUT_DIR environment variable not found"))?;

            path = PathBuf::from(out_dir);
            path.extend(PathBuf::from(&path_str[PREFIX.len()..]).iter());
        }

        Ok(Self {
            span,
            path,
        })
    }
}

/**
Procedural macro for generating the `#[wasm_bindgen]` attribute with parameter `inline_js`.
The parameter `inline_js` will be filled by this macro with the content of the file
that the path is passed as the argument `path` of this macro.
The path can contain `$out_dir` placeholders which is the value of the `OUT_DIR` environment variable.
*/
#[proc_macro_attribute]
pub fn wasmbind_dump_js_file_as_inline(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = proc_macro2::TokenStream::from(item);
    let path_option = parse_macro_input!(attr as PathOption);
    let contents = path_option.contents();

    let output = quote! {
        #[wasm_bindgen(inline_js = #contents)]
        #item
    };

    output.into()
}
