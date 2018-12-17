use super::Data;
use std::io::{Error, ErrorKind};
use std::io::prelude::*;
use std::fs::File;
use std::mem;
use std::fs;

pub struct DataExtractor {
    path_img: String,
    path_val: String
}

impl DataExtractor {
    pub fn new() -> Self {
        Self {
            path_img: String::new(),
            path_val: String::new()
        }
    }

    pub fn set_path_img(&mut self, path: &str) {
        self.path_img = String::from(path);
    }

    pub fn set_path_val(&mut self, path: &str) {
        self.path_val = String::from(path);
    }

    pub fn extract(&self, datas: &mut Vec<Data>) -> Result<(), Error> {
        let mut file_img = File::open(&self.path_img)?;
        let mut file_val = File::open(&self.path_val)?;
        let mut data = [0; 4];
        let nb_pic: u32;
        let nb_val: u32;
        let width: u32;
        let height: u32;

        let slice_to_u32 = |buf: &[u8]|  (buf[0] as u32) << 24 | (buf[1] as u32) << 16 | (buf[2] as u32) << 8 | buf[3] as u32;

        // Read image magic number
        file_img.read(&mut data)?;
        if slice_to_u32(&data) != 0x803 {
            return Err(Error::new(ErrorKind::InvalidData, "Incorrect image magic number."))
        }

        // Read value magic number
        file_val.read(&mut data)?;
        if slice_to_u32(&data) != 0x801 {
            return Err(Error::new(ErrorKind::InvalidData, "Incorrect value magic number."))
        }

        // Read the number of pictures
        file_img.read(&mut data)?;
        nb_pic = slice_to_u32(&data);

        // Read the number of values
        file_val.read(&mut data)?;
        nb_val = slice_to_u32(&data);

        if nb_pic != nb_val {
            return Err(Error::new(ErrorKind::InvalidData, "The number of pictures is differ from the number of values."));
        }

        // Read the width
        file_img.read(&mut data)?;
        width = slice_to_u32(&data);

        // Read the height
        file_img.read(&mut data)?;
        height = slice_to_u32(&data);

        let mut buf: Vec<u8> = Vec::new();
        file_img.read_to_end(&mut buf)?;

        for i in 0..nb_pic {
            let mut pixels: Vec<u8> = Vec::with_capacity((width*height) as usize);
            let mut res = Data::new();
            let mut value = [0;1];

            file_val.read(&mut value)?;

            for j in 0..width*height {
                pixels.push(buf[(j + i * width * height) as usize]);
            }

            res.set_value(value[0]);
            res.set_size(width, height);
            res.set_pixels(pixels);

            datas.push(res);
        }

        Ok(())
    }

    pub fn save_as_bmp(&self, datas: &mut Vec<Data>) -> Result<(), Error> {
        println!("Nombre d'éléments : {}", datas.len());

        for (i, data) in datas.iter().enumerate() {
            let (width, height) = match data.get_size() { Some(s) => s, None => (0, 0) };
            let value = match data.get_value() { Some(s) => s as i8, None => -1 };
            let slice_size_file = unsafe { mem::transmute::<u32, [u8; 4]>(50 + width * height * 3) };
            let slice_width = unsafe { mem::transmute::<u32, [u8; 4]>(width) };
            let slice_height = unsafe { mem::transmute::<u32, [u8; 4]>(height) };


            let mut buffer: Vec<u8> = Vec::with_capacity((50 + width * height * 3) as usize);

            // FILE HEADER
            buffer.extend_from_slice(b"BM");                // Magic number
            buffer.extend_from_slice(&slice_size_file);     // Size
            buffer.extend_from_slice(&[0;4]);//b"TKNN");    // IDs
            buffer.extend_from_slice(&[0x36, 0, 0, 0]);     // offset

            // BMP HEADER
            buffer.extend_from_slice(&[0x28, 0, 0, 0]);     // Header size
            buffer.extend_from_slice(&slice_width);         // Width
            buffer.extend_from_slice(&slice_height);        // Height
            buffer.extend_from_slice(&[1, 0]);              // ??
            buffer.extend_from_slice(&[24, 0]);             // Nb bit used for color
            buffer.extend_from_slice(&[0;4]);               // Compression mode
            buffer.extend_from_slice(&[0;4]);               // Image size
            buffer.extend_from_slice(&[0;4]);               // Vertical resolution
            buffer.extend_from_slice(&[0;4]);               // Horizontal resolution
            buffer.extend_from_slice(&[0;4]);               // Number of color (useless)
            buffer.extend_from_slice(&[0;4]);               // Important color (useless)

            let extra_bytes = match 4 - 3 * width % 4 {
                4 => 0,
                n => n
            };

            let pixels = data.get_pixels();

            for x in (0..height).rev() {
                for y in 0..width {
                    buffer.push(pixels[(x * width + y) as usize]);
                    buffer.push(pixels[(x * width + y) as usize]);
                    buffer.push(pixels[(x * width + y) as usize]);
                }

                for _ in 0..extra_bytes {
                    buffer.push(0);
                }
            }

            fs::create_dir_all("./Data/Image/")?;

            let mut img = File::create(format!("./Data/Image/img_{}_number_{}.bmp", i, value))?;

            println!("Writing file...");

            img.write(&buffer)?;
        }

        Ok(())
    }
}
