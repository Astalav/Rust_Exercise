pub static mut MOST_COMMON_MONTH: Option<(u8, f32)> = None;

pub static mut PROPABILITY_BY_MONTH: Option<[f32; 12]> = None;

pub fn set_most_common_month(month: u8, occurances: f32) -> Result<(), &'static str> {
    if month < 1 || month > 12 { 
        Err("Invalid Input!")
    } else { 
        unsafe {
            // Since we use this as a simple one-time write, n-time access Data storage unsafe should be fine here
            MOST_COMMON_MONTH = Some((month, occurances));
        }
        Ok(())
    }
}

pub fn set_propability_by_month(propability: [f32; 12]) {
    unsafe {
        // Since we use this as a simple one-time write, n-time access Data storage unsafe should be fine here
        PROPABILITY_BY_MONTH = Some(propability);
    }
}