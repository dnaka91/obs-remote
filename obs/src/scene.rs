use std::{ffi::c_void, ptr::NonNull};

use bitflags::bitflags;

use crate::{source::Source, Ref, Vec2};

pub struct Scene {
    raw: NonNull<libobs_sys::obs_scene_t>,
}

impl Drop for Scene {
    fn drop(&mut self) {
        unsafe { libobs_sys::obs_scene_release(self.raw.as_ptr()) }
    }
}

impl Scene {
    pub(crate) fn from_raw(raw: *mut libobs_sys::obs_scene_t) -> Self {
        Self {
            raw: unsafe { NonNull::new_unchecked(raw) },
        }
    }

    pub fn from_source(source: &Source) -> Option<Ref<'_, Source, Self>> {
        let raw = unsafe { libobs_sys::obs_scene_from_source(source.as_ptr()) };
        if raw.is_null() {
            None
        } else {
            unsafe { libobs_sys::obs_scene_addref(raw) };
            Some(Ref::new(Self::from_raw(raw)))
        }
    }

    pub fn list_items(&self) -> Ref<'_, Self, Vec<SceneItem>> {
        unsafe extern "C" fn callback(
            _scene: *mut libobs_sys::obs_scene_t,
            item: *mut libobs_sys::obs_sceneitem_t,
            param: *mut c_void,
        ) -> bool {
            if !item.is_null() {
                libobs_sys::obs_sceneitem_addref(item);

                let param = &mut *param.cast::<Vec<SceneItem>>();
                param.push(SceneItem::from_raw(item));
            }

            true
        }

        let mut param = Vec::<SceneItem>::new();
        unsafe {
            libobs_sys::obs_scene_enum_items(
                self.raw.as_ptr(),
                Some(callback),
                (&mut param as *mut Vec<_>).cast(),
            )
        };

        Ref::new(param)
    }

    pub fn source(&self) -> Ref<'_, Self, Source> {
        Ref::new(Source::from_raw(unsafe {
            let raw = libobs_sys::obs_scene_get_source(self.raw.as_ptr());
            libobs_sys::obs_source_get_ref(raw)
        }))
    }
}

pub struct SceneItem {
    raw: NonNull<libobs_sys::obs_sceneitem_t>,
}

impl Drop for SceneItem {
    fn drop(&mut self) {
        unsafe { libobs_sys::obs_sceneitem_release(self.raw.as_ptr()) };
    }
}

impl SceneItem {
    pub(crate) fn from_raw(raw: *mut libobs_sys::obs_sceneitem_t) -> Self {
        Self {
            raw: unsafe { NonNull::new_unchecked(raw) },
        }
    }

    pub fn id(&self) -> i64 {
        unsafe { libobs_sys::obs_sceneitem_get_id(self.raw.as_ptr()) }
    }

    pub fn pos(&self) -> (f32, f32) {
        let mut pos = Vec2::default();

        unsafe { libobs_sys::obs_sceneitem_get_pos(self.raw.as_ptr(), pos.as_ptr()) };

        (pos.x(), pos.y())
    }

    pub fn scale(&self) -> (f32, f32) {
        let mut scale = Vec2::default();

        unsafe { libobs_sys::obs_sceneitem_get_scale(self.raw.as_ptr(), scale.as_ptr()) };

        (scale.x(), scale.y())
    }

    pub fn alignment(&self) -> Alignment {
        Alignment::from_bits_truncate(unsafe {
            libobs_sys::obs_sceneitem_get_alignment(self.raw.as_ptr())
        })
    }

    pub fn visible(&self) -> bool {
        unsafe { libobs_sys::obs_sceneitem_visible(self.raw.as_ptr()) }
    }

    pub fn locked(&self) -> bool {
        unsafe { libobs_sys::obs_sceneitem_locked(self.raw.as_ptr()) }
    }

    pub fn is_group(&self) -> bool {
        unsafe { libobs_sys::obs_sceneitem_is_group(self.raw.as_ptr()) }
    }

    pub fn source(&self) -> Ref<'_, Self, Source> {
        Ref::new(Source::from_raw(unsafe {
            let raw = libobs_sys::obs_sceneitem_get_source(self.raw.as_ptr());
            libobs_sys::obs_source_get_ref(raw)
        }))
    }

    pub fn parent_scene(&self) -> Option<Ref<'_, Self, Scene>> {
        let raw = unsafe { libobs_sys::obs_sceneitem_get_scene(self.raw.as_ptr()) };
        (!raw.is_null()).then(|| {
            unsafe { libobs_sys::obs_scene_addref(raw) };
            Ref::new(Scene::from_raw(raw))
        })
    }

    pub fn list_group_items(&self) -> Option<Ref<'_, Self, Vec<Self>>> {
        if !self.is_group() {
            return None;
        }

        unsafe extern "C" fn callback(
            _scene: *mut libobs_sys::obs_scene_t,
            item: *mut libobs_sys::obs_sceneitem_t,
            param: *mut c_void,
        ) -> bool {
            if !item.is_null() {
                libobs_sys::obs_sceneitem_addref(item);

                let param = &mut *param.cast::<Vec<SceneItem>>();
                param.push(SceneItem::from_raw(item));
            }

            true
        }

        let mut param = Vec::<Self>::new();
        unsafe {
            libobs_sys::obs_sceneitem_group_enum_items(
                self.raw.as_ptr(),
                Some(callback),
                (&mut param as *mut Vec<_>).cast(),
            )
        };

        Some(Ref::new(param))
    }
}

bitflags! {
    pub struct Alignment: u32 {
        const CENTER = libobs_sys::OBS_ALIGN_CENTER;
        const LEFT = libobs_sys::OBS_ALIGN_LEFT;
        const RIGHT = libobs_sys::OBS_ALIGN_RIGHT;
        const TOP = libobs_sys::OBS_ALIGN_TOP;
        const BOTTOM = libobs_sys::OBS_ALIGN_BOTTOM;
    }
}
