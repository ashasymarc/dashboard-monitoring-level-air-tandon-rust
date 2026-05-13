use std::io;

struct Sensor {
    name: String,
    value: f32,
    offset: f32,
}

impl Sensor {
    fn new(name: String, offset: f32) -> Sensor {
        Sensor {
            name,
            value: 0.0,
            offset,
        }
    }

    fn read_value(&mut self, input: f32) {
        self.value = input;
    }

    fn calibrated_value(&self) -> f32 {
        self.value + self.offset
    }
}

struct Controller {
    pump: bool,
    alarm: bool,
}

impl Controller {
    fn new() -> Controller {
        Controller {
            pump: false,
            alarm: false,
        }
    }

    fn control(&mut self, level: f32) {
        if level <= 20.0 {
            self.pump = true;
            self.alarm = false;
        } else if level > 95.0 {
            self.pump = false;
            self.alarm = true;
        } else {
            self.pump = false;
            self.alarm = false;
        }
    }

    fn pump_status(&self) -> &str {
        if self.pump {
            "ON"
        } else {
            "OFF"
        }
    }

    fn alarm_status(&self) -> &str {
        if self.alarm {
            "ON"
        } else {
            "OFF"
        }
    }
}

struct MonitoringSystem {
    sensor: Sensor,
    controller: Controller,
    data: Vec<f32>,
}

impl MonitoringSystem {
    fn new(sensor: Sensor, controller: Controller) -> MonitoringSystem {
        MonitoringSystem {
            sensor,
            controller,
            data: Vec::new(),
        }
    }

    fn add_data(&mut self, level: f32) {
        self.sensor.read_value(level);
        let calibrated = self.sensor.calibrated_value();
        self.data.push(calibrated);
    }

    fn moving_average(&self) -> f32 {
        let sum: f32 = self.data.iter().sum();
        sum / self.data.len() as f32
    }
    fn data_count(&self) -> usize {
    self.data.len()
    }
}

fn get_status(level: f32) -> &'static str {
    if level <= 20.0 {
        "AIR RENDAH"
    } else if level <= 80.0 {
        "NORMAL"
    } else if level <= 95.0 {
        "HAMPIR PENUH"
    } else {
        "OVERFLOW WARNING"
    }
}

fn display_tank(level: f32) -> String {
    let total_bars = 20;
    let filled_bars = ((level / 100.0) * total_bars as f32).round() as usize;
    let empty_bars = total_bars - filled_bars;

    let filled = "#".repeat(filled_bars);
    let empty = "-".repeat(empty_bars);

    format!("[{}{}] {:.2}%", filled, empty, level)
}

fn main() {
    let sensor = Sensor::new(String::from("Ultrasonic Level Sensor"), 1.0);
    let controller = Controller::new();
    let mut system = MonitoringSystem::new(sensor, controller);

    println!("Masukkan jumlah data pembacaan sensor:");

    let mut jumlah_input = String::new();

    io::stdin()
        .read_line(&mut jumlah_input)
        .expect("Gagal membaca input");

    let jumlah_data: usize = jumlah_input
        .trim()
        .parse()
        .expect("Input jumlah data harus berupa angka");

    for i in 1..=jumlah_data {
        println!("\nMasukkan level air tandon ke-{} dalam persen 0-100:", i);

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Gagal membaca input");

        let level: f32 = input
            .trim()
            .parse()
            .expect("Input harus berupa angka");

        if level < 0.0 || level > 100.0 {
            println!("Error: level air harus berada pada rentang 0 sampai 100 persen.");
            continue;
        }

        system.add_data(level);

        let calibrated = system.sensor.calibrated_value();
        let average = system.moving_average();

        system.controller.control(average);

        println!("========================================");
        println!(" DASHBOARD MONITORING LEVEL AIR TANDON ");
        println!("========================================");
        println!("Sensor              : {}", system.sensor.name);
        println!("Level Aktual        : {:.2}%", system.sensor.value);
        println!("Level Terkalibrasi  : {:.2}%", calibrated);
        println!("Moving Average      : {:.2}%", average);
        println!("Status Tandon       : {}", get_status(average));
        println!("Visual Tandon       : {}", display_tank(average));
                println!("Pompa               : {}", system.controller.pump_status());
        println!("Alarm               : {}", system.controller.alarm_status());
    }

    if system.data_count() > 0 {
        let final_average = system.moving_average();
        system.controller.control(final_average);

        println!("\n========================================");
        println!(" RINGKASAN AKHIR MONITORING ");
        println!("========================================");
        println!("Jumlah Data Valid   : {}", system.data_count());
        println!("Rata-rata Level Air : {:.2}%", final_average);
        println!("Status Akhir        : {}", get_status(final_average));
        println!("Visual Tandon       : {}", display_tank(final_average));
        println!("Pompa Akhir         : {}", system.controller.pump_status());
        println!("Alarm Akhir         : {}", system.controller.alarm_status());
        println!("========================================");
    } else {
        println!("Tidak ada data valid yang diproses.");
    }
}