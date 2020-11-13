extern crate nom;

use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use sdl2::gfx::primitives::ToColor;

// cf. https://en.wikipedia.org/wiki/BMP_file_format
#[derive(Debug, Default, PartialEq)]
pub struct BmpFile {
    pub magic: Magic,
    pub filesize: u32,
    pub offset: u32,
    pub dib_header_size: DibHeaderSize,
    pub width: i32,
    pub height: i32,
    pub planes: u16,
    pub bpp: u16,
    pub compression: Option<CompressionMethod>,
    pub x_px_per_m: u32,
    pub y_px_per_m: u32,
    pub image_size: u64,
    pub bitmasks: Option<Bitmasks>,
    pub endpoints: Option<Endpoints>,
    pub gammas: Option<Gammas>,
    pub colors_in_table: u32,
    pub color_space_type: ColorSpaceType,
    pub intent: Option<IntentType>,

    pub palette: Vec<Color>,
    pub data: Vec<u8>,
    pub pixels: Vec<Vec<Color>>,
}

#[derive(Debug, PartialEq)]
pub struct Bitmasks {
    pub red_mask: u32,
    pub red_shift: u32,
    pub green_mask: u32,
    pub green_shift: u32,
    pub blue_mask: u32,
    pub blue_shift: u32,
    pub alpha_mask: u32,
    pub alpha_shift: u32,
}

#[derive(Debug, PartialEq)]
pub struct Endpoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, PartialEq)]
pub struct Endpoints {
    pub red: Endpoint,
    pub green: Endpoint,
    pub blue: Endpoint,
}

#[derive(Debug, PartialEq)]
pub struct Gammas {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u16)]
pub enum Magic {
    BM = 0x4d42,
    // TODO: BA has an extra header before the regular BM one:
    // https://www.fileformat.info/format/os2bmp/egff.htm#X058-9-OS2BMP-FG-2
    // I guess we should either skip the header and read the first BM, or read all the BMs and have
    // a vec of pictures?
    BA = 0x4142,
    CI = 0x4943,
    CP = 0x5043,
    IC = 0x4349,
    PT = 0x5450,
}

impl Default for Magic {
    fn default() -> Self { Magic::BM }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u32)]
pub enum DibHeaderSize {
    BITMAPCOREHEADER = 12,
    // TODO: Understand how we're supposed to differentiate between this one and the previous one
    // ...
    //OS21XBITMAPHEADER = 12,
    OS22XBITMAPHEADER16 = 16,
    OS22XBITMAPHEADER64 = 64,
    BITMAPINFOHEADER = 40,
    BITMAPV2INFOHEADER = 52,
    BITMAPV3INFOHEADER = 56,
    BITMAPV4HEADER = 108,
    BITMAPV5HEADER = 124,
}

impl Default for DibHeaderSize {
    fn default() -> Self { DibHeaderSize::BITMAPCOREHEADER }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CompressionMethod {
    BI_RGB = 0,
    BI_RLE8 = 1,
    BI_RLE4 = 2,
    // BI_BITFIELDS apparently collides with BI_HUFFMAN1D on OS2: http://zig.tgschultz.com/bmp_file_format.txt
    BI_BITFIELDS = 3,
    // BI_JPEG collides with BI_RLE24 on OS2: http://zig.tgschultz.com/bmp_file_format.txt
    BI_JPEG = 4,
    BI_RLE24 = 42, // Dummy value, the real one being 4 but colliding with JPEG ...
    BI_PNG = 5,
    BI_ALPHABITFIELDS = 6,
    // TODO: Couldn't find any example using those so far ...
    BI_CMYK = 11,
    BI_CMYKRLE8 = 12,
    BI_CMYKRLE4 = 13,
}

// cf. https://www.fileformat.info/format/bmp/egff.htm#MICBMP-DMYID.3.5
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ColorSpaceType {
    LCS_CALIBRATED_RGB = 0,
    LCS_DEVICED_DEPENDENT_RGB = 1,
    LCS_DEVICED_DEPENDENT_CMYK = 2,
    // Not sure what these are, but it happens ...
    // Corresponds to the string 'LINK' and 'MBED' respectively.
    // cf. https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-wmf/3c289fe1-c42e-42f6-b125-4b5fc49a2b20
    LCS_PROFILE_LINKED = 0x4c494e4b,
    LCS_PROFILE_EMBEDDED = 0x4d424544,
    // cf. https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-wmf/eb4bbd50-b3ce-4917-895c-be31f214797f
    LCS_WINDOWS_COLOR_SPACE = 0x57696E20,
    // Corresponds to the string 'BGRs'
    LCS_sRGB = 0x73524742,
}

impl Default for ColorSpaceType {
    fn default() -> Self { ColorSpaceType::LCS_DEVICED_DEPENDENT_RGB }
}

// cf. http://zig.tgschultz.com/bmp_file_format.txt
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum IntentType {
    UNKNOWN = 0,
    LCS_GM_BUSINESS = 1,
    LCS_GM_GRAPHICS = 2,
    LCS_GM_IMAGES = 4,
    LCS_GM_ABS_COLORIMETRIC = 8,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Colorf {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
}

impl ToColor for Color {
    #[inline]
    fn as_rgba(&self) -> (u8, u8, u8, u8) {
        (self.red, self.green, self.blue, self.alpha)
    }

    #[inline]
    fn as_u32(&self) -> u32 {
        (self.red as u32) | ((self.green as u32) << 8) | ((self.blue as u32) << 16) | ((self.alpha as u32) << 24)
    }
}

pub type Input<'a> = &'a [u8];
pub type Error<'a> = nom::error::VerboseError<Input<'a>>;
pub type Result<'a, O> = nom::IResult<Input<'a>, O, Error<'a>>;

impl Color {
    fn parse(input: Input, on4bytes: bool) -> Result<Self> {
        use nom::{
            error::context,
                number::complete::le_u8,
                sequence::tuple,
        };
        let (mut input, (blue, green, red)) = tuple((
                    context("Blue", le_u8),
                    context("Green", le_u8),
                    context("Red", le_u8),
                    ))(input)?;
        if on4bytes {
            let (i, _) = context("Reserved", le_u8)(input)?;
            input = i;
        }
        Ok((input, Self{red, green, blue, alpha: 255}))
    }
}

// TODO: Review which functions should be pub
pub fn shift_from_mask(mask: u32) -> u32 {
    let mut shift = 0;
    let mut mask = mask;
    while mask != 0 && (mask & 1) == 0 {
        mask = mask >> 1;
        shift += 1
    }
    shift
}

pub fn normalize_from_mask(input: u32, mask: u32, shift: u32) -> u8 {
    if mask == 0 {
        return 255;
    }
    let value = (input & mask) >> shift;
    let max_value = mask >> shift;
    let norm = (value as f64) / (max_value as f64);
    //println!("Input value: {} - Normalized: {}", value, (255.0*norm) as u8);
    (255.0*norm) as u8
}

pub fn normalize_u8(value: u8) -> f64 {
    value as f64 / 255.0
}

pub fn normalize(color: Color) -> Colorf {
    Colorf{
        red: normalize_u8(color.red),
        green: normalize_u8(color.green),
        blue: normalize_u8(color.blue),
        alpha: normalize_u8(color.alpha),
    }
}

pub fn denormalize_u8(value: f64) -> u8 {
    (value*255.0) as u8
}

pub fn denormalize(colorf: Colorf) -> Color {
    Color{
        red: denormalize_u8(colorf.red),
        green: denormalize_u8(colorf.green),
        blue: denormalize_u8(colorf.blue),
        alpha: denormalize_u8(colorf.alpha),
    }
}

pub fn apply_gamma(value: f64, gamma: f64) -> f64 {
    value.powf(gamma)
}

pub fn apply_gammas(color: Colorf, gammas: &Gammas) -> Colorf {
    let red = apply_gamma(color.red, gammas.red);
    let green = apply_gamma(color.green, gammas.green);
    let blue = apply_gamma(color.blue, gammas.blue);
    Colorf {
        red, green, blue, alpha: color.alpha
    }
}

pub fn unapply_gammas(color: Colorf, gammas: &Gammas) -> Colorf {
    let red = apply_gamma(color.red, 1.0/gammas.red);
    let green = apply_gamma(color.green, 1.0/gammas.green);
    let blue = apply_gamma(color.blue, 1.0/gammas.blue);
    Colorf {
        red, green, blue, alpha: color.alpha
    }
}

pub fn to_xyz(color: Colorf, endpoints: &Endpoints) -> Colorf {
    let x = endpoints.red.x*color.red + endpoints.green.x*color.green + endpoints.blue.x*color.blue;
    let y = endpoints.red.y*color.red + endpoints.green.y*color.green + endpoints.blue.y*color.blue;
    let z = endpoints.red.z*color.red + endpoints.green.z*color.green + endpoints.blue.z*color.blue;
    Colorf { red: x, green: y, blue: z, alpha: color.alpha }
}

pub fn srgbp_to_srgb(value: f64) -> f64 {
    if value <= 0.0031308 {
        value * 12.92
    } else {
        1.055*apply_gamma(value, 1.0/2.4) - 0.055
    }
}

// From
// https://www.image-engineering.de/library/technotes/958-how-to-convert-between-srgb-and-ciexyz
pub fn xyz_to_srgb(color: Colorf) -> Colorf {
    let (x, y, z) = (color.red, color.green, color.blue);
    let red = 3.24096994*x - 1.53738318*y - 0.49861076*z;
    let green = -0.96924364*x + 1.8759675*y + 0.04155506*z;
    let blue = 0.05563008*x - 0.20397696*y + 1.05697151*z;
    let red = srgbp_to_srgb(red);
    let green = srgbp_to_srgb(green);
    let blue = srgbp_to_srgb(blue);
    Colorf { red, green, blue, alpha: color.alpha }
}

#[allow(non_snake_case)]
pub fn xyY_to_xyz(color: Colorf) -> Colorf {
    if color.green == 0.0 {
        Colorf {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: color.alpha,
        }
    } else {
        Colorf {
         red: color.red*color.blue/color.green,
         green: color.blue,
         blue: (1.0-color.red-color.green)*color.blue/color.green,
         alpha: color.alpha,
        }
    }
}

pub fn fixpt16_16(value: u32) -> f64 {
    //println!("Gamma: {:#08x} ({}) -> {}", value, value, (value as f64) / 65536.0);
    (value as f64) / 2_f64.powf(16.0)
}

pub fn fixpt2_30(value: u32) -> f64 {
    (value as f64) / 2_f64.powf(30.0)
}

impl <'a>BmpFile {
    pub fn parse_file_header(&mut self, input: Input<'a>) -> Result<'a, ()> {
        //println!("parse_file_header: {:?}", &input[0..64]);
        use nom::{
            bytes::complete::take,
            combinator::map_res,
            error::context,
            number::complete::{le_u16, le_u32},
            sequence::tuple,
        };
        // Bitmap file header (https://en.wikipedia.org/wiki/BMP_file_format#Bitmap_file_header)
        let (i, (magic, filesize, _, offset)) = tuple((
            context("Magic", map_res(le_u16, |x| Magic::try_from(x))),
            context("FileSize", le_u32),
            context("Reserved", take(4_u8)),
            context("Offset", le_u32),
        ))(input)?;
        self.magic = magic;
        self.filesize = filesize;
        self.offset = offset;
        Ok((i, ()))
    }

    pub fn parse_dib_header_size(&mut self, input: Input<'a>) -> Result<'a, ()> {
        //println!("parse_dib_header_size: {:?}", &input[0..64]);
        use nom::{
            combinator::map_res,
            error::context,
            number::complete::le_u32,
        };
        let (i, dib_header_size) = context("Bitmap DIB Header Size", map_res(le_u32, |x| DibHeaderSize::try_from(x)))(input)?;
        self.dib_header_size = dib_header_size;
        Ok((i, ()))
    }

    pub fn parse_i16_width_and_height(&mut self, input: Input<'a>) -> Result<'a, ()> {
        //println!("parse_i16_width_and_height: {:?}", &input[0..64]);
        use nom::{
            combinator::{map, verify},
            error::context,
            number::complete::le_i16,
            sequence::tuple,
        };
        let (i, (width, height)) = tuple((
                context("Image Width (i16)", verify(map(le_i16, |x| x as i32), |x| *x >= 0)),
                context("Image Height (i16)", map(le_i16, |x| x as i32)),
        ))(input)?;
        self.width = width;
        self.height = height;
        Ok((i, ()))
    }

    pub fn parse_i32_width_and_height(&mut self, input: Input<'a>) -> Result<'a, ()> {
        //println!("parse_i32_width_and_height: {:?}", &input[0..64]);
        use nom::{
            combinator::verify,
            error::context,
            number::complete::le_i32,
            sequence::tuple,
        };
        let (i, (width, height)) = tuple((
                // TODO: make error more readable
                context("Image Width (i32)", verify(le_i32, |x| *x >= 0)),
                context("Image Height (i32)", le_i32),
        ))(input)?;
        self.width = width;
        self.height = height;
        Ok((i, ()))
    }

    pub fn parse_end_of_core_header(&mut self, input: Input<'a>) -> Result<'a, ()> {
        //println!("parse_end_of_core_header: {:?}", &input[0..64]);
        use nom::{
            error::context,
            number::complete::le_u16,
            sequence::tuple,
        };
        let (i, (planes, bpp)) = tuple((
                context("Planes", le_u16),
                context("Bpp", le_u16),
        ))(input)?;
        self.planes = planes;
        self.bpp = bpp;
        // Will be overriden later if present in the header
        self.image_size = (self.width as i64*self.height as i64*self.bpp as i64) as u64/8;
        // https://www.fileformat.info/format/bmp/egff.htm#MICBMP-DMYID.3.6
        // One-, 4-, and 8-bit BMP files are expected to always contain a color palette. Sixteen-,
        // 24-, and 32-bit BMP files never contain color palettes.
        if self.bpp <= 8 {
            // Will be overriden later if present in the header
            self.colors_in_table = 2_u32.pow(self.bpp as u32)
        }
        Ok((i, ()))
    }

    pub fn parse_info_header(&mut self, input: Input<'a>) -> Result<'a, ()> {
        //println!("parse_info_header: {:?}", &input[0..64]);
        use nom::{
            combinator::map_res,
            combinator::map,
            error::context,
            number::complete::le_u32,
            sequence::tuple,
        };
        let (i, (
                    compression, image_size, x_px_per_m, y_px_per_m,
                    mut colors_in_table, _important_color,
        )) = tuple((
            context("Compression", map_res(le_u32, |x| CompressionMethod::try_from(x))),
            context("Image Size", map(le_u32, |x| x as u64)),
            // TODO: Support non-square pixels?
            // g/pal8nonsquare.bmp in
            // http://entropymine.com/jason/bmpsuite/bmpsuite/html/bmpsuite.html
            // suggests a viewer should consider stretching. This is fine when one value is half
            // the other, but what if values are close but different? This could mean stretching _a
            // lot_. This is actually the case in the exemple where x_px_per_m = 2835 and
            // y_px_per_m = 1417 so doubling doesn't exactly make them match.
            context("X pixels per meter", le_u32),
            context("Y pixels per meter", le_u32),
            context("Colors in color table", le_u32),
            context("Important color count", le_u32),
        ))(input)?;
        // https://www.fileformat.info/format/bmp/egff.htm#MICBMP-DMYID.3.6
        // One-, 4-, and 8-bit BMP files are expected to always contain a color palette. Sixteen-,
        // 24-, and 32-bit BMP files never contain color palettes. We'll still make this check go
        // up to 24 though (it fails with 32 because 2^32 doesn't fit in u32 ...).
        if self.bpp <= 24 && colors_in_table > 2_u32.pow(self.bpp as u32) {
            colors_in_table = 0
            // There should be a simpler way to return an error, allowing to provide more detail
            // and point at the good place in the input. On the other end, we prefer to tolerate
            // this by ignoring the palette in this case.
            //return Err(nom::Err::Failure(make_error::<Input, VerboseError<Input>>(input, ErrorKind::TooLarge)));
        }
        self.compression = Some(compression);
        // BI_JPEG collides with BI_RLE24 on OS2: http://zig.tgschultz.com/bmp_file_format.txt
        if compression == CompressionMethod::BI_JPEG && self.dib_header_size == DibHeaderSize::OS22XBITMAPHEADER64 {
            self.compression = Some(CompressionMethod::BI_RLE24);
        }
        if image_size != 0 {
            self.image_size = image_size;
        }
        self.x_px_per_m = x_px_per_m;
        self.y_px_per_m = y_px_per_m;
        if colors_in_table > 0 {
            self.colors_in_table = colors_in_table;
        }
        Ok((i, ()))
    }

    fn parse_rgb_masks(&mut self, input: Input<'a>) -> Result<'a, ()> {
        //println!("parse_rgb_masks: {:?}", &input[0..64]);
        use nom::{
            error::context,
            number::complete::le_u32,
            sequence::tuple,
        };
        let (i, (
                    red_mask, green_mask, blue_mask,
        )) = tuple((
            context("Red channel bitmask", le_u32),
            context("Green channel bitmask", le_u32),
            context("Blue channel bitmask", le_u32),
        ))(input)?;
        if red_mask != 0 || green_mask != 0 || blue_mask != 0 {
            let red_shift = shift_from_mask(red_mask);
            let green_shift = shift_from_mask(green_mask);
            let blue_shift = shift_from_mask(blue_mask);
            self.bitmasks = Some(Bitmasks{red_mask, red_shift, green_mask, green_shift, blue_mask, blue_shift, alpha_mask: 0, alpha_shift: 0});
        }
        Ok((i, ()))
    }

    fn parse_alpha_mask(&mut self, input: Input<'a>) -> Result<'a, ()> {
        //println!("parse_alpha_masks: {:?}", &input[0..64]);
        use nom::{
            error::context,
            number::complete::le_u32,
        };
        let (i, alpha_mask) = context("Alpha channel bitmask", le_u32)(input)?;
        if let Some(bm) = &self.bitmasks {
            if alpha_mask != 0 {
                let alpha_shift = shift_from_mask(alpha_mask);
                self.bitmasks = Some(Bitmasks{
                    red_mask: bm.red_mask, red_shift: bm.red_shift,
                    green_mask: bm.green_mask, green_shift: bm.green_shift, 
                    blue_mask: bm.blue_mask, blue_shift: bm.blue_shift,
                    alpha_mask, alpha_shift});
            }
        }
        Ok((i, ()))
    }

    fn parse_v4_part(&mut self, input: Input<'a>) -> Result<'a, ()> {
        //println!("parse_v4_part: {:?}", &input[0..64]);
        use nom::{
            combinator::{map, map_res},
            error::context,
            number::complete::le_u32,
            sequence::tuple,
        };
        let (i, (
                    cs_type,
                    // Endpoints
                    red_x, red_y, red_z,
                    green_x, green_y, green_z,
                    blue_x, blue_y, blue_z,
                    // Gamma
                    red, green, blue,
        )) = tuple((
            context("Color space type", map_res(le_u32, |x| ColorSpaceType::try_from(x))),
            context("Color space endpoint: red x", map(le_u32, |x| fixpt2_30(x))),
            context("Color space endpoint: red y", map(le_u32, |x| fixpt2_30(x))),
            context("Color space endpoint: red z", map(le_u32, |x| fixpt2_30(x))),
            context("Color space endpoint: green x", map(le_u32, |x| fixpt2_30(x))),
            context("Color space endpoint: green y", map(le_u32, |x| fixpt2_30(x))),
            context("Color space endpoint: green z", map(le_u32, |x| fixpt2_30(x))),
            context("Color space endpoint: blue x", map(le_u32, |x| fixpt2_30(x))),
            context("Color space endpoint: blue y", map(le_u32, |x| fixpt2_30(x))),
            context("Color space endpoint: blue z", map(le_u32, |x| fixpt2_30(x))),
            context("Gamma for red channel", map(le_u32, |x| fixpt16_16(x))),
            context("Gamma for green channel", map(le_u32, |x| fixpt16_16(x))),
            context("Gamma for blue channel", map(le_u32, |x| fixpt16_16(x))),
        ))(input)?;
        self.color_space_type = cs_type;
        // TODO: Waht if all endpoints are 0?
        if cs_type == ColorSpaceType::LCS_CALIBRATED_RGB {
            self.endpoints = Some(Endpoints{
                red: Endpoint{x: red_x, y: red_y, z: red_z},
                green: Endpoint{x: green_x, y: green_y, z: green_z},
                blue: Endpoint{x: blue_x, y: blue_y, z: blue_z},
                });
        }
        // TODO: Is it possible to have only some of the gammas?
        if red != 0.0 && green != 0.0 && blue != 0.0 {
            self.gammas = Some(Gammas{red, green, blue});
        }
        Ok((i, ()))
    }

    fn parse_v5_part(&mut self, input: Input<'a>) -> Result<'a, ()> {
        //println!("parse_v5_part: {:?}", &input[0..64]);
        use nom::{
            combinator::map_res,
            error::context,
            number::complete::le_u32,
            sequence::tuple,
        };
        let (i, (
                    intent, _icc_data, _icc_size, _reserved,
        )) = tuple((
            context("Intent", map_res(le_u32, |x| IntentType::try_from(x))),
            context("ICC profile data", le_u32),
            context("ICC profile size", le_u32),
            context("Reserved", le_u32),
        ))(input)?;
        self.intent = if intent != IntentType::UNKNOWN { Some(intent) } else { None };
        Ok((i, ()))
    }

    fn parse_os2_64_part(&mut self, input: Input<'a>) -> Result<'a, ()> {
        //println!("parse_os2_64_part: {:?}", &input[0..64]);
        use nom::{
            error::context,
            number::complete::{le_u16, le_u32},
            sequence::tuple,
        };
        let (i, (
                    _units, _reserved, _recording, _rendering, _size1, _size2,
                    _color_encoding, _identifier
        )) = tuple((
            context("Units", le_u16),
            context("Reserved", le_u16),
            context("Recording", le_u16),
            context("Rendering", le_u16),
            context("Size1", le_u32),
            context("Size2", le_u32),
            context("ColorEncoding", le_u32),
            context("Identifier", le_u32),
        ))(input)?;
        Ok((i, ()))
    }

    fn parse_colors(&mut self, input: Input<'a>, nb_colors: u32) -> Result<'a, ()> {
        let mut i = input;
        for _ in 0..nb_colors {
            let (j, color) = Color::parse(i, self.dib_header_size != DibHeaderSize::BITMAPCOREHEADER)?;
            self.palette.push(color);
            i = j;
        }
        Ok((i, ()))
    }

    fn to_srgb(&self, color: Color) -> Color {
        if self.gammas == None && self.endpoints == None {
            return color;
        }
        let mut colorf = normalize(color);
        /*
        if let Some(endpoints) = &self.endpoints {
            colorf = to_xyz(colorf, endpoints);
            colorf = xyz_to_srgb(colorf);
        }
        */
        if let Some(gammas) = &self.gammas {
            colorf = apply_gammas(colorf, gammas);
        }
        if let Some(endpoints) = &self.endpoints {
            colorf = to_xyz(colorf, endpoints);
            // It turns out xyz in BMP is actually xyY. Either the specs are wrong and it should
            // read xyY instead of XYZ, or the example I found is wrong (as well as firefox &
            // chrome implementations). The latter is not that unlikely considering BMP with gamma
            // are rare and both implementations used the said example to test their code.
            //colorf = xyY_to_xyz(colorf);
            colorf = xyz_to_srgb(colorf);
            // TODO: Remove this: this is just a test
            colorf.red = 210.0 * colorf.red / 255.0;
            colorf.green = 255.0 * colorf.green / 236.0;
            colorf.blue = 255.0 * colorf.blue / 235.0;
        }
        // Unapply sRGB gamma as it will be applied by the display?
        /*
        if let Some(_) = &self.gammas {
            colorf = unapply_gammas(colorf, &Gammas{ red: 2.2, green: 2.2, blue: 2.2 });
        }
        */
        denormalize(colorf)
    }

    fn put_pixels(&mut self, x: &mut i32, y: &i32, idx: usize, length: usize) -> Result<'a, ()> {
        if idx >= self.palette.len() {
            // TODO: Better error (e.g color outside of palette)
            return Err(nom::Err::Failure(nom::error::make_error::<Input, nom::error::VerboseError<Input>>(&[], nom::error::ErrorKind::TooLarge)));
        }
        let color = self.palette[idx];
        for _ in 0..length {
            if *x as usize >= self.pixels.len() {
                // TODO: Better error (e.g pixel outside of image)
                return Err(nom::Err::Failure(nom::error::make_error::<Input, nom::error::VerboseError<Input>>(&[], nom::error::ErrorKind::TooLarge)));
            }
            if *y as usize > self.pixels[*x as usize].len() {
                // TODO: Better error (e.g pixel outside of image)
                return Err(nom::Err::Failure(nom::error::make_error::<Input, nom::error::VerboseError<Input>>(&[], nom::error::ErrorKind::TooLarge)));
            }
            self.pixels[*x as usize][*y as usize] = self.to_srgb(color);
            *x += 1;
        }
        Ok((&[], ()))
    }

    fn put_24bpp_pixels(&mut self, x: &mut i32, y: &i32, color: Color, length: usize) -> Result<'a, ()> {
        for _ in 0..length {
            if *x as usize >= self.pixels.len() {
                // TODO: Better error (e.g pixel outside of image)
                return Err(nom::Err::Failure(nom::error::make_error::<Input, nom::error::VerboseError<Input>>(&[], nom::error::ErrorKind::TooLarge)));
            }
            if *y as usize > self.pixels[*x as usize].len() {
                // TODO: Better error (e.g pixel outside of image)
                return Err(nom::Err::Failure(nom::error::make_error::<Input, nom::error::VerboseError<Input>>(&[], nom::error::ErrorKind::TooLarge)));
            }
            self.pixels[*x as usize][*y as usize] = self.to_srgb(color);
            *x += 1;
        }
        Ok((&[], ()))
    }

    fn pixels_from_rle(&mut self) -> Result<'a, ()> {
        // Copying the data just to avoid data_iter borrowing self as well as put_pixels ...
        // TODO: Avoid this copy ... Maybe by extracting data and pixels in different struct.
        let data_copy = self.data.clone();
        let mut data_iter = data_copy.iter();
        let topdown = self.height < 0;
        let mut y = if topdown { 0 } else { self.height-1 };
        let dir_y = if topdown { 1 } else { -1 };
        let mut x = 0;
        while let Some(control) = data_iter.next() {
            // TODO: Better error (e.g missing escape byte)
            let escape = *data_iter.next().ok_or(nom::Err::Failure(nom::error::make_error::<Input, nom::error::VerboseError<Input>>(&[], nom::error::ErrorKind::TooLarge)))? as usize;
            match control {
                0 => {
                    match escape {
                        0 => {
                            // end of line
                            x = 0;
                            y += dir_y;
                        },
                        1 => {
                            // end of image
                            return Ok((&[], ()));
                        },
                        2 => {
                            // displacement mode
                            // TODO: Better error (e.g missing x/y in displacement mode)
                            let dx = *data_iter.next().ok_or(nom::Err::Failure(nom::error::make_error::<Input, nom::error::VerboseError<Input>>(&[], nom::error::ErrorKind::TooLarge)))? as i32;
                            let dy = *data_iter.next().ok_or(nom::Err::Failure(nom::error::make_error::<Input, nom::error::VerboseError<Input>>(&[], nom::error::ErrorKind::TooLarge)))? as i32;
                            x += dx;
                            y += dir_y * dy;
                        },
                        pixels => {
                            // absolute mode
                            let bytes;
                            match self.bpp {
                                4 => {
                                    bytes = (pixels + 1) / 2;
                                    let mut i = 0;
                                    while i < pixels {
                                        let indexes = *data_iter.next().ok_or(nom::Err::Failure(nom::error::make_error::<Input, nom::error::VerboseError<Input>>(&[], nom::error::ErrorKind::TooLarge)))? as usize;
                                        let idx = (indexes & 0xf0) >> 4;
                                        self.put_pixels(&mut x, &y, idx, 1)?;
                                        i += 1;
                                        if i >= pixels {
                                            break;
                                        }
                                        let idx = indexes & 0xf;
                                        self.put_pixels(&mut x, &y, idx, 1)?;
                                        i += 1;
                                    }
                                },
                                8 => {
                                    bytes = pixels;
                                    for _ in 0..pixels {
                                        // TODO: Better error (e.g missing pixel in absolute mode)
                                        let idx = *data_iter.next().ok_or(nom::Err::Failure(nom::error::make_error::<Input, nom::error::VerboseError<Input>>(&[], nom::error::ErrorKind::TooLarge)))? as usize;
                                        self.put_pixels(&mut x, &y, idx, 1)?;
                                    }
                                },
                                24 => {
                                    // TODO: Find examples of RLE24 to test this
                                    bytes = pixels * 3;
                                    for _ in 0..pixels {
                                        // TODO: Better error (e.g missing pixel in absolute mode)
                                        // I'm assuming bitmasks are not used with RLE24. I may be
                                        // wrong ...
                                        let blue = *data_iter.next().ok_or(nom::Err::Failure(nom::error::make_error::<Input, nom::error::VerboseError<Input>>(&[], nom::error::ErrorKind::TooLarge)))?;
                                        let green = *data_iter.next().ok_or(nom::Err::Failure(nom::error::make_error::<Input, nom::error::VerboseError<Input>>(&[], nom::error::ErrorKind::TooLarge)))?;
                                        let red = *data_iter.next().ok_or(nom::Err::Failure(nom::error::make_error::<Input, nom::error::VerboseError<Input>>(&[], nom::error::ErrorKind::TooLarge)))?;
                                        self.put_24bpp_pixels(&mut x, &y, Color{red, green, blue, alpha: 255}, 1)?;
                                    }
                                },
                                _ => {
                                    // TODO: Better error (e.g unsupported bpp for RLE)
                                    return Err(nom::Err::Failure(nom::error::make_error::<Input, nom::error::VerboseError<Input>>(&[], nom::error::ErrorKind::TooLarge)));
                                },
                            }
                            if (bytes % 2) == 1 {
                                // TODO: Better error (e.g missing padding in absolute mode)
                                data_iter.next().ok_or(nom::Err::Failure(nom::error::make_error::<Input, nom::error::VerboseError<Input>>(&[], nom::error::ErrorKind::TooLarge)))?;
                            }
                        },
                    }
                },
                length => {
                    match self.bpp {
                        4 => {
                            let mut i = 0;
                            let idx1 = (escape & 0xf0) >> 4;
                            let idx2 = escape & 0xf;
                            while i < *length {
                                self.put_pixels(&mut x, &y, idx1, 1)?;
                                i += 1;
                                if i >= *length {
                                    break;
                                }
                                self.put_pixels(&mut x, &y, idx2, 1)?;
                                i += 1;
                            }
                        },
                        8 => {
                            self.put_pixels(&mut x, &y, escape, *length as usize)?;
                        },
                        24 => {
                            let blue = escape as u8;
                            let green = *data_iter.next().ok_or(nom::Err::Failure(nom::error::make_error::<Input, nom::error::VerboseError<Input>>(&[], nom::error::ErrorKind::TooLarge)))?;
                            let red = *data_iter.next().ok_or(nom::Err::Failure(nom::error::make_error::<Input, nom::error::VerboseError<Input>>(&[], nom::error::ErrorKind::TooLarge)))?;
                            self.put_24bpp_pixels(&mut x, &y, Color{red, green, blue, alpha: 255}, *length as usize)?;
                        },
                        // TODO: Better error (e.g unsupported bpp for RLE)
                        _ => {
                            return Err(nom::Err::Failure(nom::error::make_error::<Input, nom::error::VerboseError<Input>>(&[], nom::error::ErrorKind::TooLarge)));
                        },
                    }
                },
            }
        }

        Ok((&[], ()))
    }

    fn pixels_from_uncompressed(&mut self) -> Result<'a, ()> {
        let mut line_bytes = ((self.width*self.bpp as i32)/8) as usize;
        let ppb = (8/self.bpp) as usize;
        if line_bytes%4 != 0 {
            line_bytes += 4 - (line_bytes%4);
        }
        let topdown = self.height < 0;
        let image_bytes = line_bytes * self.height.abs() as usize;
        if image_bytes > self.data.len() {
            // TODO: Better error (e.g file smaller than expected)
            return Err(nom::Err::Failure(nom::error::make_error::<Input, nom::error::VerboseError<Input>>(&[], nom::error::ErrorKind::TooLarge)));
        }
        match self.bpp {
            1 | 2 | 4 | 8 => {
                for b in 0..(image_bytes as usize) {
                    let indexes = self.data[b] as usize;
                    for d in 0..ppb {
                        let x = (ppb*b + d)%(line_bytes*ppb);
                        if x >= self.width as usize {
                            // We're in padding bytes
                            continue;
                        }
                        let y = b/line_bytes;
                        let shift = 8-(d+1)*self.bpp as usize;
                        let mask = (2_u32.pow(self.bpp as u32) as usize-1) << shift;
                        let idx = (indexes & mask) >> shift;
                        if idx >= self.palette.len() {
                            // TODO: Better error (e.g color outside of palette)
                            return Err(nom::Err::Failure(nom::error::make_error::<Input, nom::error::VerboseError<Input>>(&[], nom::error::ErrorKind::TooLarge)));
                        }
                        let color = self.palette[idx];
                        let y = if topdown { y } else {self.height as usize - y - 1 };
                        self.pixels[x][y] = self.to_srgb(color);
                    }
                }
            },
            16 | 24 | 32 => {
                let bytespp = self.bpp / 8;
                let mut aligned_width = self.width;
                while (aligned_width as u64*bytespp as u64) % 4 != 0 {
                    aligned_width += 1;
                }
                let bitmasks = if let Some(bm) = &self.bitmasks {
                    bm
                } else {
                    match self.bpp {
                        16 => {
                            // From https://www.fileformat.info/format/bmp/egff.htm#MICBMP-DMYID.3.6:
                            // The Compression field must always be a value of 3 (bitfields encoding) when
                            // a file stores 16-bit data.
                            // And yet other places mention a default 16 bits masks of RGB-555
                            &Bitmasks {
                                red_mask: 0x7c00,
                                red_shift: 10,
                                green_mask: 0x3e0,
                                green_shift: 5,
                                blue_mask: 0x1f,
                                blue_shift: 0,
                                alpha_mask: 0,
                                alpha_shift: 0,
                            }
                        },
                        24 => {
                            &Bitmasks {
                                red_mask: 0xff0000,
                                red_shift: 16,
                                green_mask: 0x00ff00,
                                green_shift: 8,
                                blue_mask: 0x0000ff,
                                blue_shift: 0,
                                alpha_mask: 0,
                                alpha_shift: 0,
                            }
                        },
                        32 => {
                            &Bitmasks {
                                red_mask: 0x00ff0000,
                                red_shift: 16,
                                green_mask: 0x0000ff00,
                                green_shift: 8,
                                blue_mask: 0x000000ff,
                                blue_shift: 0,
                                alpha_mask: 0,
                                alpha_shift: 0,
                            }
                        },
                        _ => {
                            // TODO: Maybe replace by an error once I find out how to put any
                            // message in them.
                            panic!("Got bpp={}, which is totally unexpected!", self.bpp);
                        },
                    }
                };
                for x in 0..(self.width as usize) {
                    for y in 0..(self.height.abs() as usize) {
                        let offset = y*aligned_width as usize + x as usize;
                        let mut val = 0;
                        for i in 0..bytespp as usize {
                            val += (self.data[bytespp as usize*offset+i] as u32) << 8*i;
                        }
                        let red = normalize_from_mask(val, bitmasks.red_mask, bitmasks.red_shift);
                        let green = normalize_from_mask(val, bitmasks.green_mask, bitmasks.green_shift);
                        let blue = normalize_from_mask(val, bitmasks.blue_mask, bitmasks.blue_shift);
                        let alpha = normalize_from_mask(val, bitmasks.alpha_mask, bitmasks.alpha_shift);
                        let y = if topdown { y } else {self.height as usize - y - 1 };
                        self.pixels[x][y] = self.to_srgb(Color{red, green, blue, alpha});
                    }
                }
            }
            _ => {
                // TODO: Better error (e.g unsupported bpp)
                return Err(nom::Err::Failure(nom::error::make_error::<Input, nom::error::VerboseError<Input>>(&[], nom::error::ErrorKind::TooLarge)));
            }
        };
        Ok((&[], ()))
    }

    fn pixels_from_data(&mut self) -> Result<'a, ()> {
        self.pixels = vec![vec![Color{ ..Default::default() }; self.height.abs() as usize]; self.width as usize];
        match self.compression {
            None |
            Some(CompressionMethod::BI_RGB) |
            Some(CompressionMethod::BI_BITFIELDS) |
            Some(CompressionMethod::BI_ALPHABITFIELDS) => self.pixels_from_uncompressed(),

            // TODO: Support anything below this
            Some(CompressionMethod::BI_RLE4) |
            Some(CompressionMethod::BI_RLE8) |
            Some(CompressionMethod::BI_RLE24) => self.pixels_from_rle(),

            // TODO: Replace panic by a proper error.
            Some(CompressionMethod::BI_JPEG) |
            Some(CompressionMethod::BI_PNG) |
            Some(CompressionMethod::BI_CMYK) |
            Some(CompressionMethod::BI_CMYKRLE8) |
            Some(CompressionMethod::BI_CMYKRLE4) => panic!("Unsupported compression: {:?}", self.compression),
        }
    }
}

impl BmpFile {
    pub fn parse(input: Input) -> Result<Self> {
        let mut result = BmpFile { ..Default::default() };
        let (i, _) = result.parse_file_header(input)?;
        let (i, _) = result.parse_dib_header_size(i)?;
        let (i, _) = match result.dib_header_size {
            // From https://en.wikipedia.org/wiki/BMP_file_format:
            // "The Windows 2.x BITMAPCOREHEADER differs from the OS/2 1.x BITMAPCOREHEADER [...]
            // in [...] that the image width and height fields are signed integers, not unsigned."
            //
            // But it seems there's no way to differentiate both versions (same dib header size, no
            // other difference). So we have to pick:
            //  - Decoding a OS/2 bitmap bigger with signed would fail on bitmap bigger than 16k
            //  - Decoding a Win bitmap with unsigned would fail on bitmap with negative height
            //    (which is valid, it means top-down instead of bottom-up)
            // The latter seems more likely than the former, especially considering these are very
            // old versions of the header (pre-1992). So let's go with i16 ...
            DibHeaderSize::BITMAPCOREHEADER => result.parse_i16_width_and_height(i)?,
            //DibHeaderSize::OS21XBITMAPHEADER => result.parse_u16_width_and_height(i)?,
            _ => result.parse_i32_width_and_height(i)?,
        };
        let (i, _) = result.parse_end_of_core_header(i)?;

        let (mut i, _) = match result.dib_header_size {
            DibHeaderSize::BITMAPCOREHEADER |
            //DibHeaderSize::OS21XBITMAPHEADER |
            DibHeaderSize::OS22XBITMAPHEADER16 => {
                // Nothing to do
                (i, ())
            },
            _ => {
                result.parse_info_header(i)?
            }
        };
        if result.dib_header_size == DibHeaderSize::BITMAPINFOHEADER &&
            (result.compression == Some(CompressionMethod::BI_BITFIELDS) ||
             result.compression == Some(CompressionMethod::BI_ALPHABITFIELDS)) {
            // https://docs.microsoft.com/en-us/windows/win32/gdi/bitmap-header-types
            // When the biCompression member of BITMAPINFOHEADER is set to BI_BITFIELDS and the
            // function receives an argument of type LPBITMAPINFO, the color masks will immediately
            // follow the header. [...] BITMAPCOREHEADER bitmaps do not support color masks.
            let (j, _) = result.parse_rgb_masks(i)?;
            i = j;
            if result.compression == Some(CompressionMethod::BI_ALPHABITFIELDS) {
                let (j, _) = result.parse_alpha_mask(i)?;
                i = j;
            }
        }
        match result.dib_header_size {
            //DibHeaderSize::OS21XBITMAPHEADER |
            DibHeaderSize::OS22XBITMAPHEADER16 |
            DibHeaderSize::BITMAPCOREHEADER |
            DibHeaderSize::BITMAPINFOHEADER => {
                // Nothing more to parse
            }
            DibHeaderSize::OS22XBITMAPHEADER64 => {
                let (j, _) = result.parse_os2_64_part(i)?;
                i = j;
            }
            DibHeaderSize::BITMAPV2INFOHEADER |
            DibHeaderSize::BITMAPV3INFOHEADER |
            DibHeaderSize::BITMAPV4HEADER |
            DibHeaderSize::BITMAPV5HEADER => {
                // TODO: Ensure stuff below is set in result
                if result.dib_header_size as u32 >= DibHeaderSize::BITMAPV2INFOHEADER as u32 {
                    // BITMAPV2INFOHEADER part.
                    let (j, _) = result.parse_rgb_masks(i)?;
                    i = j;
                }
                if result.dib_header_size as u32 >= DibHeaderSize::BITMAPV3INFOHEADER as u32 {
                    // BITMAPV3INFOHEADER part.
                    let (j, _) = result.parse_alpha_mask(i)?;
                    i = j;
                }
                if result.dib_header_size as u32 >= DibHeaderSize::BITMAPV4HEADER as u32 {
                    // BITMAPV4HEADER part.
                    let (j, _) = result.parse_v4_part(i)?;
                    i = j;
                }
                if result.dib_header_size as u32 >= DibHeaderSize::BITMAPV5HEADER as u32 {
                    // BITMAPV5HEADER part.
                    let (j, _) = result.parse_v5_part(i)?;
                    i = j;
                }
            }
        }

        let (i, _) = result.parse_colors(i, result.colors_in_table)?;

        let (_, data) = input.split_at(result.offset as usize);
        //let (data, _) = data.split_at(result.image_size as usize);
        result.data = data.to_vec();

        //println!("{:#?}", result);
        result.pixels_from_data()?;

        Ok((i, result))
    }
}
