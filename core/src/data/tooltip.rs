use super::MouseEventParams;

/// How a tooltip follows the crosshair.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum TooltipMode {
    /// Next to the cursor, on the side which has the most room.
    #[default]
    Tracking,

    /// Above the value of a series at the time of the cursor (below it when there is no room above).
    Floating,

    /// Along the top edge of the chart, horizontally centered on the cursor.
    Magnifier,
}

/// Position of a tooltip, relatively to the top-left corner of the chart.
///
/// The tooltip is placed at `(left, top)` then translated by a percentage of its own size, which keeps it inside the
/// chart without knowing its size.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TooltipPlacement {
    pub left:        f64,
    pub top:         f64,
    /// Horizontal translation, in percent of the tooltip width (from `-100` to `0`).
    pub translate_x: f64,
    /// Vertical translation, in percent of the tooltip height (from `-100` to `0`).
    pub translate_y: f64,
}

impl TooltipPlacement {
    /// Compute the placement of a tooltip for a crosshair position.
    ///
    /// `anchor_series` is the identifier of the series followed by a [`TooltipMode::Floating`] tooltip; the series
    /// under the cursor, or else the first series, is followed when it is `None`. `offset` is the distance in pixels
    /// between the tooltip and the point it follows.
    ///
    /// Returns `None` when the tooltip must be hidden: the cursor is outside of the chart or of the data.
    pub fn compute(
        mode: TooltipMode,
        params: &MouseEventParams,
        anchor_series: Option<&str>,
        offset: f64,
    ) -> Option<Self> {
        let point = params.point?;
        let (width, height) = (params.chart_width, params.chart_height);

        if params.time.is_none() || point.x < 0.0 || point.y < 0.0 || point.x > width || point.y > height {
            return None;
        }

        let ratio_x = if width > 0.0 {
            (point.x / width).clamp(0.0, 1.0)
        } else {
            0.0
        };

        Some(match mode {
            TooltipMode::Tracking => {
                let right = point.x < width / 2.0;
                let below = point.y < height / 2.0;

                Self {
                    left:        if right { point.x + offset } else { point.x - offset },
                    top:         if below { point.y + offset } else { point.y - offset },
                    translate_x: if right { 0.0 } else { -100.0 },
                    translate_y: if below { 0.0 } else { -100.0 },
                }
            }

            TooltipMode::Floating => {
                let data = match anchor_series {
                    Some(id) => params.series(id),
                    None => params.main_series(),
                }?;
                let y = data.coordinate?;
                let above = y - offset > height * 0.25;

                Self {
                    left:        point.x,
                    top:         if above { y - offset } else { y + offset },
                    translate_x: -100.0 * ratio_x,
                    translate_y: if above { -100.0 } else { 0.0 },
                }
            }

            TooltipMode::Magnifier => Self {
                left:        point.x,
                top:         offset,
                translate_x: -100.0 * ratio_x,
                translate_y: 0.0,
            },
        })
    }

    /// CSS declarations placing an absolutely positioned element.
    pub fn css(&self) -> String {
        format!(
            "left:{:.1}px;top:{:.1}px;transform:translate({:.1}%,{:.1}%)",
            self.left, self.top, self.translate_x, self.translate_y
        )
    }
}
