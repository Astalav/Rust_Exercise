use super::storage;

pub fn get_year_and_month_with_most_hurricanes() -> (u16, u8, u8) {
    unsafe {
        match storage::YEAR_AND_MONTH_WITH_MOST_HURRICANES {
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