extern crate chrono;

use chrono::prelude::*;
use chrono::offset::LocalResult;
use chrono::{DateTime, Utc, Local, Duration};

pub fn get_type<'a, T: std::fmt::Debug>(_value: T) -> &'a str {
    std::any::type_name::<T>()
}
fn get_time_diff(datetimesince: Option<DateTime<Local>>) -> String {
    if let Some(datetimesince) = datetimesince {
        let now = Local::now();
        let diff = now - datetimesince;

        if diff < Duration::seconds(10) {
            return "just now".to_string();
        } else if diff < Duration::minutes(1) {
            return "a few seconds ago".to_string();
        } else if diff < Duration::hours(1) {
            return format!("about {} minutes ago", diff.num_minutes());
        } else if diff < Duration::days(1) {
            return format!("about {} hours ago", diff.num_hours());
        } else if diff < Duration::days(30) {
            return format!("about {} days ago", diff.num_days());
        } else if diff < Duration::days(365) {
            return format!("about {} months ago", diff.num_days() / 30);
        } else {
            return "more than a year ago".to_string();
        }
    } else {
        return "Invalid datetime!".to_string();
    }
}


fn convert_to_user_friendly_date_time_format_utc(datetime: DateTime<Utc>) -> String {
  let formatted_date = datetime.format("%A, %d/%m/%Y %I:%M %p").to_string();
  formatted_date
}

fn convert_to_user_friendly_date_time_format_local(datetime: DateTime<Local>) -> String {
    let formatted_date = datetime.format("%A, %d/%m/%Y %I:%M %p").to_string();
    formatted_date
}

fn main() {
    // Get the current date and time in the server's local time zone
    let local: DateTime<Local> = Local::now();
    
    // Get the current UTC date and time
    let utc: DateTime<Utc> = Utc::now();

    // Convert the DateTime<Local> to a NaiveDateTime
    let naive_local: NaiveDateTime = local.naive_local();
    
    // Convert the NaiveDateTime to DateTime<Local>
    let converted_local_time: DateTime<Local> = Local.from_local_datetime(&naive_local)
        .single()
        .expect("Failed to convert to local time");

    // Get the time zone offset for the local time zone
    let local_offset = local.offset();
    
    // Convert local time to UTC time (failed)
    // let utc_time = match local_offset.from_local_datetime(&local.naive_local()) {
    //     LocalResult::Single(utc) => utc,
    //     LocalResult::None => {
    //         // Handle non-existent local time (e.g., during daylight saving transitions)
    //         // You can choose how to handle this case, e.g., by using a fixed offset.
    //         println!("Non-existent local time. Handle this case.");
    //         return;
    //     },
    //     LocalResult::Ambiguous(_, _) => {
    //         // Handle ambiguous local time (e.g., during daylight saving transitions)
    //         // You can choose how to handle this case, e.g., by using a fixed offset.
    //         println!("Ambiguous local time. Handle this case.");
    //         return;
    //     },
    // };


    println!("Current server time in local time zone: {} type: {}", naive_local, get_type(naive_local));
    println!("DateTime<Local>: {}", converted_local_time);
    println!("Local offset: {}", local_offset); // +07:00
    // println!("DateTime<Utc>:   {}", utc_time);   // failed

    println!("Local time: {}, type: {}", local, get_type(local));
    println!("UTC time:   {}, type: {}", utc, get_type(local));

    let minus_local: DateTime<Local> = local - Duration::minutes(5);
    println!("Local time - 5m: {}", minus_local);
    
    let time_dif: String = get_time_diff(Some(minus_local));
    println!("{}", time_dif);


    let current_user_friendly_datetime: String = convert_to_user_friendly_date_time_format_local(converted_local_time);
    println!("Current User-friendly datetime: {}", current_user_friendly_datetime);

    // Output run: 
    // Current server time in local time zone: 2023-11-03 08:47:51.285489 type: chrono::naive::datetime::NaiveDateTime
    // DateTime<Local>: 2023-11-03 08:47:51.285489 +07:00
    // Local offset: +07:00
    // Local time: 2023-11-03 08:47:51.285489 +07:00, type: chrono::datetime::DateTime<chrono::offset::local::Local>
    // UTC time:   2023-11-03 01:47:51.285756 UTC, type: chrono::datetime::DateTime<chrono::offset::local::Local>
    // Local time - 5m: 2023-11-03 08:42:51.285489 +07:00
    // about 5 minutes ago
    // Current User-friendly datetime: Friday, 03/11/2023 08:47 AM
}
