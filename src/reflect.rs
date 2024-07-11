use std::sync::Arc;

use bevy::prelude::*;
use bevy_quill::*;
use bevy_quill_obsidian_inspect::{InspectableResource, Inspector};

#[derive(Clone)]
pub struct ResourcePropertyInspector<T: Resource> {
    marker: std::marker::PhantomData<T>,
}

impl<T: Resource> PartialEq for ResourcePropertyInspector<T> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<T: Resource> ResourcePropertyInspector<T> {
    pub fn new() -> Self {
        Self {
            marker: std::marker::PhantomData,
        }
    }
}

impl<T: Resource + Reflect> ViewTemplate for ResourcePropertyInspector<T> {
    type View = impl View;
    fn create(&self, _cx: &mut Cx) -> Self::View {
        Inspector::new(Arc::<InspectableResource<T>>::default())
    }
}
