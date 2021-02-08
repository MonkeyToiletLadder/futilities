use quick_xml::Reader;
use quick_xml::events::Event;
use std::{ 
    fs::File,
    io, 
    io::BufReader, 
    io::prelude::*, 
    path::Path 
};

pub trait IsImage {
    fn is_png(&self) -> Result<bool, io::Error>;
    fn is_jpg(&self) -> Result<bool, io::Error>;
    fn is_gif(&self) -> Result<bool, io::Error>;
    fn is_bmp(&self) -> Result<bool, io::Error>;
    fn is_tif(&self) -> Result<bool, io::Error>;
    fn is_xcf(&self) -> Result<bool, io::Error>;
    fn is_ppm(&self) -> Result<bool, io::Error>;
    fn is_pgm(&self) -> Result<bool, io::Error>;
    fn is_pbm(&self) -> Result<bool, io::Error>;
    fn is_svg(&self) -> Result<bool, quick_xml::Error>;
}

pub trait IsExecutable {
    fn is_elf(&self) -> Result<bool, io::Error>;
}

macro_rules! impl_isexecutable {
    ($head:ident) => {
        impl IsExecutable for $head {
            fn is_elf(&self) -> Result<bool, io::Error> {
                let mut file = File::open(self)?;
                let mut bytes: [u8; 24] = [0; 24];
                file.read_exact(&mut bytes)?;
                let magic = vec![
                    0x7F, 
                    0x45, 
                    0x4C, 
                    0x46
                ];
                let format: Vec<u8> = (0x01..=0x02).collect();
                let os: Vec<u8> = (0x00..=0x12).collect();
                let filetype: Vec<u16> = vec![
                    0x0000,
                    0x0100,
                    0x0200,
                    0x0300,
                    0x0400,
                    0xFE00,
                    0xFEFF,
                    0xFF00,
                    0xFFFF
                ];
                let architecture: Vec<u16> = vec![
                    0x0000,
                    0x0100,
                    0x0200,
                    0x0300,
                    0x0400,
                    0x0500,
                    0x0600,
                    0x0700,
                    0x0800,
                    0x0900,
                    0x0A00,
                    0x0B00,
                    0x0C00,
                    0x0D00,
                    0x0E00,
                    0x0F00,
                    0x1300,
                    0x1400,
                    0x1500,
                    0x1600,
                    0x2800,
                    0x2A00,
                    0x3200,
                    0x3E00,
                    0x8C00,
                    0xB700,
                    0xF300,
                    0x1010
                ];
                let pad: Vec<u8> = vec![
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0x00
                ];
                if bytes[0..=3] != magic {
                    return Ok(false);
                }
                // Bitset
                if let None = format.iter().position(|b| *b == bytes[4]) {
                    return Ok(false);
                }
                // Endian
                if let None = format.iter().position(|b| *b == bytes[5]) {
                    return Ok(false);
                }
                // Version
                if bytes[6] != 0x01 {
                    return Ok(false);
                }
                // Os ABI
                if let None = os.iter().position(|b| *b == bytes[7]) {
                    return Ok(false);
                }
                // 7 bytes of padding
                if &bytes[9..=15] != pad {
                    return Ok(false)
                }
                // Object file type
                if let None = filetype.iter().position(|b| *b == ((bytes[16] as u16) << 8) | (bytes[17] as u16)) {
                    return Ok(false);
                }
                // Architecture
                if let None = architecture.iter().position(|b| *b == ((bytes[18] as u16) << 8) | (bytes[19] as u16)) {
                    return Ok(false);
                }
                // Version
                if bytes[20] != 0x01 {
                    return Ok(false);
                }
                // 3 bytes of padding
                if &bytes[21..=23] != [0x00, 0x00, 0x00] {
                    return Ok(false);
                }
                Ok(true)
            }
        }
    }
}

macro_rules! impl_isimage {
    ($head:ident) => {
        impl IsImage for $head {
            fn is_png(&self) -> Result<bool, io::Error> {
                let mut file = File::open(self)?;
                let mut bytes: [u8; 8] = [0; 8];
                file.read_exact(&mut bytes)?;
                let header = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
                Ok(header == bytes) 
            }

            fn is_jpg(&self) -> Result<bool, io::Error> {
                let mut file = File::open(self)?;
                let mut bytes: [u8; 11] = [0; 11];
                file.read_exact(&mut bytes)?;
                let candidate = bytes.iter().enumerate().filter(|(i, _)| *i != 4 && *i != 5).map(|(_, b)| *b).collect::<Vec<u8>>();
                let headerv1 = vec![0xFF, 0xD8, 0xFF, 0xE0, 0x4A, 0x46, 0x49, 0x46, 0x00];
                let headerv2 = vec![0xFF, 0xD8, 0xFF, 0xFF, 0x4A, 0x46, 0x49, 0x46, 0x00];
                Ok(headerv1 == candidate || headerv2 == candidate) 
            }
            fn is_gif(&self) -> Result<bool, io::Error> {
                let mut file = File::open(self)?;
                let mut bytes: [u8; 6] = [0; 6];
                file.read_exact(&mut bytes)?;
                let headerv89a = vec![0x47, 0x49, 0x46, 0x38, 0x39, 0x61];
                let headerv87a = vec![0x47, 0x49, 0x46, 0x38, 0x37, 0x61];
                Ok(headerv89a == bytes ||  headerv87a == bytes)
            }
            fn is_bmp(&self) -> Result<bool, io::Error> {
                let mut file = File::open(self)?;
                let mut bytes: [u8; 2] = [0; 2];
                file.read_exact(&mut bytes)?;
                let header = vec![0x42, 0x4D];
                Ok(header == bytes)
            }
            fn is_tif(&self) -> Result<bool, io::Error> {
                let mut file = File::open(self)?;
                let mut bytes: [u8; 3] = [0; 3];
                file.read_exact(&mut bytes)?;
                let headerle = vec![0x49, 0x49, 0x2A];
                let headerbe = vec![0x4D, 0x4D, 0x2A];
                Ok(headerle == bytes || headerbe == bytes)
            }
            fn is_xcf(&self) -> Result<bool, io::Error> {
                let mut file = File::open(self)?;
                let mut bytes: [u8; 9] = [0; 9];
                file.read_exact(&mut bytes)?;
                let header = vec![0x67, 0x69, 0x6D, 0x70, 0x20, 0x78, 0x63, 0x66, 0x20];
                Ok(header == bytes)
            }
            fn is_ppm(&self) -> Result<bool, io::Error> {
                let mut file = File::open(self)?;
                let mut bytes: [u8; 2] = [0; 2];
                file.read_exact(&mut bytes)?;
                let headerp3 = vec![0x50, 0x33];
                let headerp6 = vec![0x50, 0x36];
                Ok(headerp3 == bytes || headerp6 == bytes)
            }
            fn is_pgm(&self) -> Result<bool, io::Error> {
                let mut file = File::open(self)?;
                let mut bytes: [u8; 2] = [0; 2];
                file.read_exact(&mut bytes)?;
                let headerp2 = vec![0x50, 0x32];
                let headerp5 = vec![0x50, 0x35];
                Ok(headerp2 == bytes || headerp5 == bytes)
            }
            fn is_pbm(&self) -> Result<bool, io::Error> {
                let mut file = File::open(self)?;
                let mut bytes: [u8; 2] = [0; 2];
                file.read_exact(&mut bytes)?;
                let headerp1 = vec![0x50, 0x31];
                Ok(headerp1 == bytes)
            }
            fn is_svg(&self) -> Result<bool, quick_xml::Error> {
                let mut reader: Reader<BufReader<File>>;
                reader = Reader::from_file(self)?;
                let mut buffer = Vec::new();
                loop {
                    match reader.read_event(&mut buffer) {
                        Ok(Event::Start(ref e)) if e.name() == b"svg" => {
                            return Ok(true);
                        }
                        Ok(Event::Eof) => break,
                        Err(error) => return Err(error),
                        _ => ()
                    }
                }
                Ok(false)
            }
        }
    }
}

macro_rules! for_each_ident {
    ($callback:ident, $head:ident) => { 
        $callback!($head);
    };
    ($callback:ident, $head:ident, $($tail:ident),*) => {
        $callback!($head);
        for_each_ident!($callback, $($tail),*);
    }
}

for_each_ident!(impl_isimage, Path, String, str);
for_each_ident!(impl_isexecutable, Path, String, str);