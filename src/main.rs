use std::{thread, time::Duration};

use clap::Parser;

use notify_rust::{Notification, Timeout};

//Cli struct
#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(short, long)]
    pub first_threshold: Option<i64>,
    #[arg(short, long)]
    pub second_threshold: Option<i64>,
    #[arg(short, long)]
    pub interval: Option<u64>,
}

fn main() {
    let cli = Cli::parse();

    //Get set value or default of 15%
    let first_threshold_warning = cli.first_threshold.unwrap_or(15);
    //Get the second threshold or default to 5%
    let second_threshold_warning = cli.second_threshold.unwrap_or(5);
    //Get the interval or default to 5 minutes
    let interval = cli.interval.unwrap_or(5);

    let manager = battery::Manager::new().unwrap();

    let mut battery = match manager.batteries().unwrap().next() {
        Some(Ok(battery)) => battery,
        Some(Err(err)) => panic!("Could not access battery info {err}"),
        None => panic!("Unable to find any batteries"),
    };

    let (mut first_shown, mut second_shown) = (false, false);
    loop {
        if let battery::State::Discharging = battery.state() {}

        match battery.state() {
            battery::State::Charging => {
                //When chargin we reset notifications shown
                first_shown = false;
                second_shown = false;
            }
            battery::State::Discharging => {
                // Get the % of the battery rounded
                let current_percentage: f64 =
                    (match format!("{:?}", battery.state_of_charge()).parse::<f64>() {
                        Ok(result) => result,
                        Err(err) => {
                            println!("{err}");
                            0.0
                        }
                    } * 100f64)
                        .round();

                //If value is bellow threshold
                let percent_to_show = current_percentage.round() as i64;
                if !first_shown
                    && current_percentage <= first_threshold_warning as f64
                    && current_percentage > second_threshold_warning as f64
                {
                    Notification::new()
                        .summary("Battery")
                        .icon("firefox")
                        .body(&format!("Running low: {percent_to_show}%"))
                        .timeout(Timeout::Milliseconds(5000))
                        .show()
                        .unwrap();

                    //Set shown to true to avoid spamming
                    first_shown = true;
                } else if !second_shown && current_percentage <= second_threshold_warning as f64 {
                    Notification::new()
                        .summary("Battery")
                        .body(&format!(
                            "Extremely low: {percent_to_show}%. Please plug me in!"
                        ))
                        .timeout(Timeout::Milliseconds(10000))
                        .show()
                        .unwrap();
                    //Set shown to true to avoid spamming
                    second_shown = true;
                }
            }
            _ => {}
        }

        //Sleep until next cycle
        thread::sleep(Duration::from_secs(interval * 60));
        manager.refresh(&mut battery).unwrap();
    }
}
