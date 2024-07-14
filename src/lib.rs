#![feature(impl_trait_in_assoc_type)]
use bevy::color::Srgba;
use bevy_mod_stylebuilder::*;

pub mod ui;

pub fn clx(styles: &'static str) -> impl StyleTuple + 'static {
    use bevy::prelude::{Srgba, Val};

    fn parse_val(prefix: &str, style: &str) -> bevy::ui::Val {
        let style = &style.replace(prefix, "").replace('[', "").replace(']', "");

        if style.ends_with("auto") {
            Val::Auto
        } else if style.contains('/') {
            todo!()
        } else if style.contains('%') {
            Val::Percent(style.replace('%', "").parse::<f32>().unwrap_or_default())
        } else if style.contains("px") {
            Val::Px(style.replace("px", "").parse::<f32>().unwrap_or_default())
        } else {
            todo!("Unknown style: {}", style)
        }
    }

    fn parse_color(prefix: &str, style: &str) -> Srgba {
        let style = &style.replace(prefix, "").replace('[', "").replace(']', "");

        if style == "black" {
            Srgba::BLACK
        } else if style == "white" {
            Srgba::WHITE
        } else if style.contains('#') {
            Srgba::hex(style).unwrap_or_default()
        } else {
            todo!("Unknown color: {}", style)
        }
    }

    return |ss: &mut StyleBuilder| {
        for style in styles.split_whitespace() {
            match style {
                "flex" => {
                    ss.display(bevy::ui::Display::Flex);
                }
                "hidden" => {
                    ss.display(bevy::ui::Display::None);
                }
                "block" => {
                    ss.display(bevy::ui::Display::Block);
                }
                "grid" => {
                    ss.display(bevy::ui::Display::Grid);
                }
                "flex-row" => {
                    ss.flex_direction(bevy::ui::FlexDirection::Row);
                }
                "flex-col" => {
                    ss.flex_direction(bevy::ui::FlexDirection::Column);
                }
                "flex-wrap" => {
                    ss.flex_wrap(bevy::ui::FlexWrap::Wrap);
                }
                "flex-nowrap" => {
                    ss.flex_wrap(bevy::ui::FlexWrap::NoWrap);
                }
                "justify-center" => {
                    ss.justify_content(bevy::ui::JustifyContent::Center);
                }
                "items-center" => {
                    ss.align_items(bevy::ui::AlignItems::Center);
                }
                "content-center" => {
                    ss.align_content(bevy::ui::AlignContent::Center);
                }
                style if style.starts_with("px-") => {
                    let val = parse_val("px-", style);
                    ss.padding_left(val);
                    ss.padding_right(val);
                }
                style if style.starts_with("py-") => {
                    let val = parse_val("py-", style);
                    ss.padding_top(val);
                    ss.padding_bottom(val);
                }
                style if style.starts_with("p-") => {
                    let val = parse_val("p-", style);
                    ss.padding(val);
                }
                style if style.starts_with("mx-") => {
                    let val = parse_val("mx-", style);
                    ss.margin_left(val);
                    ss.margin_right(val);
                }
                style if style.starts_with("my-") => {
                    let val = parse_val("my-", style);
                    ss.margin_top(val);
                    ss.margin_bottom(val);
                }
                style if style.starts_with("m-") => {
                    let val = parse_val("m-", style);
                    ss.margin(val);
                }
                style if style.starts_with("bg-") => {
                    let color = parse_color("bg-", style);
                    ss.background_color(color);
                }

                style if style.starts_with("text-") => {
                    if style.contains("px") {
                        let val = parse_val("text-", style);

                        if let Val::Px(s) = val {
                            ss.font_size(s);
                        }
                    } else {
                        let color = parse_color("text-", style);
                        ss.color(color);
                    }
                }
                style if style.starts_with("gap-") => {
                    let val = parse_val("gap-", style);
                    ss.gap(val);
                }
                style if style.starts_with("rounded-") => {
                    let val = parse_val("rounded-", style);
                    ss.border_radius(val);
                }
                style if style.starts_with("w-") => {
                    let val = parse_val("w-", style);
                    ss.width(val);
                }
                style if style.starts_with("h-") => {
                    let val = parse_val("h-", style);
                    ss.width(val);
                }
                "border-0" => {
                    ss.border(0);
                }
                _ => {
                    println!("Unknown style: {}", style);
                }
            }
        }
    };
}
