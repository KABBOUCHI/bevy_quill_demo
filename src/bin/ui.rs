#![feature(impl_trait_in_assoc_type)]
use bevy::prelude::{
    AlignItems, App, Camera2dBundle, Commands, DefaultPlugins, Display, FlexDirection, NodeBundle,
    ResMut, Srgba, Startup,
};
use bevy::render::camera::ClearColor;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_mod_stylebuilder::*;
use bevy_quill::{Cx, Element, QuillPlugin, View, ViewTemplate};

use bevy_quill_demo::{clx, ui::*};
use bevy_quill_obsidian::controls::Button as ObsidianButton;
use bevy_quill_obsidian::ObsidianUiPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Srgba::hex("#171717").unwrap().into()))
        .add_plugins((
            DefaultPlugins,
            DefaultPickingPlugins,
            QuillPlugin,
            QuillUiPlugin,
            // hover + cursor
            ObsidianUiPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(
        Element::<NodeBundle>::new()
            .style(|ss: &mut StyleBuilder| {
                ss.flex_direction(FlexDirection::Column);
            })
            .children((ThemeSwitcher, ShowcaseView, ClxView))
            .to_root(),
    );
}

#[derive(Clone, PartialEq)]
struct ThemeSwitcher;

impl ViewTemplate for ThemeSwitcher {
    type View = impl View;

    fn create(&self, cx: &mut Cx) -> Self::View {
        Element::<NodeBundle>::new()
            .style(|ss: &mut StyleBuilder| {
                ss.display(Display::Flex)
                    .gap(10)
                    .margin(16)
                    .padding(16)
                    .align_items(AlignItems::Center);
            })
            .children((
                ObsidianButton::new()
                    .children("Light Theme")
                    .on_click(cx.create_callback(
                        |mut theme: ResMut<QuillUiTheme>, mut color: ResMut<ClearColor>| {
                            theme.mode = ThemeMode::LIGHT;
                            color.0 = theme.colors.gray[2].into();
                        },
                    )),
                ObsidianButton::new()
                    .children("Dark Theme")
                    .on_click(cx.create_callback(
                        |mut theme: ResMut<QuillUiTheme>, mut color: ResMut<ClearColor>| {
                            theme.mode = ThemeMode::DARK;
                            color.0 = theme.colors.gray[9].into();
                        },
                    )),
            ))
    }
}

#[derive(Clone, PartialEq)]
struct ShowcaseView;

impl ViewTemplate for ShowcaseView {
    type View = impl View;

    fn create(&self, cx: &mut Cx) -> Self::View {
        let theme = cx.use_resource::<QuillUiTheme>().clone();

        Element::<NodeBundle>::new()
            .style(|ss: &mut StyleBuilder| {
                ss.display(Display::Flex)
                    .margin(16)
                    .flex_direction(FlexDirection::Column)
                    .border(3)
                    .gap(4)
                    .padding(3);
            })
            .children((
                Element::<NodeBundle>::new()
                    .style(|ss: &mut StyleBuilder| {
                        ss.display(Display::Flex)
                            .flex_direction(FlexDirection::Row)
                            .border(3)
                            .gap(10)
                            .padding(3);
                    })
                    .children((
                        Button::new().children("Primary"),
                        Button::new().children("Indigo").color(ButtonColor::Indigo),
                        Button::new().children("Blue").color(ButtonColor::Blue),
                        Button::new().children("Green").color(ButtonColor::Green),
                        Button::new().children("White").color(ButtonColor::White),
                        Button::new().children("Gray").color(ButtonColor::Gray),
                        Button::new().children("Black").color(ButtonColor::Black),
                    )),
                Element::<NodeBundle>::new()
                    .style(|ss: &mut StyleBuilder| {
                        ss.display(Display::Flex)
                            .flex_direction(FlexDirection::Row)
                            .align_items(AlignItems::Start)
                            .border(3)
                            .gap(10)
                            .padding(3);
                    })
                    .children((
                        Button::new().children("Size XS").size(ButtonSize::XS),
                        Button::new().children("Size SM").size(ButtonSize::SM),
                        Button::new().children("Size MD").size(ButtonSize::MD),
                        Button::new().children("Size LG").size(ButtonSize::LG),
                        Button::new().children("Size XL").size(ButtonSize::XL),
                    )),
                Element::<NodeBundle>::new().style(|ss: &mut StyleBuilder| {
                    ss.padding_top(10).padding_bottom(10);
                }),
                Element::<NodeBundle>::new()
                    .style(|ss: &mut StyleBuilder| {
                        ss.display(Display::Flex)
                            .flex_direction(FlexDirection::Row)
                            .align_items(AlignItems::Center)
                            .border(3)
                            .gap(10)
                            .padding(3);
                    })
                    .children((
                        "Primary: ",
                        Button::new()
                            .style(move |ss: &mut StyleBuilder| {
                                ss.background_color(theme.colors.indigo[5])
                                    .width(32)
                                    .height(32);
                            })
                            .on_click(cx.create_callback(|mut theme: ResMut<QuillUiTheme>| {
                                theme.colors.primary = theme.colors.indigo;
                            })),
                        Button::new()
                            .style(move |ss: &mut StyleBuilder| {
                                ss.background_color(theme.colors.blue[5])
                                    .width(32)
                                    .height(32);
                            })
                            .on_click(cx.create_callback(|mut theme: ResMut<QuillUiTheme>| {
                                theme.colors.primary = theme.colors.blue;
                            })),
                        Button::new()
                            .style(move |ss: &mut StyleBuilder| {
                                ss.background_color(theme.colors.green[5])
                                    .width(32)
                                    .height(32);
                            })
                            .on_click(cx.create_callback(|mut theme: ResMut<QuillUiTheme>| {
                                theme.colors.primary = theme.colors.green;
                            })),
                    )),
                Element::<NodeBundle>::new()
                    .style(|ss: &mut StyleBuilder| {
                        ss.display(Display::Flex)
                            .flex_direction(FlexDirection::Row)
                            .align_items(AlignItems::Center)
                            .border(3)
                            .gap(10)
                            .padding(3);
                    })
                    .children((
                        "Gray:    ",
                        Button::new()
                            .style(move |ss: &mut StyleBuilder| {
                                ss.background_color(theme.colors.slate[5])
                                    .width(32)
                                    .height(32);
                            })
                            .on_click(cx.create_callback(
                                |mut theme: ResMut<QuillUiTheme>, mut color: ResMut<ClearColor>| {
                                    theme.colors.gray = theme.colors.slate;
                                    color.0 = theme.colors.gray
                                        [if theme.mode == ThemeMode::DARK { 9 } else { 2 }]
                                    .into();
                                },
                            )),
                        Button::new()
                            .style(move |ss: &mut StyleBuilder| {
                                ss.background_color(theme.colors.cool[5])
                                    .width(32)
                                    .height(32);
                            })
                            .on_click(cx.create_callback(
                                |mut theme: ResMut<QuillUiTheme>, mut color: ResMut<ClearColor>| {
                                    theme.colors.gray = theme.colors.cool;
                                    color.0 = theme.colors.gray
                                        [if theme.mode == ThemeMode::DARK { 9 } else { 2 }]
                                    .into();
                                },
                            )),
                        Button::new()
                            .style(move |ss: &mut StyleBuilder| {
                                ss.background_color(theme.colors.zinc[5])
                                    .width(32)
                                    .height(32);
                            })
                            .on_click(cx.create_callback(
                                |mut theme: ResMut<QuillUiTheme>, mut color: ResMut<ClearColor>| {
                                    theme.colors.gray = theme.colors.zinc;
                                    color.0 = theme.colors.gray
                                        [if theme.mode == ThemeMode::DARK { 9 } else { 2 }]
                                    .into();
                                },
                            )),
                        Button::new()
                            .style(move |ss: &mut StyleBuilder| {
                                ss.background_color(theme.colors.neutral[5])
                                    .width(32)
                                    .height(32);
                            })
                            .on_click(cx.create_callback(
                                |mut theme: ResMut<QuillUiTheme>, mut color: ResMut<ClearColor>| {
                                    theme.colors.gray = theme.colors.neutral;
                                    color.0 = theme.colors.gray
                                        [if theme.mode == ThemeMode::DARK { 9 } else { 2 }]
                                    .into();
                                },
                            )),
                        Button::new()
                            .style(move |ss: &mut StyleBuilder| {
                                ss.background_color(theme.colors.stone[5])
                                    .width(32)
                                    .height(32);
                            })
                            .on_click(cx.create_callback(
                                |mut theme: ResMut<QuillUiTheme>, mut color: ResMut<ClearColor>| {
                                    theme.colors.gray = theme.colors.stone;
                                    color.0 = theme.colors.gray
                                        [if theme.mode == ThemeMode::DARK { 9 } else { 2 }]
                                    .into();
                                },
                            )),
                    )),
            ))
    }
}

#[derive(Clone, PartialEq)]
struct ClxView;

impl ViewTemplate for ClxView {
    type View = impl View;

    fn create(&self, cx: &mut Cx) -> Self::View {
        let id = cx.create_entity();

        Element::<NodeBundle>::for_entity(id)
            .style((
                clx("flex justify-center text-[18px] items-center gap-[3px]"),
                clx("bg-[#f0f0f0] px-[32px] py-[8px]"),
                clx("text-black"),
                clx("rounded-[16px]"),
                clx("w-[100px] h-[40px]"),
            ))
            .children("Clx")
    }
}
