extern crate byteorder;
extern crate itertools;

use byteorder::{BigEndian, ReadBytesExt};
use itertools::Itertools;
use std::fs::File;
use std::io::Read;

const LABELS_FILE_MAGIC : u32 = 2049;
const IMAGES_FILE_MAGIC : u32 = 2051;

#[derive(Debug, PartialEq)]
pub struct Img {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<f64>,
}

impl Img {
    pub fn new(width: u32, height: u32, pixels: Vec<f64>) -> Img {
        Img{
            width,
            height,
            pixels
        }
    }

    pub fn to_string(&self) -> String {
        self.pixels.iter().chunks(self.width as usize).into_iter().map(|line|
                line.into_iter().map(|p|
                    if *p < 0.01 {
                        " "
                    }
                    else if *p < 0.5 {
                        "."
                    }
                    else {
                        "#"
                    }
                ).join("")
            ).join("\n")
    }
}

pub fn read_labels(filename: String, limit: Option<u32>) -> Result<Vec<u8>, String> {
    let mut file = File::open(&filename).unwrap();
    let magic = file.read_u32::<BigEndian>().unwrap();
    if magic != LABELS_FILE_MAGIC {
        return Err(format!("Invalid file type for labels: {}, expected {}", magic, LABELS_FILE_MAGIC));
    }
    let mut nb_labels = file.read_u32::<BigEndian>().unwrap();
    if let Some(max_labels) = limit {
        nb_labels = std::cmp::min(max_labels, nb_labels)
    }
    let mut handle = file.take(nb_labels as u64);
    let mut result = vec!();
    let read = handle.read_to_end(&mut result);
    if let Err(_) = read {
        return Err(format!("Error reading labels from {}", filename));
    } 
    if let Ok(nb_read) = read {
        if nb_read != nb_labels as usize {
            return Err(format!("Read {} labels from {}, expected {}", nb_read, filename, nb_labels));
        }
    }
    Ok(result)
}

pub fn read_images(filename: String, limit: Option<u32>) -> Result<Vec<Img>, String> {
    let mut file = File::open(&filename).unwrap();
    let magic = file.read_u32::<BigEndian>().unwrap();
    if magic != IMAGES_FILE_MAGIC {
        return Err(format!("Invalid file type for images: {}, expected {}", magic, IMAGES_FILE_MAGIC));
    }
    let mut nb_images = file.read_u32::<BigEndian>().unwrap();
    if let Some(max_images) = limit {
        nb_images = std::cmp::min(max_images, nb_images)
    }
    let nb_rows = file.read_u32::<BigEndian>().unwrap();
    let nb_columns = file.read_u32::<BigEndian>().unwrap();
    let mut handle = file.take((nb_images*nb_rows*nb_columns) as u64);
    let mut data = vec!();
    let read = handle.read_to_end(&mut data);
    if let Err(_) = read {
        return Err(format!("Error reading images from {}", filename));
    } 
    if let Ok(nb_read) = read {
        if nb_read != (nb_images*nb_rows*nb_columns) as usize {
            return Err(format!("Read {} bytes for images from {}, expected {}", nb_read, filename, nb_images*nb_rows*nb_columns));
        }
    }
    Ok(data.into_iter().chunks((nb_rows*nb_columns) as usize).into_iter().map(|pixels| Img::new(nb_columns, nb_rows, pixels.into_iter().map(|p| p as f64/255.0).collect())).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn to_string() {
        let pixels = vec!(1.0, 0.0, 1.0, 0.1, 0.9, 0.1, 0.3, 0.7, 0.3);
        let img = Img::new(3, 3, pixels);
        assert_eq!("# #\n.#.\n.#.", img.to_string())
    }

    #[test]
    fn read_labels_file_wrong_magic() {
        let tmpdir = tempfile::tempdir().unwrap();
        let tmpfile = format!("{}/{}", tmpdir.path().to_str().unwrap(), "read_labels_file_wrong_magic");
        let data : [u8;4] = [0, 0, 8, 3]; // 2051 = 0x803
        fs::write(&tmpfile, &data).unwrap();
        assert_matches!(read_labels(tmpfile, None), Err(_));
    }

    #[test]
    fn read_labels_file_no_labels() {
        let tmpdir = tempfile::tempdir().unwrap();
        let tmpfile = format!("{}/{}", tmpdir.path().to_str().unwrap(), "read_labels_file_no_labels");
        let data : [u8;8] = [0, 0, 8, 1, // 2049 = 0x801
                             0, 0, 0, 0]; // 0 labels
        fs::write(&tmpfile, &data).unwrap();
        assert_eq!(read_labels(tmpfile, None), Ok(vec!()));
    }

    #[test]
    fn read_labels_4() {
        let tmpdir = tempfile::tempdir().unwrap();
        let tmpfile = format!("{}/{}", tmpdir.path().to_str().unwrap(), "read_labels_4");
        let data : [u8;12] = [0, 0, 8, 1, // 2049 = 0x801
                              0, 0, 0, 4, // 4 labels
                              1, 2, 3, 4];
        fs::write(&tmpfile, &data).unwrap();
        assert_eq!(read_labels(tmpfile, None), Ok(vec![1, 2, 3, 4]));
    }

    #[test]
    fn read_images_file_wrong_magic() {
        let tmpdir = tempfile::tempdir().unwrap();
        let tmpfile = format!("{}/{}", tmpdir.path().to_str().unwrap(), "read_images_file_wrong_magic");
        let data : [u8;4] = [0, 0, 8, 1]; // 2049 = 0x801
        fs::write(&tmpfile, &data).unwrap();
        assert_matches!(read_images(tmpfile, None), Err(_));
    }

    #[test]
    fn read_images_file_no_images() {
        let tmpdir = tempfile::tempdir().unwrap();
        let tmpfile = format!("{}/{}", tmpdir.path().to_str().unwrap(), "read_images_file_no_images");
        let data : [u8;16] = [0, 0, 8, 3, // 2051 = 0x803
                              0, 0, 0, 0, // 0 images
                              0, 0, 1, 0, // height
                              0, 0, 1, 0]; // width
        fs::write(&tmpfile, &data).unwrap();
        assert_eq!(read_images(tmpfile, None), Ok(vec!()));
    }

    #[test]
    fn read_images_1() {
        let tmpdir = tempfile::tempdir().unwrap();
        let tmpfile = format!("{}/{}", tmpdir.path().to_str().unwrap(), "read_images_4");
        // TODO: Can't do more than 32 bytes because a slice greater than 32 gives the following error:
        // the trait `std::convert::AsRef<[u8]>` is not implemented for `[u8; N]`
        // which is a limitation of Rust generics: rust-lang/rust#44580
        // => Find a way around it.
        let data : [u8;32] = [0, 0, 8, 3, // 2051 = 0x803
                              0, 0, 0, 4, // 4 images
                              0, 0, 0, 2, // height
                              0, 0, 0, 2, // width
                              0, 255, 0, 0,
                              255, 0, 0, 0,
                              0, 0, 255, 0,
                              255, 0, 0, 255];
        fs::write(&tmpfile, &data).unwrap();
        let img1 = Img::new(2, 2, vec![0.0, 1.0, 0.0, 0.0]);
        let img2 = Img::new(2, 2, vec![1.0, 0.0, 0.0, 0.0]);
        let img3 = Img::new(2, 2, vec![0.0, 0.0, 1.0, 0.0]);
        let img4 = Img::new(2, 2, vec![1.0, 0.0, 0.0, 1.0]);
        assert_eq!(read_images(tmpfile, None), Ok(vec![img1, img2, img3, img4]));
    }
}
