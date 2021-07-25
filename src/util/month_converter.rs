pub fn get_index_from_month_name(month_name: &str) -> u8 {
    match month_name {
        "Jan" => 0,
        "Feb" => 1,
        "Mar" => 2,
        "Apr" => 3,
        "May" => 4,
        "Jun" => 5,
        "Jul" => 6,
        "Aug" => 7,
        "Sep" => 8,
        "Oct" => 9,
        "Nov" => 10,
        "Dec" => 11,
        &_ => panic!("Error while parsing input"),
    }
}

#[test]
fn get_index_from_month_name_test() {
    assert_eq!(get_index_from_month_name("Jan"), 0);
    assert_eq!(get_index_from_month_name("Dec"), 11);
}

#[test]
#[should_panic]
fn get_index_from_month_name_test_panic() {
    get_index_from_month_name("any");
}

pub fn get_month_name_from_index(month_index: u8) -> &'static str {
    match month_index {
        0 => "Jan",
        1 => "Feb",
        2 => "Mar",
        3 => "Apr",
        4 => "May",
        5 => "Jun",
        6 => "Jul",
        7 => "Aug",
        8 => "Sep",
        9 => "Oct",
        10 => "Nov",
        11 => "Dec",
        _ => panic!("Error while parsing input"),
    }
}

#[test]
fn get_month_name_from_index_test() {
    assert_eq!(get_month_name_from_index(0), "Jan");
    assert_eq!(get_month_name_from_index(11), "Dec");
}

#[test]
#[should_panic]
fn get_month_name_from_index_test_panic() {
    get_month_name_from_index(12);
}