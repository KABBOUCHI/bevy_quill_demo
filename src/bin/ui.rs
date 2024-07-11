#![feature(impl_trait_in_assoc_type)]
use bevy::color::Color;
use bevy::prelude::{
    AlignItems, App, Camera2dBundle, Commands, DefaultPlugins, Display, FlexDirection, NodeBundle,
    ResMut, Startup,
};
use bevy::render::camera::ClearColor;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_mod_stylebuilder::*;
use bevy_quill::{Cx, Element, QuillPlugin, View, ViewTemplate};

use bevy_quill_demo::ui::*;
use bevy_quill_obsidian::controls::Button as ObsidianButton;
use bevy_quill_obsidian::ObsidianUiPlugin;

fn main() {
    App::new()
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
            .children((ThemeSwitcher, ShowcaseView))
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
                            color.0 = Color::WHITE;
                        },
                    )),
                ObsidianButton::new()
                    .children("Dark Theme")
                    .on_click(cx.create_callback(
                        |mut theme: ResMut<QuillUiTheme>, mut color: ResMut<ClearColor>| {
                            theme.mode = ThemeMode::DARK;
                            color.0 = Color::srgb_u8(43, 44, 47);
                        },
                    )),
            ))
    }
}

#[derive(Clone, PartialEq)]
struct ShowcaseView;

impl ViewTemplate for ShowcaseView {
    type View = impl View;

    fn create(&self, _cx: &mut Cx) -> Self::View {
        Element::<NodeBundle>::new()
            .style(|ss: &mut StyleBuilder| {
                ss.display(Display::Flex)
                    .margin(16)
                    .flex_direction(FlexDirection::Column)
                    .align_items(AlignItems::Center)
                    .border(3)
                    .gap(4)
                    .padding(3);
            })
            .children((
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
                        Button::new().children("Primary"),
                        Button::new().children("Indigo").color(ButtonColor::Indigo),
                        Button::new().children("Blue").color(ButtonColor::Blue),
                        Button::new().children("White").color(ButtonColor::White),
                        Button::new().children("Gray").color(ButtonColor::Gray),
                        Button::new().children("Black").color(ButtonColor::Black),
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
                        Button::new().children("Size XS").size(ButtonSize::XS),
                        Button::new().children("Size SM").size(ButtonSize::SM),
                        Button::new().children("Size MD").size(ButtonSize::MD),
                        Button::new().children("Size LG").size(ButtonSize::LG),
                        Button::new().children("Size XL").size(ButtonSize::XL),
                    )),
            ))
    }
}
