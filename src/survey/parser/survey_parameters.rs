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

fn parse_survey_format(input: &str) -> IResult<&str, SurveyFormat> {
    // If there's no format tag, then there's no format string
    let Ok((input, _)) = tag::<&str, &str, nom::error::Error<&str>>("FORMAT: ").parse(input) else {
        return Ok((input, SurveyFormat::None));
    };

    let format_len = input.len();

    let (input, bearing_units) = alt((
        value(BearingUnits::Degrees, tag("D")),
        value(BearingUnits::Quads, tag("Q")),
        value(BearingUnits::Grads, tag("R")),
    ))
    .parse(input)?;

    let (input, length_units) = alt((
        value(LengthUnits::DecimalFeet, tag("D")),
        value(LengthUnits::FeetAndInches, tag("I")),
        value(LengthUnits::Meters, tag("M")),
    ))
    .parse(input)?;
    let (input, passage_units) = alt((
        value(LengthUnits::DecimalFeet, tag("D")),
        value(LengthUnits::FeetAndInches, tag("I")),
        value(LengthUnits::Meters, tag("M")),
    ))
    .parse(input)?;
    let (input, inclination_units) = alt((
        value(InclinationUnits::Degrees, tag("D")),
        value(InclinationUnits::PercentGrade, tag("G")),
        value(InclinationUnits::DegreesAndMinutes, tag("M")),
        value(InclinationUnits::Grads, tag("R")),
        value(InclinationUnits::DepthGauge, tag("W")),
    ))
    .parse(input)?;
    let (input, passage_dimension_order_0) = alt((
        value(PassageDimension::Up, tag("U")),
        value(PassageDimension::Down, tag("D")),
        value(PassageDimension::Right, tag("R")),
        value(PassageDimension::Left, tag("L")),
    ))
    .parse(input)?;
    let (input, passage_dimension_order_1) = alt((
        value(PassageDimension::Up, tag("U")),
        value(PassageDimension::Down, tag("D")),
        value(PassageDimension::Right, tag("R")),
        value(PassageDimension::Left, tag("L")),
    ))
    .parse(input)?;
    let (input, passage_dimension_order_2) = alt((
        value(PassageDimension::Up, tag("U")),
        value(PassageDimension::Down, tag("D")),
        value(PassageDimension::Right, tag("R")),
        value(PassageDimension::Left, tag("L")),
    ))
    .parse(input)?;
    let (input, passage_dimension_order_3) = alt((
        value(PassageDimension::Up, tag("U")),
        value(PassageDimension::Down, tag("D")),
        value(PassageDimension::Right, tag("R")),
        value(PassageDimension::Left, tag("L")),
    ))
    .parse(input)?;

    let (input, shot_item_order_0) = alt((
        value(ShotItem::Length, tag("L")),
        value(ShotItem::Azimuth, tag("A")),
        value(ShotItem::Inclination, tag("D")),
        value(ShotItem::BackAzimuth, tag("a")),
        value(ShotItem::BackInclination, tag("d")),
    ))
    .parse(input)?;
    let (input, shot_item_order_1) = alt((
        value(ShotItem::Length, tag("L")),
        value(ShotItem::Azimuth, tag("A")),
        value(ShotItem::Inclination, tag("D")),
        value(ShotItem::BackAzimuth, tag("a")),
        value(ShotItem::BackInclination, tag("d")),
    ))
    .parse(input)?;

    let (mut input, shot_item_order_2) = alt((
        value(ShotItem::Length, tag("L")),
        value(ShotItem::Azimuth, tag("A")),
        value(ShotItem::Inclination, tag("D")),
        value(ShotItem::BackAzimuth, tag("a")),
        value(ShotItem::BackInclination, tag("d")),
    ))
    .parse(input)?;
    let mut shot_item_order_3 = ShotItem::BackAzimuth;
    let mut shot_item_order_4 = ShotItem::BackInclination;

    if format_len == 15 {
        let (remaining, sio3) = alt((
            value(ShotItem::Length, tag("L")),
            value(ShotItem::Azimuth, tag("A")),
            value(ShotItem::Inclination, tag("D")),
            value(ShotItem::BackAzimuth, tag("a")),
            value(ShotItem::BackInclination, tag("d")),
        ))
        .parse(input)?;

        let (remaining, sio4) = alt((
            value(ShotItem::Length, tag("L")),
            value(ShotItem::Azimuth, tag("A")),
            value(ShotItem::Inclination, tag("D")),
            value(ShotItem::BackAzimuth, tag("a")),
            value(ShotItem::BackInclination, tag("d")),
        ))
        .parse(remaining)?;
        shot_item_order_3 = sio3;
        shot_item_order_4 = sio4;
        input = remaining;
    }
    let mut backsight = RedundantBackSight::NoRedundantBacksight;
    if format_len >= 12 {
        let (intermediate, bs) = alt((
            value(RedundantBackSight::RedundantBacksight, tag("B")),
            value(RedundantBackSight::NoRedundantBacksight, tag("N")),
        ))
        .parse(input)?;
        backsight = bs;
        input = intermediate
    }
    let mut lrud_association = LRUDAssociation::ToStation;
    if format_len >= 13 {
        let (intermediate, assoc) = alt((
            value(LRUDAssociation::FromStation, tag("F")),
            value(LRUDAssociation::ToStation, tag("T")),
        ))
        .parse(input)?;
        lrud_association = assoc;
        input = intermediate;
    }
    match format_len {
        11 => Ok((
            input,
            SurveyFormat::Format11(SurveyFormat11 {
                bearing_units,
                length_units,
                passage_units,
                inclination_units,
                passage_dimension_order: [
                    passage_dimension_order_0,
                    passage_dimension_order_1,
                    passage_dimension_order_2,
                    passage_dimension_order_3,
                ],
                shot_item_order: [shot_item_order_0, shot_item_order_1, shot_item_order_2],
            }),
        )),
        12 => Ok((
            input,
            SurveyFormat::Format12(SurveyFormat12 {
                bearing_units,
                length_units,
                passage_units,
                inclination_units,
                passage_dimension_order: [
                    passage_dimension_order_0,
                    passage_dimension_order_1,
                    passage_dimension_order_2,
                    passage_dimension_order_3,
                ],
                shot_item_order: [shot_item_order_0, shot_item_order_1, shot_item_order_2],
                backsight,
            }),
        )),
        13 => Ok((
            input,
            SurveyFormat::Format13(SurveyFormat13 {
                bearing_units,
                length_units,
                passage_units,
                inclination_units,
                passage_dimension_order: [
                    passage_dimension_order_0,
                    passage_dimension_order_1,
                    passage_dimension_order_2,
                    passage_dimension_order_3,
                ],
                shot_item_order: [shot_item_order_0, shot_item_order_1, shot_item_order_2],
                backsight,
                lrud_association,
            }),
        )),
        15 => Ok((
            input,
            SurveyFormat::Format15(SurveyFormat15 {
                bearing_units,
                length_units,
                passage_units,
                inclination_units,
                passage_dimension_order: [
                    passage_dimension_order_0,
                    passage_dimension_order_1,
                    passage_dimension_order_2,
                    passage_dimension_order_3,
                ],
                shot_item_order: [
                    shot_item_order_0,
                    shot_item_order_1,
                    shot_item_order_2,
                    shot_item_order_3,
                    shot_item_order_4,
                ],
                backsight,
                lrud_association,
            }),
        )),
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
