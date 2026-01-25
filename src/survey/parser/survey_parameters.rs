use crate::{
    BackSightCorrectionFactors, CorrectionFactors, SurveyParameters,
    parser_utils::{parse_double, recognize_line},
    survey::{
        BearingUnits, InclinationUnits, LRUDAssociation, LengthUnits, PassageDimension,
        RedundantBackSight, ShotItem, SurveyFormat, SurveyFormat11, SurveyFormat12, SurveyFormat13,
        SurveyFormat15,
    },
};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, multispace0},
    combinator::value,
};

fn parse_bearing_units(input: &str) -> IResult<&str, BearingUnits> {
    alt((
        value(BearingUnits::Degrees, tag("D")),
        value(BearingUnits::Quads, tag("Q")),
        value(BearingUnits::Grads, tag("R")),
    ))
    .parse(input)
}

fn parse_length_units(input: &str) -> IResult<&str, LengthUnits> {
    alt((
        value(LengthUnits::DecimalFeet, tag("D")),
        value(LengthUnits::FeetAndInches, tag("I")),
        value(LengthUnits::Meters, tag("M")),
    ))
    .parse(input)
}

fn parse_inclination_units(input: &str) -> IResult<&str, InclinationUnits> {
    alt((
        value(InclinationUnits::Degrees, tag("D")),
        value(InclinationUnits::PercentGrade, tag("G")),
        value(InclinationUnits::DegreesAndMinutes, tag("M")),
        value(InclinationUnits::Grads, tag("R")),
        value(InclinationUnits::DepthGauge, tag("W")),
    ))
    .parse(input)
}

fn parse_passage_dimension(input: &str) -> IResult<&str, PassageDimension> {
    alt((
        value(PassageDimension::Up, tag("U")),
        value(PassageDimension::Down, tag("D")),
        value(PassageDimension::Right, tag("R")),
        value(PassageDimension::Left, tag("L")),
    ))
    .parse(input)
}

fn parse_shot_item(input: &str) -> IResult<&str, ShotItem> {
    alt((
        value(ShotItem::Length, tag("L")),
        value(ShotItem::Azimuth, tag("A")),
        value(ShotItem::Inclination, tag("D")),
        value(ShotItem::BackAzimuth, tag("a")),
        value(ShotItem::BackInclination, tag("d")),
    ))
    .parse(input)
}

fn parse_backsight(input: &str) -> IResult<&str, RedundantBackSight> {
    alt((
        value(RedundantBackSight::RedundantBacksight, tag("B")),
        value(RedundantBackSight::NoRedundantBacksight, tag("N")),
    ))
    .parse(input)
}

fn parse_lrud_association(input: &str) -> IResult<&str, LRUDAssociation> {
    alt((
        value(LRUDAssociation::FromStation, tag("F")),
        value(LRUDAssociation::ToStation, tag("T")),
    ))
    .parse(input)
}

fn parse_passage_dimension_order(input: &str) -> IResult<&str, [PassageDimension; 4]> {
    let (input, d0) = parse_passage_dimension(input)?;
    let (input, d1) = parse_passage_dimension(input)?;
    let (input, d2) = parse_passage_dimension(input)?;
    let (input, d3) = parse_passage_dimension(input)?;
    Ok((input, [d0, d1, d2, d3]))
}

fn parse_shot_item_order_3(input: &str) -> IResult<&str, [ShotItem; 3]> {
    let (input, s0) = parse_shot_item(input)?;
    let (input, s1) = parse_shot_item(input)?;
    let (input, s2) = parse_shot_item(input)?;
    Ok((input, [s0, s1, s2]))
}

fn parse_shot_item_order_5(input: &str) -> IResult<&str, [ShotItem; 5]> {
    let (input, s0) = parse_shot_item(input)?;
    let (input, s1) = parse_shot_item(input)?;
    let (input, s2) = parse_shot_item(input)?;
    let (input, s3) = parse_shot_item(input)?;
    let (input, s4) = parse_shot_item(input)?;
    Ok((input, [s0, s1, s2, s3, s4]))
}

fn parse_survey_format(input: &str) -> IResult<&str, SurveyFormat> {
    // If there's no format tag, then there's no format string
    let Ok((input, _)) = tag::<&str, &str, nom::error::Error<&str>>("FORMAT: ").parse(input) else {
        return Ok((input, SurveyFormat::None));
    };

    let format_len = input.len();

    let (input, bearing_units) = parse_bearing_units(input)?;
    let (input, length_units) = parse_length_units(input)?;
    let (input, passage_units) = parse_length_units(input)?;
    let (input, inclination_units) = parse_inclination_units(input)?;
    let (input, passage_dimension_order) = parse_passage_dimension_order(input)?;

    match format_len {
        11 => {
            let (input, shot_item_order) = parse_shot_item_order_3(input)?;
            Ok((
                input,
                SurveyFormat::Format11(SurveyFormat11 {
                    bearing_units,
                    length_units,
                    passage_units,
                    inclination_units,
                    passage_dimension_order,
                    shot_item_order,
                }),
            ))
        }
        12 => {
            let (input, shot_item_order) = parse_shot_item_order_3(input)?;
            let (input, backsight) = parse_backsight(input)?;
            Ok((
                input,
                SurveyFormat::Format12(SurveyFormat12 {
                    bearing_units,
                    length_units,
                    passage_units,
                    inclination_units,
                    passage_dimension_order,
                    shot_item_order,
                    backsight,
                }),
            ))
        }
        13 => {
            let (input, shot_item_order) = parse_shot_item_order_3(input)?;
            let (input, backsight) = parse_backsight(input)?;
            let (input, lrud_association) = parse_lrud_association(input)?;
            Ok((
                input,
                SurveyFormat::Format13(SurveyFormat13 {
                    bearing_units,
                    length_units,
                    passage_units,
                    inclination_units,
                    passage_dimension_order,
                    shot_item_order,
                    backsight,
                    lrud_association,
                }),
            ))
        }
        15 => {
            let (input, shot_item_order) = parse_shot_item_order_5(input)?;
            let (input, backsight) = parse_backsight(input)?;
            let (input, lrud_association) = parse_lrud_association(input)?;
            Ok((
                input,
                SurveyFormat::Format15(SurveyFormat15 {
                    bearing_units,
                    length_units,
                    passage_units,
                    inclination_units,
                    passage_dimension_order,
                    shot_item_order,
                    backsight,
                    lrud_association,
                }),
            ))
        }
        _ => Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::LengthValue,
        ))),
    }
}

fn parse_correction_factors(input: &str) -> IResult<&str, CorrectionFactors> {
    let (input, _) = tag("CORRECTIONS:")(input)?;
    let (input, azimuth) = parse_double(input)?;
    let (input, inclination) = parse_double(input)?;
    let (input, length) = parse_double(input)?;
    Ok((
        input,
        CorrectionFactors {
            azimuth,
            inclination,
            length,
        },
    ))
}

fn parse_backsight_correction_factors(input: &str) -> IResult<&str, BackSightCorrectionFactors> {
    let (input, _) = tag("CORRECTIONS2:")(input)?;
    let (input, azimuth) = parse_double(input)?;
    let (input, inclination) = parse_double(input)?;
    Ok((
        input,
        BackSightCorrectionFactors {
            azimuth,
            inclination,
        },
    ))
}

pub(super) fn parse_survey_parameters(input: &str) -> IResult<&str, SurveyParameters> {
    let (input, parameter_line) = recognize_line(input)?;
    let (parameter_line, _) = tag("DECLINATION:")(parameter_line)?;
    let (parameter_line, declination) = parse_double(parameter_line)?;
    let (parameter_line, format_parameters) = parse_survey_format(parameter_line)?;
    let (parameter_line, _) = alpha1(parameter_line)?;
    let (parameter_line, _) = multispace0(parameter_line)?;
    let correction_factor_result = parse_correction_factors(parameter_line);
    let (parameter_line, correction_factors) = match correction_factor_result {
        Ok((input, correction_factors)) => (input, Some(correction_factors)),
        Err(_) => (parameter_line, None),
    };
    let backsight_correction_factor_result = parse_backsight_correction_factors(parameter_line);
    let (_, backsight_correction_factors) = match backsight_correction_factor_result {
        Ok((input, backsight_correction_factors)) => (input, Some(backsight_correction_factors)),
        Err(_) => (parameter_line, None),
    };

    Ok((
        input,
        SurveyParameters {
            declination,
            format_parameters,
            correction_factors,
            backsight_correction_factors,
        },
    ))
}
