use std::{rc::Rc, time::Duration};

use crate::{
    error::{translation_err, UniPenError},
    statements::{Keyword, Statement, StatementArgument, Reserved},
    model::{
        Coordinate, CoordinateType, Hand, Sex, Skill,
        Style,
    },
};

use super::component_set::ComponentSetBuilder;

#[allow(clippy::module_name_repetitions)]
#[derive(Default)]
pub struct UniPenBuilder {
    // Used to determine the current set name when a set name is not defined
    file_stack: Vec<Rc<str>>,
    // Used to determine the coordinate time. For example, when .POINTS_PER_SECOND is used in place of T coordinate types
    current_time: Duration,
    // Used to collect data for the current component set
    current_component_set_builder: ComponentSetBuilder,
    // Old component set builders saved after a new component set is started
    component_set_builders: Vec<ComponentSetBuilder>,

    // UniPen data
    version: Option<f64>,
    data_source: Option<Rc<str>>,
    data_id: Option<Rc<str>>,
    coordinate_order: Option<Vec<CoordinateType>>,
    hierarchy_order: Option<Vec<Rc<str>>>,

    alphabet: Option<Vec<Rc<str>>>,
    alphabet_frequency: Option<Vec<i32>>,

    data_contact: Option<Rc<str>>,
    data_info: Option<Rc<str>>,
    setup: Option<Rc<str>>,
    pad: Option<Rc<str>>,

    lexicon_source: Option<Rc<str>>,
    lexicon_id: Option<Rc<str>>,
    lexicon_contact: Option<Rc<str>>,
    lexicon_info: Option<Rc<str>>,
    lexicon: Option<Vec<Rc<str>>>,
    lexicon_frequency: Option<Vec<i32>>,

    x_dimension: Option<i32>,
    y_dimension: Option<i32>,
    h_lines: Option<Vec<i32>>,
    v_lines: Option<Vec<i32>>,

    x_points_per_inch: Option<f64>,
    y_points_per_inch: Option<f64>,
    z_points_per_inch: Option<f64>,
    x_points_per_mm: Option<f64>,
    y_points_per_mm: Option<f64>,
    z_points_per_mm: Option<f64>,
    points_per_gram: Option<f64>,
    points_per_second: Option<f64>,

    style: Option<Style>,
    writer_id: Option<Rc<str>>,
    country: Option<Rc<str>>,
    hand: Option<Hand>,
    age: Option<i32>,
    sex: Option<Sex>,
    skill: Option<Skill>,
    writer_info: Option<Rc<str>>,
}

impl UniPenBuilder {
    fn pen_statement_to_coords(&self, arguments: &[StatementArgument]) -> Result<Vec<(Coordinate, f64)>, UniPenError> {
        let order = self
            .coordinate_order
            .as_ref()
            .ok_or(UniPenError::Validation("Pen statement before coordinate order".into()))?;

        let mut numbers = arguments
            .iter()
            .map(|x| {
                if let StatementArgument::Number(value) = x {
                    Ok(f64::from(value))
                } else {
                    Err(translation_err!(format!("Pen statement has non-number argument")))
                }
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .peekable();

        let mut coordinates = Vec::new();
        while numbers.peek().is_some() {
            let mut x_position: Option<f64> = None;
            let mut y_position: Option<f64> = None;
            let mut time: Option<f64> = None;
            let mut pressure: Option<f64> = None;
            let mut z_position: Option<f64> = None;
            let mut button: Option<f64> = None;
            let mut rho: Option<f64> = None;
            let mut theta: Option<f64> = None;
            let mut phi: Option<f64> = None;

            for coordinate_type in order {
                let number = numbers
                    .next()
                    .ok_or(UniPenError::Validation("Not enough numbers for coordinate order".into()))?;
                match coordinate_type {
                    CoordinateType::XPosition => x_position = Some(number),
                    CoordinateType::YPosition => y_position = Some(number),
                    CoordinateType::Time => time = Some(number),
                    CoordinateType::Pressure => pressure = Some(number),
                    CoordinateType::ZPosition => z_position = Some(number),
                    CoordinateType::Button => button = Some(number),
                    CoordinateType::Rho => rho = Some(number),
                    CoordinateType::Theta => theta = Some(number),
                    CoordinateType::Phi => phi = Some(number),
                }
                coordinates.push((
                    Coordinate {
                        x_position: x_position.ok_or(UniPenError::Validation("Missing X coordinate".into()))?,
                        y_position: y_position.ok_or(UniPenError::Validation("Missing Y coordinate".into()))?,
                        time: Duration::default(),
                        pressure,
                        z_position,
                        button,
                        rho,
                        theta,
                        phi,
                    },
                    time.ok_or(UniPenError::Validation("Missing Time coordinate".into()))?,
                ));
            }
        }
        Ok(coordinates)
    }

    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    /// Adds a statement to the `UniPenBuilder`'s representation of the UniPen data.
    ///
    /// # Arguments
    ///
    /// * `statement` - The statement to add.
    ///
    /// # Errors
    ///
    /// * `UniPenError::Translation` - If the builder was unable to translate the `Statement` into UniPen data.
    /// * `UniPenError::Validation` - If the builder was unable to create a valid structure from the UniPen data.
    ///
    pub fn statement(mut self, statement: &Statement) -> Result<Self, UniPenError> {
        macro_rules! statement_translation_err {
            ($msg:expr) => {
                Err(translation_err!(format!(
                    "Statement of {:?} has invalid argument: {}",
                    statement.keyword, $msg
                )))
            };
            () => {
                Err(translation_err!(format!(
                    "Statement of {:?} has invalid argument",
                    statement.keyword
                )))
            };
        }
        macro_rules! translate_arg {
            ( $variant:path, $into:expr, $closure:expr) => {
                match &statement.arguments[0] {
                    StatementArgument::Reserved(Reserved::Unknown) => {
                        $into = None;
                        Ok(self)
                    }
                    // https://github.com/rust-lang/rust-clippy/issues/1553
                    #[allow(clippy::redundant_closure_call)]
                    $variant(value) => {
                        $into = Some($closure(value)?);
                        Ok(self)
                    }
                    _ => statement_translation_err!(stringify!($variant)),
                }
            };
            ($variant:path, $closure:expr) => {{
                #[allow(clippy::collection_is_never_read)]
                let _unused;
                translate_arg!($variant, _unused, $closure)
            }};
        }
        macro_rules! translate_homo {
            ($variant:path, $into:expr, $closure:expr) => {{
                $into = Some(
                    statement
                        .arguments
                        .iter()
                        .map(|x| {
                            if let $variant(value) = x {
                                Ok($closure(value)?)
                            } else {
                                statement_translation_err!()
                            }
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                );
                Ok(self)
            }};
        }

        let to_str = |x: &Rc<str>| -> Result<_, UniPenError> { Ok(x.clone()) };
        let to_int = |x| -> Result<_, UniPenError> { Ok(i32::from(x)) };
        let to_float = |x| -> Result<_, UniPenError> { Ok(f64::from(x)) };

        #[allow(clippy::match_same_arms)] // TODO remove this when all arms are implemented
        match statement.keyword {
            Keyword::Keyword | Keyword::Reserve | Keyword::Comment => Ok(self),
            Keyword::Include => match &statement.arguments[0] {
                StatementArgument::String(value) => {
                    self.file_stack.push(value.clone());
                    self.current_component_set_builder = self.current_component_set_builder.name(value.clone());
                    Ok(self)
                }
                _ => statement_translation_err!(stringify!(StatementArgument::String)),
            },
            Keyword::Version => translate_arg!(StatementArgument::Number, self.version, to_float),
            Keyword::DataSource => translate_arg!(StatementArgument::FreeText, self.data_source, to_str),
            Keyword::DataId => translate_arg!(StatementArgument::String, to_str),
            Keyword::Coordinate => translate_homo!(StatementArgument::Reserved, self.coordinate_order, CoordinateType::try_from),
            Keyword::Hierarchy => translate_homo!(StatementArgument::String, self.hierarchy_order, to_str),
            Keyword::DataContact => translate_arg!(StatementArgument::FreeText, self.data_contact, to_str),
            Keyword::DataInfo => translate_arg!(StatementArgument::FreeText, self.data_info, to_str),
            Keyword::Setup => translate_arg!(StatementArgument::FreeText, self.setup, to_str),
            Keyword::Pad => translate_arg!(StatementArgument::FreeText, self.pad, to_str),
            Keyword::Alphabet => translate_homo!(StatementArgument::String, self.alphabet, to_str),
            Keyword::AlphabetFreq => translate_homo!(StatementArgument::Number, self.alphabet_frequency, to_int),
            Keyword::LexiconSource => translate_arg!(StatementArgument::FreeText, self.lexicon_source, to_str),
            Keyword::LexiconId => translate_arg!(StatementArgument::String, self.lexicon_id, to_str),
            Keyword::LexiconContact => translate_arg!(StatementArgument::FreeText, self.lexicon_contact, to_str),
            Keyword::LexiconInfo => translate_arg!(StatementArgument::FreeText, self.lexicon_info, to_str),
            Keyword::Lexicon => translate_homo!(StatementArgument::String, self.lexicon, to_str),
            Keyword::LexiconFreq => translate_homo!(StatementArgument::Number, self.lexicon_frequency, to_int),
            Keyword::XDimension => translate_arg!(StatementArgument::Number, self.x_dimension, to_int),
            Keyword::YDimension => translate_arg!(StatementArgument::Number, self.y_dimension, to_int),
            Keyword::HLine => translate_homo!(StatementArgument::Number, self.h_lines, to_int),
            Keyword::VLine => translate_homo!(StatementArgument::Number, self.v_lines, to_int),
            Keyword::XPointsPerInch => translate_arg!(StatementArgument::Number, self.x_points_per_inch, to_float),
            Keyword::YPointsPerInch => translate_arg!(StatementArgument::Number, self.y_points_per_inch, to_float),
            Keyword::ZPointsPerInch => translate_arg!(StatementArgument::Number, self.z_points_per_inch, to_float),
            Keyword::XPointsPerMm => translate_arg!(StatementArgument::Number, self.x_points_per_mm, to_float),
            Keyword::YPointsPerMm => translate_arg!(StatementArgument::Number, self.y_points_per_mm, to_float),
            Keyword::ZPointsPerMm => translate_arg!(StatementArgument::Number, self.z_points_per_mm, to_float),
            Keyword::PointsPerGram => translate_arg!(StatementArgument::Number, self.points_per_gram, to_float),
            Keyword::PointsPerSecond => translate_arg!(StatementArgument::Number, self.points_per_second, to_float),
            Keyword::PenDown => {
                let coordinates = self.pen_statement_to_coords(&statement.arguments)?;
                self.current_component_set_builder = self.current_component_set_builder.pen_down(coordinates);
                Ok(self)
            }
            Keyword::PenUp => {
                let coordinates = self.pen_statement_to_coords(&statement.arguments)?;
                self.current_component_set_builder = self.current_component_set_builder.pen_up(coordinates);
                Ok(self)
            }
            Keyword::Dt => todo!(),
            Keyword::Date => Ok(self), // TODO Implement e_date
            Keyword::Style => translate_arg!(StatementArgument::Reserved, self.style, Style::try_from),
            Keyword::WriterId => translate_arg!(StatementArgument::String, self.writer_id, to_str),
            Keyword::Country => translate_arg!(StatementArgument::FreeText, self.country, to_str),
            Keyword::Hand => translate_arg!(StatementArgument::Reserved, self.hand, Hand::try_from),
            Keyword::Age => translate_arg!(StatementArgument::Number, self.age, to_int),
            Keyword::Sex => translate_arg!(StatementArgument::Reserved, self.sex, Sex::try_from),
            Keyword::Skill => translate_arg!(StatementArgument::Reserved, self.skill, Skill::try_from),
            Keyword::WriterInfo => translate_arg!(StatementArgument::FreeText, self.writer_info, to_str),
            Keyword::Segment => Ok(self),     // TODO Implement e_segment
            Keyword::StartSet => Ok(self),    // TODO Implement e_start_set
            Keyword::StartBox => Ok(self),    // TODO Implement e_start_box
            Keyword::RecSource => Ok(self),   // TODO Implement e_rec_source
            Keyword::RecId => Ok(self),       // TODO Implement e_rec_id
            Keyword::RecContact => Ok(self),  // TODO Implement e_rec_contact
            Keyword::RecInfo => Ok(self),     // TODO Implement e_rec_info
            Keyword::Implement => Ok(self),   // TODO Implement e_implement
            Keyword::TrainingSet => Ok(self), // TODO Implement e_training_set
            Keyword::TestSet => Ok(self),     // TODO Implement e_test_set
            Keyword::AdaptSet => Ok(self),    // TODO Implement e_adapt_set
            Keyword::LexiconSet => Ok(self),  // TODO Implement e_lexicon_set
            Keyword::RecTime => Ok(self),     // TODO Implement e_rec_time
            Keyword::RecLabels => Ok(self),   // TODO Implement e_rec_labels
            Keyword::RecScores => Ok(self),   // TODO Implement e_rec_scores
            Keyword::EndOfInput => {
                self.file_stack
                    .pop()
                    .ok_or(translation_err!("End of input without matching include"))?;
                Ok(self)
            }
        }
    }
}
