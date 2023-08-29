use rbroadlink::{traits::DeviceTrait, Device};
use std::env;
use std::net::Ipv4Addr;

#[derive(PartialEq)]
enum RunMode {
    Help,
    Info,
    Toggle,
    Waybar,
}

fn main() {
    let argument = env::args().nth(1);
    let run_mode = if let Some(arg) = argument {
        match &arg[..] {
            "info" => RunMode::Info,
            "toggle" => RunMode::Toggle,
            "waybar" => RunMode::Waybar,
            _ => RunMode::Help,
        }
    } else {
        RunMode::Help
    };

    if run_mode == RunMode::Help {
        println! {"No arguments given!"};
        return;
    };

    println!(">>> device authentication ...");
    let known_ip = Ipv4Addr::new(192, 168, 0, 100);
    let device = Device::from_ip(known_ip, None).unwrap();
    let addr = device.get_info().address;
    println!(">>> device at {} => {}", addr, device);

    let hvac = match device {
        Device::Hvac { hvac } => hvac,
        _ => {
            return;
        }
    };
    if run_mode == RunMode::Info {
        println!(">>> get_info");
        let ac_info = hvac.get_info().unwrap();
        println!("Power {}", ac_info.power);
        println!("Ambient Temp = {:.1}", ac_info.get_ambient_temp());
    } else if run_mode == RunMode::Toggle {
        println!(">>> get_state");
        let mut state = hvac.get_state().unwrap();
        println!("Current state: {:?}", state);

        println!(">>> set_state");
        state.power = !state.power;
        //println!("Target temp = {:.1}", state.get_target_temp());
        //if let Err(e) = state.set_target_temp(15.0) {
        //    println!("Error setting temperature: {}", e);
        //}
        let response = hvac.set_state(&mut state).unwrap();
        println!("Final response {:02x?}", response);
    }
}
