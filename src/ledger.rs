use std::fs::File;
use std::io::Write;
use std::error::Error;
use chrono::{Local, Datelike};

pub fn ledger_log(wallet: String, time_amount: f32) -> Result<(), Box<dyn Error>>{

    let mut file = File::options().append(true).open("log/ledger.log")?;

    // Get today's date
    let today = Local::today();

    // Format the date in the desired format (YYYY/MM/DD)
    let formatted_date = today.format("%Y/%m/%d").to_string();

    writeln!(&mut file, "\n{} {}\n Expenses:{}  {}\n Assets:MyTime", formatted_date, wallet, wallet, time_amount);

    Ok(())
}
