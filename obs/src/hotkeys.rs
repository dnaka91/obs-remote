use crate::{data::Data, service::Service};

pub fn save_service(service: &Service) -> Data {
    Data::from_raw(unsafe { libobs_sys::obs_hotkeys_save_service(service.as_ptr()) })
}
