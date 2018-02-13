extern crate regex;
extern crate chrono;

use std::process::Command;
use regex::Regex;

use chrono::prelude::*;

fn battery() -> String {
    let output = Command::new("acpi").arg("-b").output().expect(
        "Could not run acpi",
    );
    let re = Regex::new(r".* \d: (.*), (\d+).*").expect(
        "Failed to regex on the output of acpi, is it installed?",
    );
    let text = String::from_utf8_lossy(&output.stdout);
    let groups = re.captures(&text).unwrap();

    let battery_status = groups.get(1).unwrap().as_str();
    let battery_percent: i32 = groups.get(2).unwrap().as_str().parse().unwrap();

    let battery_icons = ["", "", "", "", ""];

    if battery_status == "Charging" {
        format!("{}%", battery_percent)
    } else {
        format!(
            "{}  {}",
            match battery_percent {
                0...20 => battery_icons[3],
                20...40 => battery_icons[2],
                40...60 => battery_icons[1],
                60...100 => battery_icons[0],
                _ => "",
            },
            battery_percent
        )
    }
}

fn wifi() -> String {
    let output = Command::new("cat")
        .arg("/proc/net/wireless")
        .output()
        .expect("Failed to cat proc/net/wireless");
    let text = String::from_utf8_lossy(&output.stdout);

    let wifi_regex = Regex::new(r"wlp3s0: \d+\s+(\d*).").unwrap();

    let groups = wifi_regex.captures(&text).unwrap();

    let ssid = Command::new("iwgetid").arg("-r").output().expect(
        "Failed to run iwgetid",
    );
    let ssid = String::from_utf8_lossy(&ssid.stdout);
    let mut ssid = String::from(ssid);
    ssid.pop();


    format!("{} {}%", ssid, groups.get(1).unwrap().as_str())
}

fn volume() -> String {
    let output = Command::new("pactl")
        .arg("list")
        .arg("sinks")
        .output()
        .expect("Failed to run pactl, is it installed?");

    let mute_reg = Regex::new(r"Mute: (.*)").expect("Failed to regex on the output of pactl");
    let vol_reg = Regex::new(r"(\d*)%").expect("Failed to regex on the output of pactl");
    let text = String::from_utf8_lossy(&output.stdout);
    let groups1 = mute_reg.captures(&text).unwrap();
    let groups2 = vol_reg.captures(&text).unwrap();

    let mute = groups1.get(1).unwrap().as_str();
    let vol: i32 = groups2.get(1).unwrap().as_str().parse().unwrap();

    let icons = ["", "", ""];

    if mute == "yes" {
        format!("{} Muted", icons[0])
    } else {
        format!(
            "{} {}",
            match vol {
                0...50 => icons[1],
                _ => icons[2],
            },
            vol
        )
    }
}

fn date() -> String {
    let now: DateTime<Local> = Local::now();
    format!(
        " {:?} {}/{} {}:{minute:>0width$}",
        now.weekday(),
        now.day(),
        now.month(),
        now.hour(),
        minute = now.minute(),
        width = 2
    )
}



fn main() {
    print!("{} {} {} {}", wifi(), volume(), battery(), date());
}
