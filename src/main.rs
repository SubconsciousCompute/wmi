#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::Deserialize;
use wmi::{COMLibrary, WMIConnection};

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_ProcessStartTrace")]
#[serde(rename_all = "PascalCase")]
struct ProcessStartTrace {
    process_id: u32,
    parent_process_id: u32,
    process_name: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let com_con = COMLibrary::new()?;
    let wmi_con = WMIConnection::new(com_con)?;

    let iterator = wmi_con.notification::<ProcessStartTrace>()?;

    for result in iterator {
        let trace = result?;
        println!("Process started!");
        println!("{trace:#?}");
    } // Loop will end only on error

    Ok(())
}