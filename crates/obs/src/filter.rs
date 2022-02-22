use std::ptr::NonNull;

use crate::{source::Source, util};

pub struct Filter<'a> {
    raw: NonNull<libobs_sys::obs_source_t>,
    parent: &'a Source<'a>,
}

impl<'a> Drop for Filter<'a> {
    fn drop(&mut self) {
        unsafe { libobs_sys::obs_source_release(self.raw.as_ptr()) }
    }
}

impl<'a> Filter<'a> {
    pub(crate) fn from_raw(raw: *mut libobs_sys::obs_source_t, parent: &'a Source<'a>) -> Self {
        Self {
            raw: unsafe { NonNull::new_unchecked(raw) },
            parent,
        }
    }

    pub fn source(&self) -> Source<'_> {
        let raw = unsafe { libobs_sys::obs_source_get_ref(self.raw.as_ptr()) };
        Source::from_raw(raw)
    }

    pub fn into_source(self) -> Source<'a> {
        let raw = unsafe { libobs_sys::obs_source_get_ref(self.raw.as_ptr()) };
        Source::from_raw(raw)
    }

    pub fn set_order(&mut self, movement: OrderMovement) {
        unsafe {
            libobs_sys::obs_source_filter_set_order(
                self.parent.as_ptr(),
                self.raw.as_ptr(),
                movement.to_native(),
            )
        };
    }

    pub fn index(&self) -> Option<usize> {
        util::find_instance_of(
            self.parent.as_ptr(),
            self.raw.as_ptr(),
            libobs_sys::obs_source_enum_filters,
            libobs_sys::obs_source_get_ref,
            Source::from_raw,
        )
        .map(|(index, _)| index)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OrderMovement {
    Up,
    Down,
    Top,
    Bottom,
}

impl OrderMovement {
    pub(crate) fn to_native(self) -> libobs_sys::obs_order_movement::Type {
        use libobs_sys::obs_order_movement::*;

        match self {
            Self::Up => OBS_ORDER_MOVE_UP,
            Self::Down => OBS_ORDER_MOVE_DOWN,
            Self::Top => OBS_ORDER_MOVE_TOP,
            Self::Bottom => OBS_ORDER_MOVE_BOTTOM,
        }
    }
}
