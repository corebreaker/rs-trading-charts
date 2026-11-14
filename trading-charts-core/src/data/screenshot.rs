use serde::{Serialize, Serializer};

/// Image format of a screenshot.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ImageFormat {
    #[default]
    Png,

    /// JPEG, with a quality from 0 to 1.
    Jpeg(f64),

    /// WebP, with a quality from 0 to 1.
    Webp(f64),
}

impl ImageFormat {
    pub fn mime_type(&self) -> &'static str {
        match self {
            Self::Png => "image/png",
            Self::Jpeg(_) => "image/jpeg",
            Self::Webp(_) => "image/webp",
        }
    }

    pub fn quality(&self) -> Option<f64> {
        match self {
            Self::Png => None,
            Self::Jpeg(quality) | Self::Webp(quality) => Some(*quality),
        }
    }
}

/// Options of a screenshot of a chart.
///
/// The screenshots contain what Lightweight Charts draws on its canvases: the HTML elements drawn over the chart,
/// such as the built-in legend and the overlays of the adapters, are not included.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ScreenshotOptions {
    add_top_layer:     bool,
    include_crosshair: bool,
    format:            ImageFormat,
}

impl ScreenshotOptions {
    pub fn new() -> Self {
        Self::default()
    }

    /// Include the top layer (where the markers and some primitives are drawn). Default: `false`.
    pub fn with_top_layer(self, add_top_layer: bool) -> Self {
        Self {
            add_top_layer,
            ..self
        }
    }

    /// Include the crosshair, which requires the top layer. Default: `false`.
    pub fn with_crosshair(self, include_crosshair: bool) -> Self {
        Self {
            include_crosshair,
            add_top_layer: self.add_top_layer || include_crosshair,
            ..self
        }
    }

    /// Image format of the data URL and of the downloaded file. Default: PNG.
    pub fn with_format(self, format: ImageFormat) -> Self {
        Self {
            format,
            ..self
        }
    }

    pub fn add_top_layer(&self) -> bool {
        self.add_top_layer
    }

    pub fn include_crosshair(&self) -> bool {
        self.include_crosshair
    }

    pub fn format(&self) -> ImageFormat {
        self.format
    }

    pub fn set_top_layer(&mut self, add_top_layer: bool) {
        self.add_top_layer = add_top_layer;
    }

    /// Include the crosshair or not.
    /// Including it also includes the top layer, like [`with_crosshair`](Self::with_crosshair).
    pub fn set_crosshair(&mut self, include_crosshair: bool) {
        self.include_crosshair = include_crosshair;
        self.add_top_layer |= include_crosshair;
    }

    pub fn set_format(&mut self, format: ImageFormat) {
        self.format = format;
    }
}

impl Serialize for ScreenshotOptions {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Js {
            add_top_layer:     bool,
            include_crosshair: bool,
            mime_type:         &'static str,
            #[serde(skip_serializing_if = "Option::is_none")]
            quality:           Option<f64>,
        }

        Js {
            add_top_layer:     self.add_top_layer,
            include_crosshair: self.include_crosshair,
            mime_type:         self.format.mime_type(),
            quality:           self.format.quality(),
        }
        .serialize(serializer)
    }
}
