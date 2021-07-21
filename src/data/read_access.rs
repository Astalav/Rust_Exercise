use super::storage;

pub fn get_most_common_month() -> (u8, f32) {
    unsafe {
        match storage::MOST_COMMON_MONTH {
            Some(data) => return data,
            None => panic!("Datastorage is not set yet! Access to uninitialized data is impossible.")
        }
    }
}

pub fn get_propability_by_month(month: u8) -> Result<f32, &'static str> {
    unsafe {
        match storage::PROPABILITY_BY_MONTH {
            Some(data) => Ok(data[month as usize]),
            None => Err("Datastorage is not set yet! Access to uninitialized data is impossible.")
        }
    }
}