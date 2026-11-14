//! Serialization tests: the JSON shapes are the contract with the JS binding and Lightweight Charts.

use super::{
    format::{DateFormat, FontFamily, PriceFormatter, TickMarkFormatter, TickMarkType, TimeFormatter},
    options::{
        crosshair::{CrosshairLineOptions, CrosshairMode, CrosshairOptions},
        handle_scale::HandleScaleOptions,
        ChartOptions,
        FlagableOptions,
        LocalizationOptions,
        PriceFormatOptions,
    },
    Candlestick,
    Marker,
    MarkerPosition,
    MarkerShape,
    MarkerType,
    RangePreset,
    UTCTimestamp,
    ValueData,
};
use crate::series::baselines::{BaselineBaseValue, BaselineSeriesOptions};
use serde_json::{json, to_value};

fn time(s: &str) -> UTCTimestamp {
    s.parse().unwrap()
}

#[test]
fn options_only_serialize_what_is_set() {
    let options = ChartOptions::new().with_auto_size(true).with_crosshair(
        CrosshairOptions::new()
            .with_mode(CrosshairMode::Normal)
            .with_vert_line(CrosshairLineOptions::new().with_label_visible(false)),
    );

    assert_eq!(
        to_value(&options).unwrap(),
        json!({"autoSize": true, "crosshair": {"mode": 0, "vertLine": {"labelVisible": false}}})
    );
}

#[test]
fn flagable_options() {
    let flag = ChartOptions::new().with_handle_scale(false);
    let detailed = ChartOptions::new().with_handle_scale(HandleScaleOptions::new().with_mouse_wheel(false));

    assert_eq!(to_value(&flag).unwrap(), json!({"handleScale": false}));
    assert_eq!(
        to_value(&detailed).unwrap(),
        json!({"handleScale": {"mouseWheel": false}})
    );
    assert_eq!(
        serde_json::from_value::<FlagableOptions<HandleScaleOptions>>(json!(true)).unwrap(),
        FlagableOptions::Flag(true)
    );
}

#[test]
fn whitespace_data() {
    assert_eq!(
        to_value(ValueData::whitespace(time("2024-01-02"))).unwrap(),
        json!({"time": 1704153600.0})
    );
    assert_eq!(
        to_value(ValueData::new(time("2024-01-02"), 1.5).with_color("red")).unwrap(),
        json!({"time": 1704153600.0, "value": 1.5, "color": "red"})
    );
    assert!(Candlestick::whitespace(time("2024-01-02")).is_whitespace());
}

#[test]
fn markers() {
    assert_eq!(
        to_value(MarkerType::Custom(String::from("flag"))).unwrap(),
        json!("flag")
    );
    assert_eq!(
        serde_json::from_value::<MarkerType>(json!("sell")).unwrap(),
        MarkerType::Sell
    );

    let marker = Marker::styled(
        time("2024-01-02"),
        MarkerShape::Circle,
        MarkerPosition::AtPriceTop,
        "blue",
    )
    .with_price(10.0)
    .with_id("m1");

    assert_eq!(
        to_value(&marker).unwrap(),
        json!({
            "time": 1704153600.0,
            "type": "",
            "id": "m1",
            "position": "atPriceTop",
            "shape": "circle",
            "color": "blue",
            "price": 10.0,
        })
    );
}

#[test]
fn formatters() {
    let localization = LocalizationOptions::new()
        .with_date_format(DateFormat::Iso)
        .with_price_formatter(PriceFormatter::currency("EUR"))
        .with_time_formatter(TimeFormatter::pattern("HH:mm"));

    assert_eq!(
        to_value(&localization).unwrap(),
        json!({
            "priceFormatter": {"$priceFormatter": "currency", "currency": "EUR"},
            "timeFormatter": {"$timeFormatter": "pattern", "pattern": "HH:mm"},
            "dateFormat": "yyyy-MM-dd",
        })
    );

    let tick_marks = TickMarkFormatter::patterns().with_pattern(TickMarkType::Year, "'yy");
    assert_eq!(to_value(&tick_marks).unwrap()["year"], json!("'yy"));
    assert_eq!(to_value(&tick_marks).unwrap()["$tickMarkFormatter"], json!("patterns"));

    assert_eq!(
        to_value(PriceFormatOptions::custom(PriceFormatter::fixed(3))).unwrap(),
        json!({"type": "custom", "formatter": {"$priceFormatter": "fixed", "precision": 3}})
    );
}

#[test]
fn string_like_enums() {
    assert_eq!(DateFormat::from("dd.MM.yyyy"), DateFormat::Dotted);
    assert_eq!(DateFormat::from("yyyy"), DateFormat::Custom(String::from("yyyy")));
    assert_eq!(FontFamily::from(FontFamily::Monospace.css()), FontFamily::Monospace);
    assert_eq!(
        to_value(FontFamily::Custom(String::from("Inter"))).unwrap(),
        json!("Inter")
    );
}

#[test]
fn typed_enums() {
    assert_eq!(
        to_value(RangePreset::Months(3)).unwrap(),
        json!({"kind": "months", "count": 3})
    );
    assert_eq!(to_value(RangePreset::All).unwrap(), json!({"kind": "all"}));
    assert_eq!(
        to_value(BaselineSeriesOptions::new().with_base_value(BaselineBaseValue::price(10.0))).unwrap(),
        json!({"baseValue": {"type": "price", "price": 10.0}})
    );
}

#[test]
fn timestamps() {
    let t = UTCTimestamp::from_seconds(1704153600.5);

    assert_eq!(t.seconds(), 1704153600.5);
    assert_eq!(t.to_string(), "2024-01-02T00:00:00.500Z");
    assert_eq!(time("2024-01-02").seconds(), 1704153600.0);
}
