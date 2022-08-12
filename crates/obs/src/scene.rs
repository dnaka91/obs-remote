use std::{ffi::c_void, marker::PhantomData, ops::Deref, ptr::NonNull};

use bitflags::bitflags;

use crate::{cstr_ptr, graphics::Vec2, source::Source, video::ScaleType};

pub struct Scene<'a> {
    raw: NonNull<libobs_sys::obs_scene_t>,
    life: PhantomData<&'a ()>,
}

impl<'a> Drop for Scene<'a> {
    fn drop(&mut self) {
        unsafe { libobs_sys::obs_scene_release(self.raw.as_ptr()) }
    }
}

impl<'a> Scene<'a> {
    pub(crate) fn from_raw(raw: *mut libobs_sys::obs_scene_t) -> Self {
        Self {
            raw: unsafe { NonNull::new_unchecked(raw) },
            life: PhantomData::default(),
        }
    }

    pub fn create(name: &str) -> Self {
        Self::from_raw(unsafe { libobs_sys::obs_scene_create(cstr_ptr!(name)) })
    }

    pub fn from_source(source: Source<'a>) -> Option<Self> {
        let raw = unsafe { libobs_sys::obs_scene_from_source(source.as_ptr()) };

        (!raw.is_null()).then(|| Self::from_raw(unsafe { libobs_sys::obs_scene_get_ref(raw) }))
    }

    pub fn list_items(&self) -> Vec<SceneItem<'_>> {
        unsafe extern "C" fn callback(
            _scene: *mut libobs_sys::obs_scene_t,
            item: *mut libobs_sys::obs_sceneitem_t,
            param: *mut c_void,
        ) -> bool {
            if !item.is_null() {
                libobs_sys::obs_sceneitem_addref(item);

                let param = &mut *param.cast::<Vec<SceneItem<'_>>>();
                param.push(SceneItem::from_raw(item));
            }

            true
        }

        let mut param = Vec::<SceneItem<'_>>::new();
        unsafe {
            libobs_sys::obs_scene_enum_items(
                self.raw.as_ptr(),
                Some(callback),
                (&mut param as *mut Vec<_>).cast(),
            )
        };

        param
    }

    pub fn source(&self) -> Source<'_> {
        Source::from_raw(unsafe {
            let raw = libobs_sys::obs_scene_get_source(self.raw.as_ptr());
            libobs_sys::obs_source_get_ref(raw)
        })
    }

    pub fn into_source(self) -> Source<'a> {
        Source::from_raw(unsafe {
            let raw = libobs_sys::obs_scene_get_source(self.raw.as_ptr());
            libobs_sys::obs_source_get_ref(raw)
        })
    }

    pub fn atomic_update<F, T>(&mut self, update: F) -> T
    where
        F: FnOnce(&mut Scene<'_>) -> T,
        T: Default,
    {
        struct Param<F, T> {
            update: Option<F>,
            result: Option<T>,
        }

        unsafe extern "C" fn callback<F, T>(param: *mut c_void, raw: *mut libobs_sys::obs_scene_t)
        where
            F: FnOnce(&mut Scene<'_>) -> T,
        {
            let param = &mut *param.cast::<Param<F, T>>();
            if let Some(update) = param.update.take() {
                param.result = Some(update(&mut Scene::from_raw(raw)));
            }
        }

        let mut param = Param {
            update: Some(update),
            result: None,
        };

        unsafe {
            libobs_sys::obs_scene_atomic_update(
                self.raw.as_ptr(),
                Some(callback::<F, T>),
                (&mut param as *mut Param<_, _>).cast(),
            )
        };

        param.result.unwrap_or_default()
    }

    pub fn add(&mut self, source: &Source<'_>) -> SceneItem<'_> {
        let raw = unsafe { libobs_sys::obs_scene_add(self.raw.as_ptr(), source.as_ptr()) };
        unsafe { libobs_sys::obs_sceneitem_addref(raw) };
        SceneItem::from_raw(raw)
    }

    pub fn prune_sources(&mut self) {
        unsafe { libobs_sys::obs_scene_prune_sources(self.raw.as_ptr()) };
    }
}

pub struct SceneItem<'a> {
    raw: NonNull<libobs_sys::obs_sceneitem_t>,
    life: PhantomData<&'a ()>,
}

impl<'a> Drop for SceneItem<'a> {
    fn drop(&mut self) {
        unsafe { libobs_sys::obs_sceneitem_release(self.raw.as_ptr()) };
    }
}

impl<'a> SceneItem<'a> {
    pub(crate) fn from_raw(raw: *mut libobs_sys::obs_sceneitem_t) -> Self {
        Self {
            raw: unsafe { NonNull::new_unchecked(raw) },
            life: PhantomData::default(),
        }
    }

    pub fn id(&self) -> i64 {
        unsafe { libobs_sys::obs_sceneitem_get_id(self.raw.as_ptr()) }
    }

    pub fn pos(&self) -> (f32, f32) {
        let mut pos = Vec2::default();

        unsafe { libobs_sys::obs_sceneitem_get_pos(self.raw.as_ptr(), pos.as_ptr_mut()) };

        (pos.x(), pos.y())
    }

    pub fn alignment(&self) -> Alignment {
        Alignment::from_bits_truncate(unsafe {
            libobs_sys::obs_sceneitem_get_alignment(self.raw.as_ptr())
        })
    }

    pub fn rot(&self) -> f32 {
        unsafe { libobs_sys::obs_sceneitem_get_rot(self.raw.as_ptr()) }
    }

    pub fn scale(&self) -> (f32, f32) {
        let mut scale = Vec2::default();

        unsafe { libobs_sys::obs_sceneitem_get_scale(self.raw.as_ptr(), scale.as_ptr_mut()) };

        (scale.x(), scale.y())
    }

    pub fn scale_filter(&self) -> ScaleType {
        ScaleType::from_native(unsafe {
            libobs_sys::obs_sceneitem_get_scale_filter(self.raw.as_ptr())
        })
    }

    pub fn blending_method(&self) -> BlendingMethod {
        BlendingMethod::from_native(unsafe {
            libobs_sys::obs_sceneitem_get_blending_method(self.raw.as_ptr())
        })
    }

    pub fn blending_mode(&self) -> BlendingType {
        BlendingType::from_native(unsafe {
            libobs_sys::obs_sceneitem_get_blending_mode(self.raw.as_ptr())
        })
    }

    pub fn crop(&self) -> (i32, i32, i32, i32) {
        let mut crop = libobs_sys::obs_sceneitem_crop::default();

        unsafe { libobs_sys::obs_sceneitem_get_crop(self.raw.as_ptr(), &mut crop as *mut _) };

        (crop.left, crop.top, crop.right, crop.bottom)
    }

    pub fn bounds(&self) -> (f32, f32) {
        let mut bounds = Vec2::default();

        unsafe { libobs_sys::obs_sceneitem_get_bounds(self.raw.as_ptr(), bounds.as_ptr_mut()) };

        (bounds.x(), bounds.y())
    }

    pub fn bounds_type(&self) -> BoundsType {
        BoundsType::from_native(unsafe {
            libobs_sys::obs_sceneitem_get_bounds_type(self.raw.as_ptr())
        })
    }

    pub fn bounds_alignment(&self) -> Alignment {
        Alignment::from_bits_truncate(unsafe {
            libobs_sys::obs_sceneitem_get_bounds_alignment(self.raw.as_ptr())
        })
    }

    pub fn visible(&self) -> bool {
        unsafe { libobs_sys::obs_sceneitem_visible(self.raw.as_ptr()) }
    }

    pub fn set_visible(&self, visible: bool) {
        unsafe { libobs_sys::obs_sceneitem_set_visible(self.raw.as_ptr(), visible) };
    }

    pub fn locked(&self) -> bool {
        unsafe { libobs_sys::obs_sceneitem_locked(self.raw.as_ptr()) }
    }

    pub fn set_locked(&self, locked: bool) {
        unsafe { libobs_sys::obs_sceneitem_set_locked(self.raw.as_ptr(), locked) };
    }

    pub fn is_group(&self) -> bool {
        unsafe { libobs_sys::obs_sceneitem_is_group(self.raw.as_ptr()) }
    }

    pub fn source(&self) -> Source<'_> {
        Source::from_raw(unsafe {
            let raw = libobs_sys::obs_sceneitem_get_source(self.raw.as_ptr());
            libobs_sys::obs_source_get_ref(raw)
        })
    }

    pub fn parent_scene(&self) -> Option<Scene<'_>> {
        let raw = unsafe { libobs_sys::obs_sceneitem_get_scene(self.raw.as_ptr()) };
        (!raw.is_null()).then(|| Scene::from_raw(unsafe { libobs_sys::obs_scene_get_ref(raw) }))
    }

    pub fn list_group_items(&self) -> Option<Vec<Self>> {
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

                let param = &mut *param.cast::<Vec<SceneItem<'_>>>();
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

        Some(param)
    }

    pub fn remove(&self) {
        unsafe { libobs_sys::obs_sceneitem_remove(self.raw.as_ptr()) };
    }

    pub fn update<F>(&mut self, f: F)
    where
        F: FnOnce(&mut EditableSceneItem<'_, '_>),
    {
        unsafe { libobs_sys::obs_sceneitem_defer_update_begin(self.raw.as_ptr()) };
        f(&mut EditableSceneItem(self));
        unsafe { libobs_sys::obs_sceneitem_defer_update_end(self.raw.as_ptr()) };
    }
}

pub struct EditableSceneItem<'a, 'b>(&'a mut SceneItem<'b>);

impl<'a, 'b> EditableSceneItem<'a, 'b> {
    pub fn set_pos(&mut self, pos: (f32, f32)) {
        let pos = Vec2::new(pos.0, pos.1);
        unsafe { libobs_sys::obs_sceneitem_set_pos(self.0.raw.as_ptr(), pos.as_ptr()) };
    }

    pub fn set_alignment(&mut self, alignment: Alignment) {
        unsafe { libobs_sys::obs_sceneitem_set_alignment(self.0.raw.as_ptr(), alignment.bits()) };
    }

    pub fn set_rot(&mut self, rot: f32) {
        unsafe { libobs_sys::obs_sceneitem_set_rot(self.0.raw.as_ptr(), rot) };
    }

    pub fn set_scale(&mut self, scale: (f32, f32)) {
        let scale = Vec2::new(scale.0, scale.1);
        unsafe { libobs_sys::obs_sceneitem_set_scale(self.0.raw.as_ptr(), scale.as_ptr()) };
    }

    pub fn set_scale_filter(&mut self, filter: ScaleType) {
        unsafe {
            libobs_sys::obs_sceneitem_set_scale_filter(self.0.raw.as_ptr(), filter.to_native())
        };
    }

    pub fn set_blending_method(&mut self, method: BlendingMethod) {
        unsafe {
            libobs_sys::obs_sceneitem_set_blending_method(self.0.raw.as_ptr(), method.to_native())
        };
    }

    pub fn set_blending_mode(&mut self, mode: BlendingType) {
        unsafe {
            libobs_sys::obs_sceneitem_set_blending_mode(self.0.raw.as_ptr(), mode.to_native())
        };
    }

    pub fn set_crop(&mut self, crop: (i32, i32, i32, i32)) {
        let crop = libobs_sys::obs_sceneitem_crop {
            left: crop.0,
            top: crop.1,
            right: crop.2,
            bottom: crop.3,
        };
        unsafe { libobs_sys::obs_sceneitem_set_crop(self.0.raw.as_ptr(), &crop as *const _) };
    }

    pub fn set_bounds(&mut self, bounds: (f32, f32)) {
        let bounds = Vec2::new(bounds.0, bounds.1);
        unsafe { libobs_sys::obs_sceneitem_set_bounds(self.0.raw.as_ptr(), bounds.as_ptr()) };
    }

    pub fn set_bounds_type(&mut self, ty: BoundsType) {
        unsafe { libobs_sys::obs_sceneitem_set_bounds_type(self.0.raw.as_ptr(), ty.to_native()) };
    }

    pub fn set_bounds_alignment(&mut self, alignment: Alignment) {
        unsafe {
            libobs_sys::obs_sceneitem_set_bounds_alignment(self.0.raw.as_ptr(), alignment.bits())
        };
    }
}

impl<'a, 'b> Deref for EditableSceneItem<'a, 'b> {
    type Target = SceneItem<'b>;

    fn deref(&self) -> &Self::Target {
        self.0
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

/// Used with scene items to indicate the type of bounds to use for scene items. Mostly determines
/// how the image will be scaled within those bounds, or whether to use bounds at all.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug)]
pub enum BoundsType {
    /// No bounds.
    None,
    /// Stretch (ignores base scale).
    Stretch,
    /// Scales to inner rectangle.
    ScaleInner,
    /// Scales to outer rectangle.
    ScaleOuter,
    /// Scales to the width.
    ScaleToWidth,
    /// Scales to the height.
    ScaleToHeight,
    /// No scaling, maximum size only.
    MaxOnly,
    Unknown(u32),
}

impl BoundsType {
    fn from_native(value: libobs_sys::obs_bounds_type::Type) -> Self {
        use libobs_sys::obs_bounds_type::*;

        match value {
            OBS_BOUNDS_NONE => Self::None,
            OBS_BOUNDS_STRETCH => Self::Stretch,
            OBS_BOUNDS_SCALE_INNER => Self::ScaleInner,
            OBS_BOUNDS_SCALE_OUTER => Self::ScaleOuter,
            OBS_BOUNDS_SCALE_TO_WIDTH => Self::ScaleToWidth,
            OBS_BOUNDS_SCALE_TO_HEIGHT => Self::ScaleToHeight,
            OBS_BOUNDS_MAX_ONLY => Self::MaxOnly,
            _ => Self::Unknown(value as u32),
        }
    }

    fn to_native(self) -> libobs_sys::obs_bounds_type::Type {
        use libobs_sys::obs_bounds_type::*;

        match self {
            Self::None => OBS_BOUNDS_NONE,
            Self::Stretch => OBS_BOUNDS_STRETCH,
            Self::ScaleInner => OBS_BOUNDS_SCALE_INNER,
            Self::ScaleOuter => OBS_BOUNDS_SCALE_OUTER,
            Self::ScaleToWidth => OBS_BOUNDS_SCALE_TO_WIDTH,
            Self::ScaleToHeight => OBS_BOUNDS_SCALE_TO_HEIGHT,
            Self::MaxOnly => OBS_BOUNDS_MAX_ONLY,
            Self::Unknown(value) => value as _,
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug)]
pub enum BlendingMethod {
    Default,
    SrgbOff,
    Unknown(u32),
}

impl BlendingMethod {
    pub(crate) fn from_native(ty: libobs_sys::obs_blending_method::Type) -> Self {
        use libobs_sys::obs_blending_method::*;

        match ty {
            OBS_BLEND_METHOD_DEFAULT => Self::Default,
            OBS_BLEND_METHOD_SRGB_OFF => Self::SrgbOff,
            _ => Self::Unknown(ty as _),
        }
    }

    pub(crate) fn to_native(self) -> libobs_sys::obs_blending_method::Type {
        use libobs_sys::obs_blending_method::*;

        match self {
            Self::Default => OBS_BLEND_METHOD_DEFAULT,
            Self::SrgbOff => OBS_BLEND_METHOD_SRGB_OFF,
            Self::Unknown(value) => value as _,
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug)]
pub enum BlendingType {
    Normal,
    Additive,
    Subtract,
    Screen,
    Multiply,
    Lighten,
    Darken,
    Unknown(u32),
}

impl BlendingType {
    pub(crate) fn from_native(value: libobs_sys::obs_blending_type::Type) -> Self {
        use libobs_sys::obs_blending_type::*;

        match value {
            OBS_BLEND_NORMAL => Self::Normal,
            OBS_BLEND_ADDITIVE => Self::Additive,
            OBS_BLEND_SUBTRACT => Self::Subtract,
            OBS_BLEND_SCREEN => Self::Screen,
            OBS_BLEND_MULTIPLY => Self::Multiply,
            OBS_BLEND_LIGHTEN => Self::Lighten,
            OBS_BLEND_DARKEN => Self::Darken,
            _ => Self::Unknown(value as _),
        }
    }

    pub(crate) fn to_native(self) -> libobs_sys::obs_blending_type::Type {
        use libobs_sys::obs_blending_type::*;

        match self {
            Self::Normal => OBS_BLEND_NORMAL,
            Self::Additive => OBS_BLEND_ADDITIVE,
            Self::Subtract => OBS_BLEND_SUBTRACT,
            Self::Screen => OBS_BLEND_SCREEN,
            Self::Multiply => OBS_BLEND_MULTIPLY,
            Self::Lighten => OBS_BLEND_LIGHTEN,
            Self::Darken => OBS_BLEND_LIGHTEN,
            Self::Unknown(value) => value as _,
        }
    }
}
