use rusty_hass_laptop::{
    hass,
    sensors::Provider,
    sensors::{battery::BatteryProvider, cpu::CpuProvider},
    Config,
};

fn main() -> Result<(), battery::Error> {
    let mut client = hass::Client::new(Config::new()).unwrap();
    match client.config.webhook_id {
        None => client.register_app().unwrap(),
        _ => (),
    }
    client.config.save_to_file().unwrap();

    let mut battery = BatteryProvider::new();
    let mut cpu = CpuProvider::new();
    battery.update_all().unwrap();
    cpu.update_all().unwrap();
    client.register_sensor(&battery.level).unwrap();
    client.register_sensor(&cpu.cpu_freq).unwrap();
    client.register_sensor(&cpu.cpu_cores).unwrap();
    loop {
        battery.update_all().unwrap();
        println!("{:?}", battery.level.state);
        client.update_sensor(&battery.level).unwrap();
        client.update_sensor(&cpu.cpu_freq).unwrap();
        client.update_sensor(&cpu.cpu_cores).unwrap();
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}
