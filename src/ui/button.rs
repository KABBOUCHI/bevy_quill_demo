use crate::ui::ThemeMode;
use bevy::{
    color::{Color, Srgba},
    prelude::NodeBundle,
    ui::Val,
    window::CursorIcon,
};
use bevy_mod_stylebuilder::{
    StyleBuilder, StyleBuilderBackground, StyleBuilderBorderColor, StyleBuilderBorderRadius,
    StyleBuilderFont, StyleBuilderLayout,
};
use bevy_quill::{Element, IntoViewChild, View, ViewChild, ViewTemplate};
use bevy_quill_obsidian::{cursor::StyleBuilderCursor, hooks::UseIsHover};

use super::QuillUiTheme;

#[derive(Default, Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    Link,
    #[default]
    Solid,
    Outline,
    Soft,
    Ghost,
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum ButtonColor {
    #[default]
    Primary,
    Indigo,
    Blue,

    White,
    Black,
    Gray,
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum ButtonSize {
    XS,
    #[default]
    SM,
    MD,
    LG,
    XL,
}

#[derive(Default, Clone, PartialEq)]
pub struct Button {
    pub children: ViewChild,
    pub color: ButtonColor,
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub block: bool,
}

impl Button {
    pub fn new() -> Button {
        Self::default()
    }

    pub fn children(mut self, children: impl IntoViewChild) -> Self {
        self.children = children.into_view_child();
        self
    }

    pub fn color(mut self, color: ButtonColor) -> Self {
        self.color = color;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }
}

impl ViewTemplate for Button {
    type View = impl View;

    fn create(&self, cx: &mut bevy_quill::Cx) -> Self::View {
        let id = cx.create_entity();
        let color = self.color;
        let hovering = cx.is_hovered(id);
        let theme = cx.use_resource::<QuillUiTheme>().clone();

        Element::<NodeBundle>::for_entity(id)
            .named("Button")
            .style(|ss: &mut StyleBuilder| {
                ss.cursor(CursorIcon::Pointer);

                // block: w-full flex justify-center items-center
                ss.display(bevy::ui::Display::Flex)
                    .flex_shrink(0.)
                    .justify_items(bevy::ui::JustifyItems::Center)
                    // .width(Val::Px(100.))
                    .align_items(bevy::ui::AlignItems::Center);

                // inline: inline-flex items-center

                // rounded-md
                ss.border_radius(6.0);

                ss.border(1).border_color(Color::BLACK);
            })
            .style_dyn(
                |(hovering, color, theme), sb| {
                    sb.background_color(button_bg_color(color, theme.clone(), hovering));
                    sb.color(button_color(color, theme.clone(), hovering));

                    if color == ButtonColor::White || color == ButtonColor::Gray {
                        sb.border(1).border_color(
                            theme.colors.gray[if theme.mode == ThemeMode::DARK { 7 } else { 3 }],
                        );
                    } else {
                        sb.border(0);
                    }
                },
                (hovering, color, theme),
            )
            .style_dyn(
                |size, sb| match size {
                    ButtonSize::XS => {
                        sb.column_gap(6.0).font_size(12).padding((10, 6));
                    }
                    ButtonSize::SM => {
                        sb.column_gap(6.0).font_size(16).padding((10, 6));
                    }
                    ButtonSize::MD => {
                        sb.column_gap(8.0).font_size(16).padding((12, 8));
                    }
                    ButtonSize::LG => {
                        sb.column_gap(10.0).font_size(16).padding((14, 10));
                    }
                    ButtonSize::XL => {
                        sb.column_gap(10.0).font_size(16).padding((14, 10));
                    }
                },
                self.size,
            )
            .children(self.children.clone())
    }
}

fn button_bg_color(color: ButtonColor, theme: QuillUiTheme, is_hovering: bool) -> Srgba {
    let mode = theme.mode;
    let index = if is_hovering {
        5
    } else {
        if mode == ThemeMode::DARK {
            4
        } else {
            6
        }
    };

    if color == ButtonColor::Primary {
        return theme.colors.primary[index];
    }

    if color == ButtonColor::Indigo {
        return theme.colors.indigo[index];
    }

    if color == ButtonColor::Blue {
        return theme.colors.blue[index];
    }

    if color == ButtonColor::White {
        if mode == ThemeMode::DARK {
            return {
                if is_hovering {
                    theme.colors.gray[8]
                } else {
                    theme.colors.gray[9]
                }
            };
        } else {
            return {
                if is_hovering {
                    theme.colors.gray[1]
                } else {
                    theme.colors.white
                }
            };
        }
    }

    if color == ButtonColor::Black {
        if mode == ThemeMode::DARK {
            return {
                if is_hovering {
                    theme.colors.gray[1]
                } else {
                    theme.colors.white
                }
            };
        } else {
            return {
                if is_hovering {
                    theme.colors.gray[8]
                } else {
                    theme.colors.gray[9]
                }
            };
        }
    }

    if color == ButtonColor::Gray {
        if mode == ThemeMode::DARK {
            return theme.colors.gray[8];
        } else {
            return theme.colors.gray[0];
        }
    }

    return theme.colors.indigo[0];
}
fn button_color(color: ButtonColor, theme: QuillUiTheme, is_hovering: bool) -> Srgba {
    let mode = theme.mode;

    if color == ButtonColor::White {
        if mode == ThemeMode::DARK {
            return theme.colors.white;
        }

        if mode == ThemeMode::LIGHT {
            return theme.colors.gray[9];
        }
    }

    if color == ButtonColor::Gray {
        if mode == ThemeMode::DARK {
            return theme.colors.white;
        } else {
            return theme.colors.gray[7];
        }
    }

    if mode == ThemeMode::LIGHT {
        return theme.colors.white;
    }

    return theme.colors.gray[9];
}