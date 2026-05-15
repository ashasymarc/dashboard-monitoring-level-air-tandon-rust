use std::io::{self, Write};

struct Sensor {
    nama: String,
    nilai_raw: f32,
    nilai_kal: f32,
    error: bool,
    buffer: Vec<f32>,
}

impl Sensor {
    fn baru(nama: &str) -> Self {
        Sensor { nama: nama.to_string(), nilai_raw: 0.0, nilai_kal: 0.0, error: false, buffer: Vec::new() }
    }
    fn set_nilai(&mut self, v: f32) {
        if v < 0.0 || v > 100.0 { self.error = true; return; }
        self.error = false; self.nilai_raw = v;
        self.nilai_kal = ((v * 1.02) + (-1.0)).clamp(0.0, 100.0);
    }
    fn moving_average(&mut self) -> f32 {
        self.buffer.push(self.nilai_kal);
        if self.buffer.len() > 3 { self.buffer.remove(0); }
        let buf: Vec<String> = self.buffer.iter().map(|x| format!("{:.2}", x)).collect();
        println!("  Buffer MA : {:?}", buf);
        self.buffer.iter().sum::<f32>() / self.buffer.len() as f32
    }
    fn status(ma: f32) -> &'static str {
        match ma as u32 {
            0..=9   => "KRITIS   - Tandon hampir kosong!",
            10..=29 => "RENDAH   - Segera isi tandon",
            30..=79 => "NORMAL   - Level aman",
            80..=89 => "TINGGI   - Mendekati penuh",
            _       => "OVERFLOW - Tandon penuh!",
        }
    }
}

struct Controller {
    batas_bawah: f32,
    batas_atas: f32,
    pompa: bool,
}

impl Controller {
    fn baru() -> Self { Controller { batas_bawah: 30.0, batas_atas: 80.0, pompa: false } }
    fn update(&mut self, ma: f32) {
        if ma < self.batas_bawah { self.pompa = true; }
        else if ma >= self.batas_atas { self.pompa = false; }
    }
}

struct MonitoringSystem {
    siklus: u32,
    histori: Vec<String>,
}

impl MonitoringSystem {
    fn baru() -> Self { MonitoringSystem { siklus: 0, histori: Vec::new() } }
    fn catat(&mut self, s: &Sensor, ma: f32, pompa: bool) {
        self.siklus += 1;
        self.histori.push(format!(
            "Siklus {:02} | Raw:{:.2}% | Kal:{:.2}% | MA:{:.2}% | {} | Pompa:{}",
            self.siklus, s.nilai_raw, s.nilai_kal, ma,
            Sensor::status(ma), if pompa { "NYALA" } else { "MATI" }
        ));
    }
    fn tampilkan(&self, s: &Sensor, c: &Controller, ma: f32) {
        println!("\n╔══════════════════════════════════════════╗");
        println!("║   MONITORING LEVEL AIR TANDON GEDUNG    ║");
        println!("╠══════════════════════════════════════════╣");
        println!("  Siklus    : {}      |  Sensor : {}", self.siklus, s.nama);
        println!("  Raw       : {:.2}%  |  Kalibrasi    : {:.2}%", s.nilai_raw, s.nilai_kal);
        println!("  MA        : {:.2}%  |  Kondisi      : {}", ma, Sensor::status(ma));
        println!("  Pompa     : {}  (ON<{:.0}% | OFF>={:.0}%)",
            if c.pompa { "NYALA" } else { "MATI  " }, c.batas_bawah, c.batas_atas);
        println!("╚══════════════════════════════════════════╝");
    }
    fn histori(&self) {
        println!("\n===== HISTORI =====");
        for h in &self.histori { println!("  {}", h); }
        println!("===================");
    }
}

fn main() {
    println!("╔══════════════════════════════════════════╗");
    println!("║   SISTEM MONITORING LEVEL AIR TANDON     ║");
    println!("╚══════════════════════════════════════════╝");

    let mut sensor  = Sensor::baru("Sensor Ultrasonik");
    let mut control = Controller::baru();
    let mut monitor = MonitoringSystem::baru();

    loop {
        print!("\nMasukkan level (0-100), 'h' histori, 'q' keluar: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        match input {
            "q" | "Q" => { monitor.histori(); println!("Program dihentikan."); break; }
            "h" | "H" => monitor.histori(),
            _ => match input.parse::<f32>() {
                Ok(v) => {
                    sensor.set_nilai(v);
                    if sensor.error { println!("ERROR: Nilai harus antara 0 sampai 100!"); }
                    else {
                        let ma = sensor.moving_average();
                        control.update(ma);
                        monitor.catat(&sensor, ma, control.pompa);
                        monitor.tampilkan(&sensor, &control, ma);
                    }
                }
                Err(_) => println!("Input tidak valid.")
            }
        }
    }
}