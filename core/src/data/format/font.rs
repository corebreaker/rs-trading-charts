use crate::{bindings::js, JsError};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Font family of the texts of a chart, of its legend or of its watermarks.
///
/// The predefined values are CSS font stacks ending with a generic family, so that a font is always available. The
/// fonts which are not installed on every system (`Inter`, `Roboto`, ...) must be provided by the page, for instance
/// with [`load_web_font`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum FontFamily {
    /// The default font stack of Lightweight Charts.
    #[default]
    System,

    /// The sans-serif font of the user interface of the system.
    SansSerif,

    /// A serif font.
    Serif,

    /// A monospace font, which aligns the digits of the prices.
    Monospace,

    Arial,
    Helvetica,
    Verdana,
    Tahoma,
    TrebuchetMs,
    Georgia,
    TimesNewRoman,
    CourierNew,

    /// [Inter](https://rsms.me/inter/), which must be provided by the page.
    Inter,

    /// [Roboto](https://fonts.google.com/specimen/Roboto), which must be provided by the page on most systems.
    Roboto,

    /// [JetBrains Mono](https://www.jetbrains.com/lp/mono/), which must be provided by the page.
    JetBrainsMono,

    /// Any CSS font family list.
    Custom(String),
}

impl FontFamily {
    /// The CSS `font-family` value.
    pub fn css(&self) -> &str {
        match self {
            Self::System => "-apple-system, BlinkMacSystemFont, 'Trebuchet MS', Roboto, Ubuntu, sans-serif",
            Self::SansSerif => "system-ui, -apple-system, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif",
            Self::Serif => "Georgia, Cambria, 'Times New Roman', Times, serif",
            Self::Monospace => "ui-monospace, SFMono-Regular, Menlo, Consolas, 'Liberation Mono', monospace",
            Self::Arial => "Arial, Helvetica, sans-serif",
            Self::Helvetica => "'Helvetica Neue', Helvetica, Arial, sans-serif",
            Self::Verdana => "Verdana, Geneva, sans-serif",
            Self::Tahoma => "Tahoma, Geneva, sans-serif",
            Self::TrebuchetMs => "'Trebuchet MS', Helvetica, sans-serif",
            Self::Georgia => "Georgia, serif",
            Self::TimesNewRoman => "'Times New Roman', Times, serif",
            Self::CourierNew => "'Courier New', Courier, monospace",
            Self::Inter => "Inter, system-ui, sans-serif",
            Self::Roboto => "Roboto, system-ui, sans-serif",
            Self::JetBrainsMono => "'JetBrains Mono', ui-monospace, monospace",
            Self::Custom(css) => css,
        }
    }

    fn predefined() -> [Self; 15] {
        [
            Self::System,
            Self::SansSerif,
            Self::Serif,
            Self::Monospace,
            Self::Arial,
            Self::Helvetica,
            Self::Verdana,
            Self::Tahoma,
            Self::TrebuchetMs,
            Self::Georgia,
            Self::TimesNewRoman,
            Self::CourierNew,
            Self::Inter,
            Self::Roboto,
            Self::JetBrainsMono,
        ]
    }
}

impl From<&str> for FontFamily {
    fn from(css: &str) -> Self {
        Self::predefined()
            .into_iter()
            .find(|predefined| predefined.css() == css)
            .unwrap_or_else(|| Self::Custom(css.to_string()))
    }
}

impl From<String> for FontFamily {
    fn from(css: String) -> Self {
        Self::from(css.as_str())
    }
}

impl Display for FontFamily {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.css())
    }
}

impl Serialize for FontFamily {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.css())
    }
}

impl<'de> Deserialize<'de> for FontFamily {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(deserializer)?))
    }
}

/// Load a web font in the page, under the name `family`, then redraw the charts.
///
/// `source` is the URL of the font file (WOFF2, WOFF, TTF...), or a CSS `src` value such as `url(...) format(...)`.
/// Charts drawn before the font is loaded are redrawn when it becomes available. The returned promise resolves to
/// `true` when the font is loaded, or to `false` when it cannot be loaded (the error is reported on the console).
///
/// Use [`FontFamily::Custom`] with `family` to use the font.
pub fn load_web_font(family: &str, source: &str) -> Result<js_sys::Promise, JsError> {
    Ok(js::loadWebFont(family, source, wasm_bindgen::JsValue::NULL)?)
}
