use std::{ops::RangeInclusive, rc::Rc, time::Duration};

use crate::{
    error::{translation_err, UniPenError},
    statements::Reserved,
    builder::unipen::UniPenBuilder
};

pub struct UniPen {
    pub version: f64,
    pub data_source: Rc<str>,
    pub data_id: Rc<str>,
    pub coordinate_order: Vec<CoordinateType>,
    pub hierarchy_order: Vec<Rc<str>>,
    //data_documentation: DataDocumentation,
    //alphabet: Option<Vec<AlphabetItem>>,
    //lexicon: Option<Rc<Lexicon>>,
    //layout: Layout,
    //units: Units,
    //date: Option<Date>,
    //style: Option<Style>,
    //writer: Writer,
    //sets: Vec<Set>,
    //bounding_boxes: Vec<BoundingBox>,
    //recognizer: Recognizer,

    // TODO Recognition tagging unimplemented
}

impl UniPen {
    #[must_use]
    pub fn builder() -> UniPenBuilder {
        UniPenBuilder::default()
    }
}

struct DataDocumentation {
    // Data Documentation
    data_contact: Option<Rc<str>>,
    data_info: Option<Rc<str>>,
    setup: Option<Rc<str>>,
    pad: Option<Rc<str>>,
}

struct Layout {
    // Data Layout
    x_dimension: Option<f64>,
    y_dimension: Option<f64>,
    h_lines: Option<Vec<f64>>,
    v_lines: Option<Vec<f64>>,
}

struct Units {
    // Unit System
    x_points_per_inch: Option<f64>,
    y_points_per_inch: Option<f64>,
    z_points_per_inch: Option<f64>,
    x_points_per_mm: Option<f64>,
    y_points_per_mm: Option<f64>,
    z_points_per_mm: Option<f64>,
    points_per_gram: Option<f64>,
    points_per_second: Option<f64>,
}

struct Writer {
    writer_id: Rc<str>,
    country: Option<Rc<str>>,
    hand: Option<Hand>,
    age: Option<f64>,
    sex: Option<Sex>,
    skill: Option<Skill>,
    writer_info: Option<Rc<str>>,
}

struct Recognizer {
    // Recognizer Documentation
    recognizer_source: Rc<str>,
    recognizer_id: Rc<str>,
    recognizer_contact: Option<Rc<str>>,
    recognizer_info: Option<Rc<str>>,
    recognizer_implementation: Option<Rc<str>>,
}

pub enum CoordinateType {
    XPosition,
    YPosition,
    Time,
    Pressure,
    ZPosition,
    Button,
    Rho,
    Theta,
    Phi,
}

impl TryFrom<&Reserved> for CoordinateType {
    type Error = UniPenError;

    fn try_from(value: &Reserved) -> Result<Self, UniPenError> {
        match value {
            Reserved::X => Ok(Self::XPosition),
            Reserved::Y => Ok(Self::YPosition),
            Reserved::Time => Ok(Self::Time),
            Reserved::Pressure => Ok(Self::Pressure),
            Reserved::Z => Ok(Self::ZPosition),
            Reserved::Button => Ok(Self::Button),
            Reserved::Rho => Ok(Self::Rho),
            Reserved::Theta => Ok(Self::Theta),
            Reserved::Phi => Ok(Self::Phi),
            _ => Err(translation_err!("No coordinate unit rule")),
        }
    }
}

struct AlphabetItem {
    character: char,
    frequency: Option<f64>,
}

struct LexiconItem {
    label: Rc<str>,
    frequency: Option<f64>,
}

pub struct Date {
    month: Option<i32>,
    day: Option<i32>,
    year: Option<i32>,
}

pub enum Style {
    Printed,
    Cursive,
    Mixed,
}

impl TryFrom<&Reserved> for Style {
    type Error = UniPenError;

    fn try_from(value: &Reserved) -> Result<Self, UniPenError> {
        match value {
            Reserved::Printed => Ok(Self::Printed),
            Reserved::Cursive => Ok(Self::Cursive),
            Reserved::Mixed => Ok(Self::Mixed),
            _ => Err(translation_err!("No style rule")),
        }
    }
}

pub enum Hand {
    Left,
    Right,
}

impl TryFrom<&Reserved> for Hand {
    type Error = UniPenError;

    fn try_from(value: &Reserved) -> Result<Self, UniPenError> {
        match value {
            Reserved::LeftHand => Ok(Self::Left),
            Reserved::RightHand => Ok(Self::Right),
            _ => Err(translation_err!("No hand rule")),
        }
    }
}

pub enum Sex {
    Male,
    Female,
}

impl TryFrom<&Reserved> for Sex {
    type Error = UniPenError;

    fn try_from(value: &Reserved) -> Result<Self, UniPenError> {
        match value {
            Reserved::Male => Ok(Self::Male),
            Reserved::Female => Ok(Self::Female),
            _ => Err(translation_err!("No sex rule")),
        }
    }
}

pub enum Skill {
    Bad,
    Ok,
    Good,
}

impl TryFrom<&Reserved> for Skill {
    type Error = UniPenError;

    fn try_from(value: &Reserved) -> Result<Self, UniPenError> {
        match value {
            Reserved::Bad => Ok(Self::Bad),
            Reserved::Ok => Ok(Self::Ok),
            Reserved::Good => Ok(Self::Good),
            _ => Err(translation_err!("No skill rule")),
        }
    }
}

pub enum Quality {
    Ok,
    Good,
}

impl TryFrom<&Reserved> for Quality {
    type Error = UniPenError;

    fn try_from(value: &Reserved) -> Result<Self, UniPenError> {
        match value {
            Reserved::Ok => Ok(Self::Ok),
            Reserved::Good => Ok(Self::Good),
            _ => Err(translation_err!("No quality rule")),
        }
    }
}

struct Lexicon {
    // Lexicon
    lexicon_source: Option<Rc<str>>,
    lexicon_id: Option<Rc<str>>,
    lexicon_contact: Option<Rc<str>>,
    lexicon_info: Option<Rc<str>>,
    lexicon: Option<Vec<LexiconItem>>,
}

pub type CoordinateIndex = usize;

pub struct ComponentSet {
    pub name: Rc<str>,
    pub coordinates: Rc<[Coordinate]>,
    pub components: Rc<[Component]>,
    pub segments: Rc<[Segment]>,
    pub bounding_boxes: Rc<[BoundingBox]>,
}

pub struct Coordinate {
    pub x_position: f64,
    pub y_position: f64,
    pub time: Duration,
    pub pressure: Option<f64>,
    pub z_position: Option<f64>,
    pub button: Option<f64>,
    pub rho: Option<f64>,
    pub theta: Option<f64>,
    pub phi: Option<f64>,
}

pub enum Component {
    PenDown(RangeInclusive<CoordinateIndex>),
    PenUp(RangeInclusive<CoordinateIndex>),
    Dt(Duration),
}

pub struct Segment {
    pub hierarchy: Rc<str>,
    pub coordinates: Rc<[RangeInclusive<CoordinateIndex>]>,
    pub quality: Option<Quality>,
    pub label: Option<Rc<str>>,
}

pub struct BoundingBox {
    pub x_min: f64,
    pub y_min: f64,
    pub x_max: f64,
    pub y_max: f64,
    pub coordinates: Rc<[RangeInclusive<CoordinateIndex>]>,
}
