
pub static mut YEAR_AND_MONTH_WITH_MOST_HURRICANES: Option<(u16, u8, u8)> = None;

pub static mut PROPABILITY_BY_MONTH: Option<[f32; 12]> = None;

pub fn set_year_and_month_with_most_hurricanes((year, month, occurances): (u16 , u8, u8)) -> Result<(), &'static str> {
    if month < 1 || month > 12 { 
        Err("Invalid Input!")
    } else { 
        unsafe {
            // Since we use this as a simple one-time write, n-time access Data storage unsafe should be fine here
            YEAR_AND_MONTH_WITH_MOST_HURRICANES = Some((year, month, occurances));
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