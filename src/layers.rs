use kanata_keyberon::action::*;
use kanata_keyberon::layout::*;

use crate::custom_action::*;

macro_rules! empty_layer {
    () => {
        [
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
            Action::Trans,
        ]
    };
}

pub(crate) use empty_layer;

pub const MAX_LAYERS: usize = 25;
pub const ACTUAL_NUM_LAYERS: usize = MAX_LAYERS * 2;

pub type KanataLayers = Layers<256, 1, ACTUAL_NUM_LAYERS, CustomAction>;

pub fn new_layers() -> Box<KanataLayers> {
    let boxed_slice: Box<[[kanata_keyberon::action::Action<CustomAction>; 256]]> = {
        let mut layers = Vec::with_capacity(ACTUAL_NUM_LAYERS);
        for _ in 0..ACTUAL_NUM_LAYERS {
            layers.push(empty_layer!());
        }
        layers
    }.into_boxed_slice();
    let ptr = Box::into_raw(boxed_slice) as *mut KanataLayers;
    unsafe { Box::from_raw(ptr) }
}
