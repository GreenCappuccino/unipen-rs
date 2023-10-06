use lazy_regex::regex;
use log::debug;
use pest::iterators::Pair;
use pest::Parser;
use std::path::Path;
use std::{fs, rc::Rc};

use crate::error::{translation_err, UniPenError};

#[derive(Parser)]
#[grammar = "statements.pest"]
struct StatementParser;

/// Parses the UniPen keyword statements from a file. If the file contains an include directive, the include directory must be provided. Recursively parses included files.
/// `.INCLUDE` statements are generated for each file path parsed. The data in the original `.INCLUDE` statement are not outputted.
///
/// # Arguments
///
/// * `path` - The path to the file to parse.
/// * `include` - The path to the include directory.
///
/// # Returns
///
/// The UniPen keyword statements parsed from the file.
///
/// # Errors
///
/// * `UniPenError::Io` - If an I/O error occurs while reading the file.
/// * `UniPenError::PestRule` - If the file does not conform to the grammar.
/// * `UniPenError::MissingInclude` - If the file contains an include directive, but no include directory was provided.
/// * `UniPenError::ParseInt` - If a number could not be parsed as an integer.
/// * `UniPenError::ParseFloat` - If a number could not be parsed as a float.
/// * `UniPenError::Translation` - If a translation error occurs.
///
pub fn parse(path: &Path, include: Option<&Path>) -> Result<Vec<Statement>, UniPenError> {
    debug!("Parsing statements from {:?}", path);
    let content = fs::read_to_string(path).map_err(UniPenError::Io)?;
    debug!("Finished reading {} bytes from {:?}", content.len(), path);
    let statement_pairs = StatementParser::parse(Rule::file, content.as_str())
        .map_err(|err| UniPenError::PestRule(Box::new(err.with_path(path.to_string_lossy().as_ref()))))?
        .next()
        .ok_or(translation_err!("Did not parser file"))?
        .into_inner();
    let mut statements = Vec::new();
    statements.push(Statement {
        keyword: Keyword::Include,
        arguments: vec![StatementArgument::String(path.to_string_lossy().into())],
    });
    for statement_pair in statement_pairs {
        match statement_pair.as_rule() {
            Rule::s_include => {
                let mut include_statements = parse(
                    &include
                        .ok_or(UniPenError::MissingInclude)?
                        .join(Path::new(parse_include_path(statement_pair)?)),
                    include,
                )?;
                statements.append(&mut include_statements);
            }
            _ => statements.push(Statement::try_from(statement_pair)?),
        }
    }
    debug!("Finished parsing {} statements from {:?}", statements.len(), path);
    Ok(statements)
}

fn parse_include_path(include_expression: Pair<Rule>) -> Result<&str, UniPenError> {
    match include_expression.as_rule() {
        Rule::s_include => Ok(include_expression
            .into_inner()
            .next()
            .ok_or(translation_err!("No include path in rule"))?
            .as_str()),
        _ => Err(translation_err!("Tried to convert a non-include rule to path")),
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub struct Statement {
    pub keyword: Keyword,
    pub arguments: Vec<StatementArgument>,
}

impl TryFrom<Pair<'_, Rule>> for Statement {
    type Error = UniPenError;

    fn try_from(value: Pair<Rule>) -> Result<Self, UniPenError> {
        Ok(Self {
            keyword: Keyword::try_from(value.as_rule())?,
            arguments: value
                .into_inner()
                .map(StatementArgument::try_from)
                .filter_map(std::result::Result::ok)
                .collect(),
        })
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub enum Keyword {
    Keyword,
    Reserve,
    Comment,
    Include,
    Version,
    DataSource,
    DataId,
    Coordinate,
    Hierarchy,
    DataContact,
    DataInfo,
    Setup,
    Pad,
    Alphabet,
    AlphabetFreq,
    LexiconSource,
    LexiconId,
    LexiconContact,
    LexiconInfo,
    Lexicon,
    LexiconFreq,
    XDimension,
    YDimension,
    HLine,
    VLine,
    XPointsPerInch,
    YPointsPerInch,
    ZPointsPerInch,
    XPointsPerMm,
    YPointsPerMm,
    ZPointsPerMm,
    PointsPerGram,
    PointsPerSecond,
    PenDown,
    PenUp,
    Dt,
    Date,
    Style,
    WriterId,
    Country,
    Hand,
    Age,
    Sex,
    Skill,
    WriterInfo,
    Segment,
    StartSet,
    StartBox,
    RecSource,
    RecId,
    RecContact,
    RecInfo,
    Implement,
    TrainingSet,
    TestSet,
    AdaptSet,
    LexiconSet,
    RecTime,
    RecLabels,
    RecScores,
    EndOfInput,
}

impl TryFrom<Rule> for Keyword {
    type Error = UniPenError;

    fn try_from(value: Rule) -> Result<Self, UniPenError> {
        match value {
            Rule::s_keyword => Ok(Self::Keyword),
            Rule::s_reserve => Ok(Self::Reserve),
            Rule::s_comment => Ok(Self::Comment),
            Rule::s_include => Ok(Self::Include),
            Rule::s_version => Ok(Self::Version),
            Rule::s_data_source => Ok(Self::DataSource),
            Rule::s_data_id => Ok(Self::DataId),
            Rule::s_coord => Ok(Self::Coordinate),
            Rule::s_hierarchy => Ok(Self::Hierarchy),
            Rule::s_data_contact => Ok(Self::DataContact),
            Rule::s_data_info => Ok(Self::DataInfo),
            Rule::s_setup => Ok(Self::Setup),
            Rule::s_pad => Ok(Self::Pad),
            Rule::s_alphabet => Ok(Self::Alphabet),
            Rule::s_alphabet_freq => Ok(Self::AlphabetFreq),
            Rule::s_lexicon_source => Ok(Self::LexiconSource),
            Rule::s_lexicon_id => Ok(Self::LexiconId),
            Rule::s_lexicon_contact => Ok(Self::LexiconContact),
            Rule::s_lexicon_info => Ok(Self::LexiconInfo),
            Rule::s_lexicon => Ok(Self::Lexicon),
            Rule::s_lexicon_freq => Ok(Self::LexiconFreq),
            Rule::s_x_dim => Ok(Self::XDimension),
            Rule::s_y_dim => Ok(Self::YDimension),
            Rule::s_h_line => Ok(Self::HLine),
            Rule::s_v_line => Ok(Self::VLine),
            Rule::s_x_points_per_inch => Ok(Self::XPointsPerInch),
            Rule::s_y_points_per_inch => Ok(Self::YPointsPerInch),
            Rule::s_z_points_per_inch => Ok(Self::ZPointsPerInch),
            Rule::s_x_points_per_mm => Ok(Self::XPointsPerMm),
            Rule::s_y_points_per_mm => Ok(Self::YPointsPerMm),
            Rule::s_z_points_per_mm => Ok(Self::ZPointsPerMm),
            Rule::s_points_per_gram => Ok(Self::PointsPerGram),
            Rule::s_points_per_second => Ok(Self::PointsPerSecond),
            Rule::s_pen_down => Ok(Self::PenDown),
            Rule::s_pen_up => Ok(Self::PenUp),
            Rule::s_dt => Ok(Self::Dt),
            Rule::s_date => Ok(Self::Date),
            Rule::s_style => Ok(Self::Style),
            Rule::s_writer_id => Ok(Self::WriterId),
            Rule::s_country => Ok(Self::Country),
            Rule::s_hand => Ok(Self::Hand),
            Rule::s_age => Ok(Self::Age),
            Rule::s_sex => Ok(Self::Sex),
            Rule::s_skill => Ok(Self::Skill),
            Rule::s_writer_info => Ok(Self::WriterInfo),
            Rule::s_segment => Ok(Self::Segment),
            Rule::s_start_set => Ok(Self::StartSet),
            Rule::s_start_box => Ok(Self::StartBox),
            Rule::s_rec_source => Ok(Self::RecSource),
            Rule::s_rec_id => Ok(Self::RecId),
            Rule::s_rec_contact => Ok(Self::RecContact),
            Rule::s_rec_info => Ok(Self::RecInfo),
            Rule::s_implement => Ok(Self::Implement),
            Rule::s_training_set => Ok(Self::TrainingSet),
            Rule::s_test_set => Ok(Self::TestSet),
            Rule::s_adapt_set => Ok(Self::AdaptSet),
            Rule::s_lexicon_set => Ok(Self::LexiconSet),
            Rule::s_rec_time => Ok(Self::RecTime),
            Rule::s_rec_labels => Ok(Self::RecLabels),
            Rule::s_rec_scores => Ok(Self::RecScores),
            Rule::EOI => Ok(Self::EndOfInput),
            _ => Err(translation_err!(
                "Tried to convert a non-keyword rule to keyword statement"
            )),
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub enum StatementArgument {
    Number(Number),
    String(Rc<str>),
    FreeText(Rc<str>),
    Reserved(Reserved),
    Label(Rc<str>),
    List(ComponentList),
}

impl TryFrom<Pair<'_, Rule>> for StatementArgument {
    type Error = UniPenError;

    fn try_from(value: Pair<Rule>) -> Result<Self, UniPenError> {
        match value.as_rule() {
            Rule::t_number => value
                .into_inner()
                .next()
                .ok_or(translation_err!("Number rule did not contain a number"))
                .and_then(|pair| Number::try_from(pair).map(StatementArgument::Number)),
            Rule::t_string => Ok(Self::String(value.as_str().into())),
            Rule::t_free_text => Ok(Self::FreeText(value.as_str().into())),
            Rule::t_label => {
                let value = value.as_str();
                let whitespace_regex = regex!(r"\s|\t|\r|\n");
                let escape_regex = regex!(r"\\(.)");
                let normalized = whitespace_regex.replace_all(value, " ");
                let escaped = escape_regex.replace_all(&normalized, |captures: &regex::Captures| -> String {
                    match captures.get(1).map(|m| m.as_str()) {
                        Some("n") => "\n".into(),
                        Some("t") => "\t".into(),
                        Some(c) => c.into(),
                        None => String::new(),
                    }
                });
                // Based on the grammar, the label should always be enclosed in quotes, so we can safely remove the first and last characters.
                Ok(Self::Label((&escaped[1..escaped.len() - 1]).into()))
            }
            Rule::r_list => Ok(Self::List(ComponentList::try_from(value)?)),
            rule => Ok(Self::Reserved(Reserved::try_from(rule)?)),
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub enum Reserved {
    Type,
    X,
    Y,
    Time,
    Pressure,
    Z,
    Button,
    Rho,
    Theta,
    Phi,
    LeftHand,
    RightHand,
    Male,
    Female,
    Bad,
    Ok,
    Good,
    Unknown,
    Printed,
    Cursive,
    Mixed,
    Accept,
    Reject,
}

impl TryFrom<Rule> for Reserved {
    type Error = UniPenError;

    fn try_from(value: Rule) -> Result<Self, UniPenError> {
        match value {
            Rule::r_type => Ok(Self::Type),
            Rule::r_x => Ok(Self::X),
            Rule::r_y => Ok(Self::Y),
            Rule::r_time => Ok(Self::Time),
            Rule::r_pressure => Ok(Self::Pressure),
            Rule::r_z => Ok(Self::Z),
            Rule::r_button => Ok(Self::Button),
            Rule::r_rho => Ok(Self::Rho),
            Rule::r_theta => Ok(Self::Theta),
            Rule::r_phi => Ok(Self::Phi),
            Rule::r_left_hand => Ok(Self::LeftHand),
            Rule::r_right_hand => Ok(Self::RightHand),
            Rule::r_male => Ok(Self::Male),
            Rule::r_female => Ok(Self::Female),
            Rule::r_bad => Ok(Self::Bad),
            Rule::r_ok => Ok(Self::Ok),
            Rule::r_good => Ok(Self::Good),
            Rule::r_unknown => Ok(Self::Unknown),
            Rule::r_printed => Ok(Self::Printed),
            Rule::r_cursive => Ok(Self::Cursive),
            Rule::r_mixed => Ok(Self::Mixed),
            Rule::r_accept => Ok(Self::Accept),
            Rule::r_reject => Ok(Self::Reject),
            _ => Err(translation_err!("No reserved rule")),
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub enum Number {
    Integer(i32),
    Decimal(f64),
}

impl From<&Number> for i32 {
    fn from(value: &Number) -> Self {
        match value {
            Number::Integer(n) => *n,
            #[allow(clippy::cast_possible_truncation)]
            Number::Decimal(n) => *n as Self,
        }
    }
}

impl From<&Number> for f64 {
    fn from(value: &Number) -> Self {
        match value {
            Number::Integer(n) => Self::from(*n),
            Number::Decimal(n) => *n,
        }
    }
}

impl TryFrom<Pair<'_, Rule>> for Number {
    type Error = UniPenError;

    fn try_from(value: Pair<Rule>) -> Result<Self, UniPenError> {
        match value.as_rule() {
            Rule::integer => value
                .as_str()
                .parse::<i32>()
                .map(Number::Integer)
                .map_err(UniPenError::ParseInt),
            Rule::decimal => value
                .as_str()
                .parse::<f64>()
                .map(Number::Decimal)
                .map_err(UniPenError::ParseFloat),
            _ => Err(translation_err!("Number rule did not contain an integer or decimal")),
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub struct ComponentList(pub Vec<ComponentItem>);

impl TryFrom<Pair<'_, Rule>> for ComponentList {
    type Error = UniPenError;

    fn try_from(value: Pair<Rule>) -> Result<Self, UniPenError> {
        Ok(Self(
            value
                .into_inner()
                .map(ComponentItem::try_from)
                .collect::<Result<Vec<ComponentItem>, _>>()?,
        ))
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub enum ComponentItem {
    Single(ComponentPoint),
    Range(ComponentRange),
}

impl TryFrom<Pair<'_, Rule>> for ComponentItem {
    type Error = UniPenError;

    fn try_from(value: Pair<'_, Rule>) -> Result<Self, UniPenError> {
        match value.as_rule() {
            Rule::component => Ok(Self::Single(ComponentPoint::try_from(value)?)),
            Rule::range => Ok(Self::Range(ComponentRange::try_from(value)?)),
            _ => Err(translation_err!(
                "Component item rule did not contain a component or range"
            )),
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub struct ComponentRange {
    pub start: ComponentPoint,
    pub end: ComponentPoint,
}

impl TryFrom<Pair<'_, Rule>> for ComponentRange {
    type Error = UniPenError;

    fn try_from(value: Pair<'_, Rule>) -> Result<Self, UniPenError> {
        let mut inner = value.into_inner();
        Ok(Self {
            start: ComponentPoint::try_from(
                inner
                    .next()
                    .ok_or(translation_err!("Component range rule did not contain a start"))?,
            )?,
            end: ComponentPoint::try_from(
                inner
                    .next()
                    .ok_or(translation_err!("Component range rule did not contain an end"))?,
            )?,
        })
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub struct ComponentPoint {
    pub component: usize,
    pub point: Point,
}

impl TryFrom<Pair<'_, Rule>> for ComponentPoint {
    type Error = UniPenError;

    fn try_from(value: Pair<'_, Rule>) -> Result<Self, UniPenError> {
        let mut inner = value.into_inner();
        let component = inner
            .next()
            .ok_or(translation_err!("Component point rule did not contain a component"))
            .and_then(|pair| pair.as_str().parse::<usize>().map_err(UniPenError::ParseInt))?;
        let point = match inner.next() {
            Some(n) => Point::Index(n.as_str().parse::<usize>().map_err(UniPenError::ParseInt)?),
            None => Point::All,
        };
        #[allow(clippy::cast_sign_loss)]
        Ok(Self { component, point })
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub enum Point {
    All,
    Index(usize),
}
