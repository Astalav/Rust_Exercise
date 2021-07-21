use std::io::Error;
use std::io::ErrorKind;
use std::fs;

use super::storage;
use crate::util::month_converter;

pub fn load() -> Result<(), Error>{
    let csv = match fs::read_to_string(".\\assets\\hurricanes.csv"){
        Ok(file_content) => file_content,
        Err(error) => return Err(error),
    };

    let mut most_common_month: (u8, f32) = (0, 0.0);
    let mut propability_by_month: [f32; 12] = [0.0; 12];

    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    for record in reader.records() {
        let record = record?;

        most_common_month = update_most_common_month(most_common_month, &record);
        propability_by_month = update_propability_by_month(propability_by_month, &record);
    }
    
    match storage::set_most_common_month(most_common_month.0, most_common_month.1) {
        Ok(_) => (),
        Err(error) => return Err(Error::new(ErrorKind::Other, error)),
    };

    storage::set_propability_by_month(propability_by_month);

    return Ok(())
}

fn update_most_common_month(most_common_month: (u8, f32), record: &csv::StringRecord) -> (u8, f32) {
    let month_average = record[1].trim().to_string().parse::<f32>().unwrap();

    if month_average > most_common_month.1 {
        let month_index = month_converter::get_index_from_month_name(&record[0]);
        return (month_index, month_average);
    } else {
        return most_common_month
    }
}

fn update_propability_by_month(propability_by_month: [f32; 12], record: &csv::StringRecord) -> [f32; 12] {
    let record_length = record.len() - 1;
    let number_of_years = record.len() - 2;

    let mut years_with_hurricanes = 0;

    for i in 2..record_length {
        let hurricane_count_for_year = record[i].trim().to_string().parse::<u8>().unwrap();
        if hurricane_count_for_year > 0 {
            years_with_hurricanes+=1;
        }
    };

    if years_with_hurricanes == 0 {
        return propability_by_month
    } else {
        let month_index = month_converter::get_index_from_month_name(&record[0]);
        let month_average = years_with_hurricanes as f32 / number_of_years as f32;
        let mut new_array = propability_by_month.clone();

        new_array[usize::from(month_index)] = month_average; 
        return new_array
    }
}