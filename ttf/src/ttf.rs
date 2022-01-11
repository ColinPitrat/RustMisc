extern crate bitflags;
extern crate fixed;
extern crate nom;

use bitflags::bitflags;
use chrono::NaiveDateTime;
use fixed::{FixedU32,types::extra::U16};
use num_enum::{TryFromPrimitive,TryFromPrimitiveError};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u32)]
pub enum ScalerType {
    TrueType = 0x00010000,
    TrueType_ = 0x74727565,
    PostScript = 0x74797031,
    OpenType = 0x4F54544F,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OffsetSubtable {
    scaler_type: ScalerType,
    num_tables: u16,
    search_range: u16,
    entry_selector: u16,
    range_shift: u16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TableDirectoryEntry {
    tag: String,
    checksum: u32,
    offset: u32,
    length: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TableDirectory {
    entries: HashMap<String, TableDirectoryEntry>,
}

#[derive(Debug, PartialEq)]
pub struct FontDirectory {
    offset_subtable: OffsetSubtable,
    table_directory: TableDirectory,
}

#[derive(Clone, PartialEq)]
pub struct FWord(i16);

impl fmt::Display for FWord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} FUnits", self.0)
    }
}

impl fmt::Debug for FWord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

// The long internal format of a date in seconds since 12:00 midnight, January 1, 1904. It is
// represented as a signed 64-bit integer.
#[derive(Clone, PartialEq)]
pub struct LongDateTime(i64);

impl fmt::Display for LongDateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // -2082843279 is the timestamp of 1904-01-01 00:00:00
        let timestamp = self.0 - 2082841200;
        let this_date_time = NaiveDateTime::from_timestamp(timestamp, 0);
        write!(f, "{}", this_date_time.format("%Y-%m-%d %H:%M:%S"))
    }
}

impl fmt::Debug for LongDateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(i16)]
pub enum IndexToLocFormat {
    ShortOffsets = 0,
    LongOffsets = 1,
}

#[derive(Debug, PartialEq)]
pub struct FontHeader {
    version: FixedU32<U16>,
    revision: FixedU32<U16>,
    checksum_adjustment: u32,
    // TODO: type for flags
    flags: u16,
    units_per_em: u16,
    created: LongDateTime,
    modified: LongDateTime,
    x_min: FWord,
    y_min: FWord,
    x_max: FWord,
    y_max: FWord,
    mac_style: u16,
    lowest_rec_ppem: u16,
    font_direction_hint: i16,
    index_to_loc_format: IndexToLocFormat,
    glyph_data_format: i16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MaxProfile {
    // TODO: Remove public fields
    pub version: FixedU32<U16>,
    pub nb_glyphs: u16
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u16)]
pub enum PlatformId {
    Unicode = 0,
    Macintosh = 1,
    Microsoft = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u16)]
pub enum UnicodePlatformSpecificId {
    Version1_0 = 0,
    Version1_1 = 1,
    Iso10646 = 2,
    Unicode2Bmp = 3,
    Unicode2 = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u16)]
pub enum MacintoshPlatformSpecificId {
    Roman = 0,
    Japanese = 1,
    TraditionalChinese = 2,
    Korean = 3,
    Arabic = 4,
    Hebrew = 5,
    Greek = 6,
    Russian = 7,
    RSymbol = 8,
    Devanagari = 9,
    Gurmukyi = 10,
    Gujarati = 11,
    Oriya = 12,
    Bengali = 13,
    Tamil = 14,
    Telugu = 15,
    Kannada = 16,
    Malayalam = 17,
    Sinhalese = 18,
    Burmese = 19,
    Khmer = 20,
    Thai = 21,
    Laotian = 22,
    Georgian = 23,
    Armenian = 24,
    SimplifiedChinese = 25,
    Tibetan = 26,
    Mongolian = 27,
    Geez = 28,
    Slavic = 29,
    Vietnamese = 30,
    Sindhi = 31,
    Uninterpreted = 32,
}

// From https://docs.microsoft.com/en-us/typography/opentype/otspec140/name#enc3
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u16)]
pub enum MicrosoftPlatformSpecificId {
    Symbol = 0,
    UnicodeBmp = 1,
    ShiftJIS = 2,
    PRC = 3,
    Big5 = 4,
    Wansung = 5,
    Johab = 6,
    // 7, 8 and 9 are reserved
    UnicodeFull = 10,
}

#[derive(Debug, PartialEq)]
pub enum PlatformSpecificId {
    Unicode(UnicodePlatformSpecificId),
    Macintosh(MacintoshPlatformSpecificId),
    Microsoft(MicrosoftPlatformSpecificId),
}

impl PlatformSpecificId {
    pub fn try_from_unicode(x: u16) -> std::result::Result<Self, TryFromPrimitiveError<UnicodePlatformSpecificId>> {
        Ok(PlatformSpecificId::Unicode(UnicodePlatformSpecificId::try_from(x)?))
    }

    pub fn try_from_macintosh(x: u16) -> std::result::Result<Self, TryFromPrimitiveError<MacintoshPlatformSpecificId>> {
        Ok(PlatformSpecificId::Macintosh(MacintoshPlatformSpecificId::try_from(x)?))
    }

    pub fn try_from_microsoft(x: u16) -> std::result::Result<Self, TryFromPrimitiveError<MicrosoftPlatformSpecificId>> {
        Ok(PlatformSpecificId::Microsoft(MicrosoftPlatformSpecificId::try_from(x)?))
    }
}

// For names with a Unicode platformID, the language code is unused and should be set to 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u16)]
pub enum UnicodeLanguageId {
    Unicode = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u16)]
pub enum MacintoshLanguageId {
    English = 0,
    French = 1,
    German = 2,
    Italian = 3,
    Dutch = 4,
    Swedish = 5,
    Spanish = 6,
    Danish = 7,
    Portuguese = 8,
    Norwegian = 9,
    Hebrew = 10,
    Japanese = 11,
    Arabic = 12,
    Finnish = 13,
    Greek = 14,
    Icelandic = 15,
    Maltese = 16,
    Turkish = 17,
    Croatian = 18,
    ChineseTraditional = 19,
    Urdu = 20,
    Hindi = 21,
    Thai = 22,
    Korean = 23,
    Lithuanian = 24,
    Polish = 25,
    Hungarian = 26,
    Estonian = 27,
    Latvian = 28,
    Sami = 29,
    Faroese = 30,
    FarsiPersian = 31,
    Russian = 32,
    ChineseSimplified = 33,
    Flemish = 34,
    IrishGaelic = 35,
    Albanian = 36,
    Romanian = 37,
    Czech = 38,
    Slovak = 39,
    Slovenian = 40,
    Yiddish = 41,
    Serbian = 42,
    Macedonian = 43,
    Bulgarian = 44,
    Ukrainian = 45,
    Byelorussian = 46,
    Uzbek = 47,
    Kazakh = 48,
    AzerbaijaniCyrillic = 49,
    AzerbaijaniArabic = 50,
    Armenian = 51,
    Georgian = 52,
    Moldavian = 53,
    Kirghiz = 54,
    Tajiki = 55,
    Turkmen = 56,
    Mongolian = 57,
    MongolianCyrillic = 58,
    Pashto = 59,
    Kurdish = 60,
    Kashmiri = 61,
    Sindhi = 62,
    Tibetan = 63,
    Nepali = 64,
    Sanskrit = 65,
    Marathi = 66,
    Bengali = 67,
    Assamese = 68,
    Gujarati = 69,
    Punjabi = 70,
    Oriya = 71,
    Malayalam = 72,
    Kannada = 73,
    Tamil = 74,
    Telugu = 75,
    Sinhalese = 76,
    Burmese = 77,
    Khmer = 78,
    Lao = 79,
    Vietnamese = 80,
    Indonesian = 81,
    Tagalog = 82,
    MalayRoman = 83,
    MalayArabic = 84,
    Amharic = 85,
    Tigrinya = 86,
    Galla = 87,
    Somali = 88,
    Swahili = 89,
    KinyarwandaRuanda = 90,
    Rundi = 91,
    NyanjaChewa = 92,
    Malagasy = 93,
    Esperanto = 94,
    Welsh = 128,
    Basque = 129,
    Catalan = 130,
    Latin = 131,
    Quechua = 132,
    Guarani = 133,
    Aymara = 134,
    Tatar = 135,
    Uighur = 136,
    Dzongkha = 137,
    JavaneseRoman = 138,
    SundaneseRoman = 139,
    Galician = 140,
    Afrikaans = 141,
    Breton = 142,
    Inuktitut = 143,
    ScottishGaelic = 144,
    ManxGaelic = 145,
    IrishGaelicWithDot = 146,
    Tongan = 147,
    GreekPolytonic = 148,
    Greenlandic = 149,
    AzerbaijaniRoman = 150,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u16)]
pub enum MicrosoftLanguageId {
    Albanian = 0x041C,
    Basque = 0x042D,
    Byelorussian = 0x0423,
    Bulgarian = 0x0402,
    Catalan = 0x0403,
    Croatian = 0x041A,
    Czech = 0x0405,
    Danish = 0x0406,
    DutchStandard = 0x0413,
    DutchFlemish = 0x0813,
    EnglishAmerican = 0x0409,
    EnglishBritish = 0x0809,
    EnglishAustralian = 0x0C09,
    EnglishCanadian = 0x1009,
    EnglishNewZealand = 0x1409,
    EnglishIreland = 0x1809,
    Estonian = 0x0425,
    Finnish = 0x040B,
    FrenchStandard = 0x040C,
    FrenchBelgian = 0x080C,
    FrenchCanadian = 0x0C0C,
    FrenchSwiss = 0x100C,
    FrenchLuxembourg = 0x140C,
    GermanStandard = 0x0407,
    GermanSwiss = 0x0807,
    GermanAustrian = 0x0C07,
    GermanLuxembourg = 0x1007,
    GermanLiechtenstein = 0x1407,
    Greek = 0x0408,
    Hungarian = 0x040E,
    Icelandic = 0x040F,
    ItalianStandard = 0x0410,
    Italian = 0x0810,
    Latvian = 0x0426,
    Lithuanian = 0x0427,
    NorwegianBokmal = 0x0414,
    NorwegianNynorsk = 0x0814,
    Polish = 0x0415,
    PortugueseBrazilian = 0x0416,
    PortugueseStandard = 0x0816,
    Romanian = 0x0418,
    Russian = 0x0419,
    Slovak = 0x041B,
    Slovenian = 0x0424,
    SpanishTraditional = 0x040A,
    SpanishMexican = 0x080A,
    SpanishModern = 0x0C0A,
    Swedish = 0x041D,
    Turkish = 0x041F,
    Ukrainian = 0x0422
}

#[derive(Debug, PartialEq)]
pub enum LanguageId {
    Unicode(UnicodeLanguageId),
    Macintosh(MacintoshLanguageId),
    Microsoft(MicrosoftLanguageId),
}

impl LanguageId {
    pub fn try_from_unicode(x: u16) -> std::result::Result<Self, TryFromPrimitiveError<UnicodeLanguageId>> {
        Ok(LanguageId::Unicode(UnicodeLanguageId::try_from(x)?))
    }

    pub fn try_from_macintosh(x: u16) -> std::result::Result<Self, TryFromPrimitiveError<MacintoshLanguageId>> {
        Ok(LanguageId::Macintosh(MacintoshLanguageId::try_from(x)?))
    }

    pub fn try_from_microsoft(x: u16) -> std::result::Result<Self, TryFromPrimitiveError<MicrosoftLanguageId>> {
        Ok(LanguageId::Microsoft(MicrosoftLanguageId::try_from(x)?))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u16)]
pub enum KnownNameId {
    Copyright = 0,
    FontFamily = 1,
    FontSubfamily = 2,
    SubfamilyId = 3,
    FontName = 4,
    Version = 5,
    PostScriptName = 6,
    Trademark = 7,
    Manufacturer = 8,
    Designer = 9,
    Description = 10,
    VendorUrl = 11,
    DesignerUrl = 12,
    License = 13,
    LicenseUrl = 14,
    Reserved15 = 15,
    PreferredFamily = 16,
    PreferredSubfamily = 17,
    CompatibleFull = 18,
    SampleText = 19,
    PostScriptFontName = 20,
    VariationsPrefix = 25,
}

#[derive(PartialEq)]
pub struct NameId(u16);

impl fmt::Display for NameId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match KnownNameId::try_from(self.0) {
            Ok(val) => write!(f, "{:?} ({})", val, self.0),
            Err(_) => write!(f, "{}", self.0),
        }
    }
}

impl fmt::Debug for NameId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

// TODO: use enums for ID as defined in:
// https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6name.html
#[derive(Debug, PartialEq)]
pub struct NameRecordPos {
    platform_id: PlatformId,
    platform_specific_id: PlatformSpecificId,
    language_id: LanguageId,
    name_id: NameId,
    length: u16,
    offset: u16,
}

#[derive(Debug, PartialEq)]
pub struct NameRecord {
    platform_id: PlatformId,
    platform_specific_id: PlatformSpecificId,
    language_id: LanguageId,
    name_id: NameId,
    value: String,
}

#[derive(Debug, PartialEq)]
pub struct NameTable {
    format: u16,
    records: Vec<NameRecord>,
}

bitflags! {
    pub struct GlyfFlags: u8 {
        const ON_CURVE = 0b00000001;
        const X_SHORT  = 0b00000010;
        const Y_SHORT  = 0b00000100;
        const REPEAT  = 0b00001000;
        const X_SAME   = 0b00010000;
        const Y_SAME   = 0b00100000;
        const OVERLAP   = 0b01000000;
        const RESERVED   = 0b10000000;
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Point {
    // TODO: remove pub
    pub x: i32,
    pub y: i32,
    pub on_curve: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Contour {
    // TODO: remove pub
    pub points: Vec<Point>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum GlyphData {
    VoidGlyph,
    SimpleGlyph{
        endpoints_idx: Vec<u16>,
        instructions: Vec<u8>,
        flags: Vec<GlyfFlags>,
        xs: Vec<i32>,
        ys: Vec<i32>,
        // TODO: remove pub
        contours: Vec<Contour>,
    },
    CompoundGlyph,
}

#[derive(Debug, PartialEq)]
pub struct Glyph {
    // TODO: Remove public field
    pub nb_contours: i16,
    x_min: FWord,
    y_min: FWord,
    x_max: FWord,
    y_max: FWord,
    // TODO: Remove public field
    pub glyph_data: GlyphData,
}

#[derive(Debug, PartialEq)]
pub struct GlyphTable {
    pub glyphs: HashMap<u32, Glyph>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LocationTable {
    locations: Vec<u32>,
}

#[derive(Debug, PartialEq)]
pub struct TtfFile {
    font_directory: FontDirectory,
    font_header: Option<FontHeader>,
    // TODO: Remove public fields
    pub max_profile: Option<MaxProfile>,
    name_table: Option<NameTable>,
    loca_table: Option<LocationTable>,
    pub glyph_table: Option<GlyphTable>,
}

pub type Input<'a> = &'a [u8];
pub type Error<'a> = nom::error::VerboseError<Input<'a>>;
pub type Result<'a, O> = nom::IResult<Input<'a>, O, Error<'a>>;
pub type SimpleResult<'a, O> = std::result::Result<O, nom::Err<Error<'a>>>;

impl OffsetSubtable {
    pub fn parse(input: Input) -> Result<Self> {
        use nom::{
            combinator::map_res,
            error::context,
            number::complete::{be_u16, be_u32},
            sequence::tuple,
        };
        let (i, (scaler_type, num_tables, search_range, entry_selector, range_shift)) = tuple((
            context("Scaler Type", map_res(be_u32, |x| ScalerType::try_from(x))),
            context("Num Tables", be_u16),
            context("Search Range", be_u16),
            context("Entry Selector", be_u16),
            context("Range Shift", be_u16),
        ))(input)?;
        Ok((i, OffsetSubtable{
            scaler_type,
            num_tables,
            search_range,
            entry_selector,
            range_shift,
        }))
    }
}

impl TableDirectoryEntry {
    pub fn parse(input: Input) -> Result<Self> {
        use nom::{
            combinator::map_res,
            error::context,
            multi::count,
            number::complete::{be_u8, be_u32},
            sequence::tuple,
        };
        let (i, (tag, checksum, offset, length)) = tuple((
            context("Tag", map_res(count(be_u8, 4), |x| String::from_utf8(x))),
            context("Checksum", be_u32),
            context("Offset", be_u32),
            context("Length", be_u32),
        ))(input)?;
        Ok((i, TableDirectoryEntry{
            tag,
            checksum,
            offset,
            length
        }))
    }
}

impl TableDirectory {
    pub fn parse(input: Input, nb_entries: u16) -> Result<Self> {
        let mut entries = HashMap::new();
        let mut i = input;
        for _ in 0..nb_entries {
            let (j, entry) = TableDirectoryEntry::parse(i)?;
            entries.insert(entry.tag.clone(), entry);
            i = j;
        }
        Ok((i, TableDirectory{
            entries,
        }))
    }
}

impl FontDirectory {
    pub fn parse(input: Input) -> Result<Self> {
        let (i, offset_subtable) = OffsetSubtable::parse(input)?;
        let (i, table_directory) = TableDirectory::parse(i, offset_subtable.num_tables)?;
        Ok((i, FontDirectory{
            offset_subtable,
            table_directory,
        }))
    }
}

impl FontHeader {
    pub fn parse(input: Input) -> Result<Self> {
        use nom::{
            combinator::map_res,
            bytes::complete::tag,
            error::context,
            number::complete::{be_i16,be_u16,be_u32,be_i64},
            sequence::tuple,
        };
        let (i, (version, revision, checksum_adjustment)) = tuple((
            context("Version", map_res::<_,_,_,_,Error,_,_>(be_u32, |x| Ok(FixedU32::<U16>::from_bits(x)))),
            context("Revision", map_res::<_,_,_,_,Error,_,_>(be_u32, |x| Ok(FixedU32::<U16>::from_bits(x)))),
            context("Checksum Adjustment", be_u32),
        ))(input)?;
        let (i, _magic) = context("Magic Number", tag(&[0x5F, 0x0F, 0x3C, 0xF5]))(i)?;
        let (i, (flags, units_per_em, created, modified, x_min, y_min, x_max, y_max, mac_style, lowest_rec_ppem, font_direction_hint, index_to_loc_format, glyph_data_format)) = tuple((
            context("Flags", be_u16),
            context("Units per EM", be_u16),
            context("Created", map_res::<_,_,_,_,Error,_,_>(be_i64, |x| Ok(LongDateTime(x)))),
            context("Modified", map_res::<_,_,_,_,Error,_,_>(be_i64, |x| Ok(LongDateTime(x)))),
            context("X min", map_res::<_,_,_,_,Error,_,_>(be_i16, |x| Ok(FWord(x)))),
            context("X max", map_res::<_,_,_,_,Error,_,_>(be_i16, |x| Ok(FWord(x)))),
            context("Y min", map_res::<_,_,_,_,Error,_,_>(be_i16, |x| Ok(FWord(x)))),
            context("Y max", map_res::<_,_,_,_,Error,_,_>(be_i16, |x| Ok(FWord(x)))),
            context("Mac Style", be_u16),
            context("Lowest Rec PPEM", be_u16),
            context("Font Direction Hint", be_i16),
            context("Index to Loc Format", map_res(be_i16, |x| IndexToLocFormat::try_from(x))),
            context("Glyph Data Format", be_i16),
        ))(i)?;
        Ok((i, FontHeader{
            version,
            revision,
            checksum_adjustment,
            flags,
            units_per_em,
            created,
            modified,
            x_min,
            y_min,
            x_max,
            y_max,
            mac_style,
            lowest_rec_ppem,
            font_direction_hint,
            index_to_loc_format,
            glyph_data_format
        }))
    }
}

impl MaxProfile {
    pub fn parse(input: Input) -> Result<Self> {
        use nom::{
            combinator::map_res,
            error::context,
            number::complete::{be_u16,be_u32},
            sequence::tuple,
        };
        let (i, (version, nb_glyphs)) = tuple((
            context("Version", map_res::<_,_,_,_,Error,_,_>(be_u32, |x| Ok(FixedU32::<U16>::from_bits(x)))),
            context("Nb Glyphs", be_u16),
        ))(input)?;
        Ok((i, MaxProfile{
            version,
            nb_glyphs,
        }))
    }
}

impl NameRecordPos {
    pub fn parse(input: Input) -> Result<Self> {
        use nom::{
            combinator::map_res,
            error::context,
            number::complete::be_u16,
            sequence::tuple,
        };
        let (i, platform_id) = context("Platform ID", map_res(be_u16, |x| PlatformId::try_from(x)))(input)?;
        let (i, (platform_specific_id, language_id)) = match platform_id {
            PlatformId::Unicode => tuple((
                        context("Unicode platform specific ID", map_res(be_u16, |x| PlatformSpecificId::try_from_unicode(x))),
                        context("Unicode language ID", map_res(be_u16, |x| LanguageId::try_from_unicode(x))),
                    ))(i)?,
            PlatformId::Macintosh => tuple((
                        context("Macintosh platform specific ID", map_res(be_u16, |x| PlatformSpecificId::try_from_macintosh(x))),
                        context("Macintosh language ID", map_res(be_u16, |x| LanguageId::try_from_macintosh(x))),
                    ))(i)?,
            PlatformId::Microsoft => tuple((
                        context("Microsoft platform specific ID", map_res(be_u16, |x| PlatformSpecificId::try_from_microsoft(x))),
                        context("Microsoft language ID", map_res(be_u16, |x| LanguageId::try_from_microsoft(x))),
                    ))(i)?,
        };
        let (i, (name_id, length, offset)) = tuple((
            context("Name ID", map_res::<_,_,_,_,Error,_,_>(be_u16, |x| Ok(NameId(x)))),
            context("Length", be_u16),
            context("Offset", be_u16),
        ))(i)?;
        Ok((i, NameRecordPos{
            platform_id,
            platform_specific_id,
            language_id,
            name_id,
            length,
            offset,
        }))
    }
}

impl NameTable {
    pub fn parse(input: Input) -> Result<Self> {
        use nom::{
            combinator::map_res,
            error::context,
            multi::count,
            number::complete::be_u16,
            sequence::tuple,
        };
        let (mut i, (format, cnt, string_offset)) = tuple((
            context("Format", be_u16),
            context("Count", be_u16),
            context("String Offset", be_u16),
        ))(input)?;
        let mut records = vec!();
        for _ in 0..cnt {
            let (j, recordpos) = NameRecordPos::parse(i)?;
            let start = usize::from(string_offset+recordpos.offset);
            let end = start + usize::from(recordpos.length);
            // TODO: Identify cases where this is not UTF-16 BE.
            // From https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6name.html:
            // The character set encoding used for the raw name data is determined by the
            // platform and specific identifier codes. For example, if the platform identifier is
            // for macOS and the specific identifier is Roman, then the raw string data is
            // MacRoman. If the platform identifier is Unicode, then the raw string data is Unicode
            // text in the UTF-16BE encoding format. The character strings can thus be localized to
            // any language and script.
            let (_, value) = map_res(count(be_u16, (recordpos.length/2).into()), |x| String::from_utf16(&x))(&input[start..end])?;
            let record = NameRecord {
                platform_id: recordpos.platform_id,
                platform_specific_id: recordpos.platform_specific_id,
                language_id: recordpos.language_id,
                name_id: recordpos.name_id,
                value,
            };
            records.push(record);
            i = j;
        }
        Ok((i, NameTable{
            format,
            records,
        }))
    }
}

impl Glyph {
    pub fn parse(input: Input) -> Result<Self> {
        use nom::{
            combinator::map_res,
            error::context,
            number::complete::{be_u8,be_i16,be_u16},
            sequence::tuple,
        };
        let (mut i, (nb_contours, x_min, y_min, x_max, y_max)) = tuple((
            context("Nb Contours", be_i16),
            context("X Min", map_res::<_,_,_,_,Error,_,_>(be_i16, |x| Ok(FWord(x)))),
            context("Y Min", map_res::<_,_,_,_,Error,_,_>(be_i16, |x| Ok(FWord(x)))),
            context("X Max", map_res::<_,_,_,_,Error,_,_>(be_i16, |x| Ok(FWord(x)))),
            context("Y Max", map_res::<_,_,_,_,Error,_,_>(be_i16, |x| Ok(FWord(x)))),
        ))(input)?;
        let glyph_data = if nb_contours == 0 {
            GlyphData::VoidGlyph
        } else if nb_contours > 0 {
            let mut endpoints_idx = vec!();
            for _ in 0..nb_contours {
                let (j, idx) = context("Endpoint of contour", be_u16)(i)?;
                endpoints_idx.push(idx);
                i = j;
            }
            // Simple glyph
            let (mut i, nb_instructions) = context("Instructions Length", be_u16)(i)?;
            let mut instructions = vec!();
            for _ in 0..nb_instructions {
                let (j, inst) = context("Instruction", be_u8)(i)?;
                i = j;
                instructions.push(inst);
            }
            let nb_points = endpoints_idx.last().unwrap().clone() as usize + 1;
            let mut flags = vec!();
            while flags.len() < nb_points {
                let (j, flag) = context("Flags", map_res::<_,_,_,_,Error,_,_>(be_u8, |x| Ok(GlyfFlags { bits: x })))(i)?;
                i = j;
                flags.push(flag);
                if flag.contains(GlyfFlags::REPEAT) {
                    let (j, repeat) = context("Repeats", be_u8)(i)?;
                    i = j;
                    for _ in 0..repeat {
                        flags.push(flag);
                    }
                }
            }
            let mut xs = vec!();
            {
                let mut x = 0;
                for flag in flags.iter() {
                    if flag.contains(GlyfFlags::X_SHORT) {
                        // read one byte, sign provided by flag & GlyfFlags::X_SAME
                        let (j, dx) = context("X (byte)", be_u8)(i)?;
                        i = j;
                        if flag.contains(GlyfFlags::X_SAME) {
                            x += dx as i32;
                        } else {
                            x -= dx as i32;
                        }
                        xs.push(x);
                    } else {
                        if flag.contains(GlyfFlags::X_SAME) {
                            xs.push(x);
                        } else {
                            let (j, dx) = context("X (i16)", be_i16)(i)?;
                            i = j;
                            x += dx as i32;
                            xs.push(x);
                        }
                    }
                }
            }
            // TODO: Dedupe the code for x & y that are the same apart from flag values
            let mut ys = vec!();
            {
                let mut y = 0;
                for flag in flags.iter() {
                    if flag.contains(GlyfFlags::Y_SHORT) {
                        // read one byte, sign provided by flag & GlyfFlags::Y_SAME
                        let (j, dy) = context("Y (byte)", be_u8)(i)?;
                        i = j;
                        if flag.contains(GlyfFlags::Y_SAME) {
                            y += dy as i32;
                        } else {
                            y -= dy as i32;
                        }
                        ys.push(y);
                    } else {
                        if flag.contains(GlyfFlags::Y_SAME) {
                            ys.push(y);
                        } else {
                            let (j, dy) = context("Y (i16)", be_i16)(i)?;
                            i = j;
                            y += dy as i32;
                            ys.push(y);
                        }
                    }
                }
            }
            let mut contours = vec!();
            let mut i: usize = 0;
            for e in endpoints_idx.iter() {
                let mut points = vec!();
                while i <= *e as usize {
                    let (x, y, on_curve) = (xs[i], ys[i], flags[i].contains(GlyfFlags::ON_CURVE));
                    points.push(Point { x, y, on_curve });
                    i += 1;
                }
                contours.push(Contour{points});
            }
            // TODO: Read xCoordinates & yCoordinates taking into account repeats.
            // Flags (xshort & yshort) also tell if it's 1 byte (set) or 2 bytes (reset).
            // If short, xsame & ysame provide sign (>= 0 if set, <0 if not)
            // If not short, if set, it means coordinate is the same as the previous (and so no
            // data).
            // Separate them in N contours.
            GlyphData::SimpleGlyph{
                endpoints_idx,
                instructions,
                flags,
                xs,
                ys,
                contours,
            }
        } else {
            // nb_contours < 0 means compound glyph
            GlyphData::CompoundGlyph
        };
        Ok((i, Glyph{
            nb_contours,
            x_min,
            y_min,
            x_max,
            y_max,
            glyph_data,
        }))
    }
}

impl GlyphTable {
    pub fn parse(input: Input) -> Result<Self> {
        let mut glyphs = HashMap::new();
        let mut i = input;
        let mut n = 0;
        loop {
            println!("Remaining glyf length: {}", i.len());
            match Glyph::parse(i) {
                Ok((j, glyph)) => {
                    i = j;
                    glyphs.insert(n, glyph);
                },
                Err(nom::Err::Error(nom::error::VerboseError{errors})) if errors[0].1 == nom::error::VerboseErrorKind::Nom(nom::error::ErrorKind::Eof) => {
                    println!("Reached end of glyphs while reading it: {:?}", errors);
                    break;
                }
                Err(e) => {
                    return Err(e);
                }
            }
            if i.is_empty() {
                break;
            }
            n += 1;
        }
        Ok((i, GlyphTable{
            glyphs,
        }))
    }
}

impl LocationTable {
    pub fn parse_short(input: Input) -> Result<Self> {
        use nom::{
            error::context,
            number::complete::be_u16,
        };
        let mut locations = vec!();
        let mut i = input;
        for _ in 0..input.len()/2 {
            let (j, offset) = context("Offset", be_u16)(i)?;
            i = j;
            locations.push(offset as u32);
        }
        Ok((input, LocationTable{
            locations
        }))
    }

    pub fn parse_long(input: Input) -> Result<Self> {
        use nom::{
            error::context,
            number::complete::be_u32,
        };
        let mut locations = vec!();
        let mut i = input;
        for _ in 0..input.len()/4 {
            let (j, offset) = context("Offset", be_u32)(i)?;
            i = j;
            locations.push(offset);
        }
        Ok((input, LocationTable{
            locations
        }))
    }
}

// Required tables (for TrueType font, not necessarily for OpenType, bitmap...):
// 'cmap'   character to glyph mapping
// 'glyf'   glyph data
// 'head'   font header
// 'hhea'   horizontal header
// 'hmtx'   horizontal metrics
// 'loca'   index to location
// 'maxp'   maximum profile
// 'name'   naming
// 'post'   PostScript
impl TtfFile {
    pub fn parse_table<'a, F, T>(input: Input<'a>, directory: TableDirectory, tag: &str, parser: F) -> SimpleResult<'a, Option<T>>
        where F: Fn(Input) -> Result<T> {
        let entry = &directory.entries.get(tag);
        match entry {
            Some(e) => Ok(Some(parser(&input[e.offset as usize..(e.offset+e.length) as usize])?.1)),
            None => Ok(None),
        }
    }

    pub fn parse(input: Input) -> Result<Self> {
        let (i, font_directory) = FontDirectory::parse(input)?;
        let font_header = TtfFile::parse_table(input, font_directory.table_directory.clone(), "head", FontHeader::parse)?;
        let max_profile = TtfFile::parse_table(input, font_directory.table_directory.clone(), "maxp", MaxProfile::parse)?;
        let name_table = TtfFile::parse_table(input, font_directory.table_directory.clone(), "name", NameTable::parse)?;
        let loca_table = if font_header.as_ref().unwrap().index_to_loc_format == IndexToLocFormat::ShortOffsets {
            TtfFile::parse_table(input, font_directory.table_directory.clone(), "loca", LocationTable::parse_short)?
        } else {
            TtfFile::parse_table(input, font_directory.table_directory.clone(), "loca", LocationTable::parse_long)?
        };
        //let glyph_table = TtfFile::parse_table(input, font_directory.table_directory.clone(), "glyf", GlyphTable::parse)?;
        let entry = &font_directory.table_directory.entries.get("glyf").unwrap();
        let mut glyphs = HashMap::new();
        for l in loca_table.clone().unwrap().locations {
            if l == entry.length {
                break;
            }
            if !glyphs.contains_key(&l) {
                glyphs.insert(l, Glyph::parse(&input[(entry.offset+l) as usize..(entry.offset+entry.length) as usize])?.1);
            }
        }
        let glyph_table = Some(GlyphTable{
            glyphs
        });
        Ok((i, TtfFile{
            font_directory,
            font_header,
            max_profile,
            name_table,
            loca_table,
            glyph_table,
        }))
    }
}
