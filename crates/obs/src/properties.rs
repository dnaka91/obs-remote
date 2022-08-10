use std::{marker::PhantomData, ptr::NonNull};

use bitflags::bitflags;
use libobs_sys::obs_group_type::OBS_COMBO_INVALID;

use crate::{cstr_ptr, util::StringConversion};

pub struct Properties<'a> {
    raw: NonNull<libobs_sys::obs_properties_t>,
    life: PhantomData<&'a ()>,
    release: bool,
}

impl<'a> Drop for Properties<'a> {
    fn drop(&mut self) {
        if self.release {
            unsafe { libobs_sys::obs_properties_destroy(self.raw.as_ptr()) }
            self.release = false;
        }
    }
}

impl<'a> Properties<'a> {
    pub(crate) fn from_raw(raw: *mut libobs_sys::obs_properties_t) -> Self {
        Self {
            raw: unsafe { NonNull::new_unchecked(raw) },
            life: PhantomData,
            release: true,
        }
    }

    fn from_raw_no_release(raw: *mut libobs_sys::obs_properties_t) -> Self {
        Self {
            raw: unsafe { NonNull::new_unchecked(raw) },
            life: PhantomData,
            release: false,
        }
    }

    pub fn get(&self, property: &str) -> Option<Property<'_>> {
        let raw = unsafe { libobs_sys::obs_properties_get(self.raw.as_ptr(), cstr_ptr!(property)) };
        (!raw.is_null()).then(|| Property::from_raw(raw))
    }

    pub fn first(&self) -> Option<Property<'_>> {
        let raw = unsafe { libobs_sys::obs_properties_first(self.raw.as_ptr()) };
        (!raw.is_null()).then(|| Property::from_raw(raw))
    }

    pub fn flags(&self) -> PropertiesFlags {
        PropertiesFlags::from_bits_truncate(unsafe {
            libobs_sys::obs_properties_get_flags(self.raw.as_ptr())
        })
    }

    pub fn parent(&self) -> Option<Properties<'_>> {
        let raw = unsafe { libobs_sys::obs_properties_get_parent(self.raw.as_ptr()) };
        (!raw.is_null()).then(|| Self::from_raw_no_release(raw))
    }

    pub fn iter(&self) -> PropertyIter<'_> {
        PropertyIter(self.first())
    }

    // TODO: obs_properties_get_param
}

bitflags! {
    pub struct PropertiesFlags: u32 {
        const DEFER_UPDATE = libobs_sys::OBS_PROPERTIES_DEFER_UPDATE;
    }
}

pub struct PropertyIter<'a>(Option<Property<'a>>);

impl<'a> Iterator for PropertyIter<'a> {
    type Item = Property<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let p = self.0.take();
        self.0 = p.as_ref().and_then(|p| p.next());
        p
    }
}

pub struct Property<'a> {
    raw: NonNull<libobs_sys::obs_property_t>,
    life: PhantomData<&'a ()>,
}

impl<'a> Property<'a> {
    fn from_raw(raw: *mut libobs_sys::obs_property_t) -> Self {
        Self {
            raw: unsafe { NonNull::new_unchecked(raw) },
            life: PhantomData,
        }
    }

    pub fn next(&self) -> Option<Property<'a>> {
        let mut raw = self.raw.as_ptr();
        let exists = unsafe { libobs_sys::obs_property_next(&mut raw) };

        (exists && !raw.is_null()).then(|| Self::from_raw(raw))
    }

    pub fn name(&self) -> String {
        unsafe { libobs_sys::obs_property_name(self.raw.as_ptr()) }.into_string()
    }

    pub fn description(&self) -> String {
        unsafe { libobs_sys::obs_property_description(self.raw.as_ptr()) }.into_string()
    }

    pub fn long_description(&self) -> Option<String> {
        unsafe { libobs_sys::obs_property_long_description(self.raw.as_ptr()) }.into_opt_string()
    }

    pub fn enabled(&self) -> bool {
        unsafe { libobs_sys::obs_property_enabled(self.raw.as_ptr()) }
    }

    pub fn visible(&self) -> bool {
        unsafe { libobs_sys::obs_property_visible(self.raw.as_ptr()) }
    }

    pub fn ty(&self) -> PropertyType {
        PropertyType::from_native(unsafe { libobs_sys::obs_property_get_type(self.raw.as_ptr()) })
    }

    pub fn as_int(&self) -> Option<IntProperty<'_>> {
        (self.ty() == PropertyType::Int).then(|| IntProperty(self))
    }

    pub fn as_float(&self) -> Option<FloatProperty<'_>> {
        (self.ty() == PropertyType::Float).then(|| FloatProperty(self))
    }

    pub fn as_text(&self) -> Option<TextProperty<'_>> {
        (self.ty() == PropertyType::Text).then(|| TextProperty(self))
    }

    pub fn as_path(&self) -> Option<PathProperty<'_>> {
        (self.ty() == PropertyType::Path).then(|| PathProperty(self))
    }

    pub fn as_list(&self) {
        todo!()
    }

    pub fn as_button(&self) -> Option<ButtonProperty<'_>> {
        (self.ty() == PropertyType::Button).then(|| ButtonProperty(self))
    }

    pub fn as_typed(&self) -> Option<TypedProperty<'_>> {
        Some(match self.ty() {
            PropertyType::Int => TypedProperty::Int(IntProperty(self)),
            PropertyType::Float => TypedProperty::Float(FloatProperty(self)),
            PropertyType::Text => TypedProperty::Text(TextProperty(self)),
            PropertyType::Path => TypedProperty::Path(PathProperty(self)),
            PropertyType::List => TypedProperty::List(ListProperty(self)),
            PropertyType::Button => TypedProperty::Button(ButtonProperty(self)),
            PropertyType::EditableList => TypedProperty::EditableList(EditableListProperty(self)),
            PropertyType::FrameRate => TypedProperty::FrameRate(FrameRateProperty(self)),
            PropertyType::Group => TypedProperty::Group(GroupProperty(self)),
            _ => return None,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PropertyType {
    Invalid,
    Bool,
    Int,
    Float,
    Text,
    Path,
    List,
    Color,
    Button,
    Font,
    EditableList,
    FrameRate,
    Group,
    ColorAlpha,
    Unknown(u32),
}

impl PropertyType {
    fn from_native(ty: libobs_sys::obs_property_type::Type) -> Self {
        use libobs_sys::obs_property_type::*;

        match ty {
            OBS_PROPERTY_INVALID => Self::Invalid,
            OBS_PROPERTY_BOOL => Self::Bool,
            OBS_PROPERTY_INT => Self::Int,
            OBS_PROPERTY_FLOAT => Self::Float,
            OBS_PROPERTY_TEXT => Self::Text,
            OBS_PROPERTY_PATH => Self::Path,
            OBS_PROPERTY_LIST => Self::List,
            OBS_PROPERTY_COLOR => Self::Color,
            OBS_PROPERTY_BUTTON => Self::Button,
            OBS_PROPERTY_FONT => Self::Font,
            OBS_PROPERTY_EDITABLE_LIST => Self::EditableList,
            OBS_PROPERTY_FRAME_RATE => Self::FrameRate,
            OBS_PROPERTY_GROUP => Self::Group,
            OBS_PROPERTY_COLOR_ALPHA => Self::ColorAlpha,
            _ => Self::Unknown(ty as _),
        }
    }
}

pub enum TypedProperty<'a> {
    Int(IntProperty<'a>),
    Float(FloatProperty<'a>),
    Text(TextProperty<'a>),
    Path(PathProperty<'a>),
    List(ListProperty<'a>),
    Button(ButtonProperty<'a>),
    EditableList(EditableListProperty<'a>),
    FrameRate(FrameRateProperty<'a>),
    Group(GroupProperty<'a>),
}

pub struct IntProperty<'a>(&'a Property<'a>);

impl<'a> IntProperty<'a> {
    pub fn min(&self) -> i32 {
        unsafe { libobs_sys::obs_property_int_min(self.0.raw.as_ptr()) }
    }

    pub fn max(&self) -> i32 {
        unsafe { libobs_sys::obs_property_int_max(self.0.raw.as_ptr()) }
    }

    pub fn step(&self) -> i32 {
        unsafe { libobs_sys::obs_property_int_step(self.0.raw.as_ptr()) }
    }

    pub fn suffix(&self) -> Option<String> {
        unsafe { libobs_sys::obs_property_int_suffix(self.0.raw.as_ptr()) }.into_opt_string()
    }

    pub fn ty(&self) -> NumberType {
        NumberType::from_native(unsafe { libobs_sys::obs_property_int_type(self.0.raw.as_ptr()) })
    }
}

pub struct FloatProperty<'a>(&'a Property<'a>);

impl<'a> FloatProperty<'a> {
    pub fn min(&self) -> f64 {
        unsafe { libobs_sys::obs_property_float_min(self.0.raw.as_ptr()) }
    }

    pub fn max(&self) -> f64 {
        unsafe { libobs_sys::obs_property_float_max(self.0.raw.as_ptr()) }
    }

    pub fn step(&self) -> f64 {
        unsafe { libobs_sys::obs_property_float_step(self.0.raw.as_ptr()) }
    }

    pub fn suffix(&self) -> Option<String> {
        unsafe { libobs_sys::obs_property_float_suffix(self.0.raw.as_ptr()) }.into_opt_string()
    }

    pub fn ty(&self) -> NumberType {
        NumberType::from_native(unsafe { libobs_sys::obs_property_int_type(self.0.raw.as_ptr()) })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum NumberType {
    Scroller,
    Slider,
    Unknown(u32),
}

impl NumberType {
    fn from_native(ty: libobs_sys::obs_number_type::Type) -> Self {
        use libobs_sys::obs_number_type::*;

        match ty {
            OBS_NUMBER_SCROLLER => Self::Scroller,
            OBS_NUMBER_SLIDER => Self::Slider,
            _ => Self::Unknown(ty as _),
        }
    }
}

pub struct TextProperty<'a>(&'a Property<'a>);

impl<'a> TextProperty<'a> {
    pub fn monospace(&self) -> bool {
        unsafe { libobs_sys::obs_property_text_monospace(self.0.raw.as_ptr()) }
    }

    pub fn ty(&self) -> TextType {
        TextType::from_native(unsafe { libobs_sys::obs_property_text_type(self.0.raw.as_ptr()) })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TextType {
    Default,
    Password,
    Multiline,
    Unknown(u32),
}

impl TextType {
    fn from_native(ty: libobs_sys::obs_text_type::Type) -> Self {
        use libobs_sys::obs_text_type::*;

        match ty {
            OBS_TEXT_DEFAULT => Self::Default,
            OBS_TEXT_PASSWORD => Self::Password,
            OBS_TEXT_MULTILINE => Self::Multiline,
            _ => Self::Unknown(ty as _),
        }
    }
}

pub struct PathProperty<'a>(&'a Property<'a>);

impl<'a> PathProperty<'a> {
    pub fn default_path(&self) -> Option<String> {
        unsafe { libobs_sys::obs_property_path_default_path(self.0.raw.as_ptr()) }.into_opt_string()
    }

    pub fn filter(&self) -> String {
        unsafe { libobs_sys::obs_property_path_filter(self.0.raw.as_ptr()) }.into_string()
    }

    pub fn ty(&self) -> PathType {
        PathType::from_native(unsafe { libobs_sys::obs_property_path_type(self.0.raw.as_ptr()) })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum PathType {
    File,
    FileSave,
    Directory,
    Unknown(u32),
}

impl PathType {
    fn from_native(ty: libobs_sys::obs_path_type::Type) -> Self {
        use libobs_sys::obs_path_type::*;

        match ty {
            OBS_PATH_FILE => Self::File,
            OBS_PATH_FILE_SAVE => Self::FileSave,
            OBS_PATH_DIRECTORY => Self::Directory,
            _ => Self::Unknown(ty as _),
        }
    }
}

pub struct ButtonProperty<'a>(&'a Property<'a>);

impl<'a> ButtonProperty<'a> {
    pub fn url(&self) -> Option<String> {
        unsafe { libobs_sys::obs_property_button_url(self.0.raw.as_ptr()) }.into_opt_string()
    }

    pub fn ty(&self) -> ButtonType {
        ButtonType::from_native(unsafe {
            libobs_sys::obs_property_button_type(self.0.raw.as_ptr())
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ButtonType {
    Default,
    Url,
    Unknown(u32),
}

impl ButtonType {
    fn from_native(ty: libobs_sys::obs_button_type::Type) -> Self {
        use libobs_sys::obs_button_type::*;

        match ty {
            OBS_BUTTON_DEFAULT => Self::Default,
            OBS_BUTTON_URL => Self::Url,
            _ => Self::Unknown(ty as _),
        }
    }
}

pub struct ListProperty<'a>(&'a Property<'a>);

impl<'a> ListProperty<'a> {
    pub fn count(&self) -> u64 {
        unsafe { libobs_sys::obs_property_list_item_count(self.0.raw.as_ptr()) }
    }

    pub fn format(&self) -> ComboFormat {
        ComboFormat::from_native(unsafe {
            libobs_sys::obs_property_list_format(self.0.raw.as_ptr())
        })
    }

    pub fn item_name(&self, index: u64) -> String {
        unsafe { libobs_sys::obs_property_list_item_name(self.0.raw.as_ptr(), index) }.into_string()
    }

    pub fn item_disabled(&self, index: u64) -> bool {
        unsafe { libobs_sys::obs_property_list_item_disabled(self.0.raw.as_ptr(), index) }
    }

    pub fn item_int(&self, index: u64) -> i64 {
        unsafe { libobs_sys::obs_property_list_item_int(self.0.raw.as_ptr(), index) }
    }

    pub fn item_float(&self, index: u64) -> f64 {
        unsafe { libobs_sys::obs_property_list_item_float(self.0.raw.as_ptr(), index) }
    }

    pub fn item_string(&self, index: u64) -> String {
        unsafe { libobs_sys::obs_property_list_item_string(self.0.raw.as_ptr(), index) }
            .into_string()
    }

    pub fn ty(&self) -> ComboType {
        ComboType::from_native(unsafe { libobs_sys::obs_property_list_type(self.0.raw.as_ptr()) })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ComboFormat {
    Invalid,
    Int,
    Float,
    String,
    Unknown(u32),
}

impl ComboFormat {
    fn from_native(ty: libobs_sys::obs_combo_format::Type) -> Self {
        use libobs_sys::obs_combo_format::*;

        match ty {
            OBS_COMBO_FORMAT_INVALID => Self::Invalid,
            OBS_COMBO_FORMAT_INT => Self::Int,
            OBS_COMBO_FORMAT_FLOAT => Self::Float,
            OBS_COMBO_FORMAT_STRING => Self::String,
            _ => Self::Unknown(ty as _),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ComboType {
    Invalid,
    Editable,
    List,
    Unknown(u32),
}

impl ComboType {
    fn from_native(ty: libobs_sys::obs_combo_type::Type) -> Self {
        use libobs_sys::obs_combo_type::*;

        match ty {
            OBS_COMBO_INVALID => Self::Invalid,
            OBS_COMBO_TYPE_EDITABLE => Self::Editable,
            OBS_COMBO_TYPE_LIST => Self::List,
            _ => Self::Unknown(ty as _),
        }
    }
}

pub struct EditableListProperty<'a>(&'a Property<'a>);

impl<'a> EditableListProperty<'a> {
    pub fn default_path(&self) -> Option<String> {
        unsafe { libobs_sys::obs_property_editable_list_default_path(self.0.raw.as_ptr()) }
            .into_opt_string()
    }

    pub fn filter(&self) -> String {
        unsafe { libobs_sys::obs_property_editable_list_filter(self.0.raw.as_ptr()) }.into_string()
    }

    pub fn ty(&self) -> EditableListType {
        EditableListType::from_native(unsafe {
            libobs_sys::obs_property_editable_list_type(self.0.raw.as_ptr())
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum EditableListType {
    Strings,
    Files,
    FilesAndUrls,
    Unknown(u32),
}

impl EditableListType {
    fn from_native(ty: libobs_sys::obs_editable_list_type::Type) -> Self {
        use libobs_sys::obs_editable_list_type::*;

        match ty {
            OBS_EDITABLE_LIST_TYPE_STRINGS => Self::Strings,
            OBS_EDITABLE_LIST_TYPE_FILES => Self::Files,
            OBS_EDITABLE_LIST_TYPE_FILES_AND_URLS => Self::FilesAndUrls,
            _ => Self::Unknown(ty as _),
        }
    }
}

pub struct FrameRateProperty<'a>(&'a Property<'a>);

impl<'a> FrameRateProperty<'a> {
    pub fn fps_ranges_count(&self) -> u64 {
        unsafe { libobs_sys::obs_property_frame_rate_fps_ranges_count(self.0.raw.as_ptr()) }
    }

    pub fn fps_range_min(&self, index: u64) -> MediaFps {
        MediaFps::from_native(unsafe {
            libobs_sys::obs_property_frame_rate_fps_range_min(self.0.raw.as_ptr(), index)
        })
    }

    pub fn fps_range_max(&self, index: u64) -> MediaFps {
        MediaFps::from_native(unsafe {
            libobs_sys::obs_property_frame_rate_fps_range_max(self.0.raw.as_ptr(), index)
        })
    }

    pub fn fps_range_iter(&self) -> FpsRangeIter<'_> {
        FpsRangeIter {
            prop: self,
            count: self.fps_ranges_count(),
            pos: 0,
        }
    }

    pub fn options_count(&self) -> u64 {
        unsafe { libobs_sys::obs_property_frame_rate_options_count(self.0.raw.as_ptr()) }
    }

    pub fn option_name(&self, index: u64) -> String {
        unsafe { libobs_sys::obs_property_frame_rate_option_name(self.0.raw.as_ptr(), index) }
            .into_string()
    }

    pub fn option_description(&self, index: u64) -> String {
        unsafe {
            libobs_sys::obs_property_frame_rate_option_description(self.0.raw.as_ptr(), index)
        }
        .into_string()
    }

    pub fn option_iter(&self) -> OptionIter<'_> {
        OptionIter {
            prop: self,
            count: self.options_count(),
            pos: 0,
        }
    }
}

#[derive(Debug)]
pub struct MediaFps {
    pub numberator: u32,
    pub denominator: u32,
}

impl MediaFps {
    fn from_native(value: libobs_sys::media_frames_per_second) -> Self {
        Self {
            numberator: value.numerator,
            denominator: value.denominator,
        }
    }
}

pub struct FpsRangeIter<'a> {
    prop: &'a FrameRateProperty<'a>,
    count: u64,
    pos: u64,
}

impl<'a> Iterator for FpsRangeIter<'a> {
    type Item = (MediaFps, MediaFps);

    fn next(&mut self) -> Option<Self::Item> {
        (self.pos < self.count).then(|| {
            let min = self.prop.fps_range_min(self.pos);
            let max = self.prop.fps_range_max(self.pos);
            self.pos += 1;
            (min, max)
        })
    }
}

impl<'a> ExactSizeIterator for FpsRangeIter<'a> {
    fn len(&self) -> usize {
        self.count as _
    }
}

pub struct OptionIter<'a> {
    prop: &'a FrameRateProperty<'a>,
    count: u64,
    pos: u64,
}

impl<'a> Iterator for OptionIter<'a> {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        (self.pos < self.count).then(|| {
            let name = self.prop.option_name(self.pos);
            let description = self.prop.option_description(self.pos);
            self.pos += 1;
            (name, description)
        })
    }
}

impl<'a> ExactSizeIterator for OptionIter<'a> {
    fn len(&self) -> usize {
        self.count as _
    }
}

pub struct GroupProperty<'a>(&'a Property<'a>);

impl<'a> GroupProperty<'a> {
    pub fn content(&self) -> Properties<'a> {
        Properties::from_raw_no_release(unsafe {
            libobs_sys::obs_property_group_content(self.0.raw.as_ptr())
        })
    }

    pub fn ty(&self) -> GroupType {
        GroupType::from_native(unsafe { libobs_sys::obs_property_group_type(self.0.raw.as_ptr()) })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum GroupType {
    Invalid,
    Normal,
    Checkable,
    Unknown(u32),
}

impl GroupType {
    fn from_native(ty: libobs_sys::obs_group_type::Type) -> Self {
        use libobs_sys::obs_group_type::*;

        match ty {
            OBS_COMBO_INVALID => Self::Invalid,
            OBS_GROUP_NORMAL => Self::Normal,
            OBS_GROUP_CHECKABLE => Self::Checkable,
            _ => Self::Unknown(ty as _),
        }
    }
}
