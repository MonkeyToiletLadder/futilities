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