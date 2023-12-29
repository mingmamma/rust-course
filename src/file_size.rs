enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}

struct File {
    size: FileSize,
}

impl File {
    fn new(bytes: u64) -> Self {
        let file_size: FileSize = match bytes {
            0..=999 => FileSize::Bytes(bytes),
            1000..=999_999 => FileSize::Kilobytes(bytes as f64 / 1_000.0),
            1_000_000..=999_999_999 => FileSize::Megabytes(bytes as f64 / 1_000_000.0),
            _ => FileSize::Gigabytes(bytes as f64 / 1_000_000_000.0),            
        };
        
        Self {
            size: file_size
        }
    } 

    fn format_size(&self) -> String {
        match &self.size {
            FileSize::Bytes(bytes) => format!("{} bytes", bytes),
            FileSize::Kilobytes(kilobytes) => format!("{:.2} KB", kilobytes),
            FileSize::Megabytes(megabytes) => format!("{:.2} MB", megabytes),
            FileSize::Gigabytes(gigabytes) => format!("{:.2} GB", gigabytes),
        }
    }
}


fn main() {
    // let result = format_size(6888837399);
    // println!("{}", result)

    let file_1 = File::new(1);
    println!("{}",file_1.format_size());

    let file_1 = File::new(1_000);
    println!("{}",file_1.format_size());

    let file_1 = File::new(6_888);
    println!("{}",file_1.format_size());

    let file_1 = File::new(1_000_000);
    println!("{}",file_1.format_size());

    let file_1 = File::new(1_000_000_000);
    println!("{}",file_1.format_size());

    let file_1 = File::new(6_888_837_399);
    println!("{}",file_1.format_size());

}