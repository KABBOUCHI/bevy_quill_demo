#![feature(impl_trait_in_assoc_type)]

use bevy::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_mod_stylebuilder::*;
use bevy_quill::{Cx, Element, For, QuillPlugin, View, ViewTemplate};
use bevy_quill_obsidian::{
    controls::{Button, ListView},
    ObsidianUiPlugin,
};
use bevy_quill_obsidian_inspect::{InspectorPlugin, ValueRange};
use inline_tweak::tweak_fn;
use reflect::ResourcePropertyInspector;
mod reflect;

#[derive(Resource, Default, Clone, Reflect)]
struct Counter {
    #[reflect(@ValueRange::<i32>(-100..100))]
    value: i32,
}

#[tweak_fn]
fn main() {
    App::new()
        .init_resource::<Counter>()
        .add_plugins((
            DefaultPlugins,
            DefaultPickingPlugins,
            QuillPlugin,
            ObsidianUiPlugin,
            InspectorPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(
        node()
            .style(|ss: &mut StyleBuilder| {
                ss.align_items(AlignItems::Start);
            })
            .children((InspectorView, CounterView))
            .to_root(),
    );
}

#[derive(Clone, PartialEq)]
struct CounterView;

impl ViewTemplate for CounterView {
    type View = impl View;

    #[tweak_fn]
    fn create(&self, cx: &mut Cx) -> Self::View {
        let inc = cx.create_callback(|mut counter: ResMut<Counter>| {
            counter.value += 1;
        });

        let dec = cx.create_callback(|mut counter: ResMut<Counter>| {
            counter.value -= 1;
        });

        let counter = cx.use_resource::<Counter>();

        node()
            .style(|ss: &mut StyleBuilder| {
                ss.display(Display::Flex)
                    .margin(16)
                    .flex_direction(FlexDirection::Row)
                    .align_items(AlignItems::Center)
                    .border(3)
                    .gap(10)
                    .padding(3);
            })
            .children((
                btn().children("-").on_click(dec),
                format!("The count is: {}", counter.value),
                btn().children("+").on_click(inc),
            ))
    }
}

#[derive(Clone, PartialEq)]
struct InspectorView;

impl ViewTemplate for InspectorView {
    type View = impl View;

    fn create(&self, _cx: &mut Cx) -> Self::View {
        list_view()
            .style(|ss: &mut StyleBuilder| {
                ss.row_gap(16)
                    .flex_grow(1.)
                    .min_width(300)
                    .min_height(Val::Vh(100.));
            })
            .children(For::each(0..20, |_| {
                ResourcePropertyInspector::<Counter>::new()
            }))
    }
}

fn node() -> Element<NodeBundle> {
    Element::<NodeBundle>::new()
}

fn btn() -> Button {
    Button::new()
}

fn list_view() -> ListView {
    ListView::new()
}
