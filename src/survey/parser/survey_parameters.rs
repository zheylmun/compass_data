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

    // Extract just the format string (alphabetic characters until whitespace or other delimiter)
    let (format_remaining, format_str) = alpha1(input)?;

    let format_len = format_str.len();

    let (_, bearing_units) = parse_bearing_units(format_str)?;
    let format_str = &format_str[1..];
    let (_, length_units) = parse_length_units(format_str)?;
    let format_str = &format_str[1..];
    let (_, passage_units) = parse_length_units(format_str)?;
    let format_str = &format_str[1..];
    let (_, inclination_units) = parse_inclination_units(format_str)?;
    let format_str = &format_str[1..];
    let (format_str, passage_dimension_order) = parse_passage_dimension_order(format_str)?;

    match format_len {
        11 => {
            let (_, shot_item_order) = parse_shot_item_order_3(format_str)?;
            Ok((
                format_remaining,
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
            let (format_str, shot_item_order) = parse_shot_item_order_3(format_str)?;
            let (_, backsight) = parse_backsight(format_str)?;
            Ok((
                format_remaining,
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
            let (format_str, shot_item_order) = parse_shot_item_order_3(format_str)?;
            let (format_str, backsight) = parse_backsight(format_str)?;
            let (_, lrud_association) = parse_lrud_association(format_str)?;
            Ok((
                format_remaining,
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
            let (format_str, shot_item_order) = parse_shot_item_order_5(format_str)?;
            let (format_str, backsight) = parse_backsight(format_str)?;
            let (_, lrud_association) = parse_lrud_association(format_str)?;
            Ok((
                format_remaining,
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
            format_remaining,
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

#[cfg(test)]
mod tests {
    use super::*;
    use float_eq::assert_float_eq;

    // Bearing units tests
    #[test]
    fn test_parse_bearing_units() {
        assert_eq!(parse_bearing_units("D").unwrap().1, BearingUnits::Degrees);
        assert_eq!(parse_bearing_units("Q").unwrap().1, BearingUnits::Quads);
        assert_eq!(parse_bearing_units("R").unwrap().1, BearingUnits::Grads);
        assert!(parse_bearing_units("X").is_err());
    }

    // Length units tests
    #[test]
    fn test_parse_length_units() {
        assert_eq!(parse_length_units("D").unwrap().1, LengthUnits::DecimalFeet);
        assert_eq!(parse_length_units("I").unwrap().1, LengthUnits::FeetAndInches);
        assert_eq!(parse_length_units("M").unwrap().1, LengthUnits::Meters);
        assert!(parse_length_units("X").is_err());
    }

    // Inclination units tests
    #[test]
    fn test_parse_inclination_units() {
        assert_eq!(parse_inclination_units("D").unwrap().1, InclinationUnits::Degrees);
        assert_eq!(parse_inclination_units("G").unwrap().1, InclinationUnits::PercentGrade);
        assert_eq!(parse_inclination_units("M").unwrap().1, InclinationUnits::DegreesAndMinutes);
        assert_eq!(parse_inclination_units("R").unwrap().1, InclinationUnits::Grads);
        assert_eq!(parse_inclination_units("W").unwrap().1, InclinationUnits::DepthGauge);
        assert!(parse_inclination_units("X").is_err());
    }

    // Passage dimension tests
    #[test]
    fn test_parse_passage_dimension() {
        assert_eq!(parse_passage_dimension("U").unwrap().1, PassageDimension::Up);
        assert_eq!(parse_passage_dimension("D").unwrap().1, PassageDimension::Down);
        assert_eq!(parse_passage_dimension("L").unwrap().1, PassageDimension::Left);
        assert_eq!(parse_passage_dimension("R").unwrap().1, PassageDimension::Right);
        assert!(parse_passage_dimension("X").is_err());
    }

    #[test]
    fn test_parse_passage_dimension_order() {
        let (remaining, order) = parse_passage_dimension_order("UDLR").unwrap();
        assert_eq!(remaining, "");
        assert_eq!(order[0], PassageDimension::Up);
        assert_eq!(order[1], PassageDimension::Down);
        assert_eq!(order[2], PassageDimension::Left);
        assert_eq!(order[3], PassageDimension::Right);

        // Different order
        let (_, order) = parse_passage_dimension_order("RLDU").unwrap();
        assert_eq!(order[0], PassageDimension::Right);
        assert_eq!(order[3], PassageDimension::Up);
    }

    // Shot item tests
    #[test]
    fn test_parse_shot_item() {
        assert_eq!(parse_shot_item("L").unwrap().1, ShotItem::Length);
        assert_eq!(parse_shot_item("A").unwrap().1, ShotItem::Azimuth);
        assert_eq!(parse_shot_item("D").unwrap().1, ShotItem::Inclination);
        assert_eq!(parse_shot_item("a").unwrap().1, ShotItem::BackAzimuth);
        assert_eq!(parse_shot_item("d").unwrap().1, ShotItem::BackInclination);
    }

    #[test]
    fn test_parse_shot_item_order_3() {
        let (remaining, order) = parse_shot_item_order_3("LAD").unwrap();
        assert_eq!(remaining, "");
        assert_eq!(order[0], ShotItem::Length);
        assert_eq!(order[1], ShotItem::Azimuth);
        assert_eq!(order[2], ShotItem::Inclination);
    }

    #[test]
    fn test_parse_shot_item_order_5() {
        let (remaining, order) = parse_shot_item_order_5("LADad").unwrap();
        assert_eq!(remaining, "");
        assert_eq!(order[0], ShotItem::Length);
        assert_eq!(order[4], ShotItem::BackInclination);
    }

    // Backsight and LRUD association tests
    #[test]
    fn test_parse_backsight() {
        assert_eq!(parse_backsight("B").unwrap().1, RedundantBackSight::RedundantBacksight);
        assert_eq!(parse_backsight("N").unwrap().1, RedundantBackSight::NoRedundantBacksight);
        assert!(parse_backsight("X").is_err());
    }

    #[test]
    fn test_parse_lrud_association() {
        assert_eq!(parse_lrud_association("F").unwrap().1, LRUDAssociation::FromStation);
        assert_eq!(parse_lrud_association("T").unwrap().1, LRUDAssociation::ToStation);
        assert!(parse_lrud_association("X").is_err());
    }

    // Format parsing tests
    #[test]
    fn test_parse_survey_format_none() {
        let (remaining, format) = parse_survey_format("no format here").unwrap();
        assert_eq!(remaining, "no format here");
        assert!(matches!(format, SurveyFormat::None));
    }

    #[test]
    fn test_parse_survey_format_11() {
        let (remaining, format) = parse_survey_format("FORMAT: DDDDUDLRLAD  rest").unwrap();
        assert_eq!(remaining, "  rest");
        match format {
            SurveyFormat::Format11(f) => {
                assert_eq!(f.bearing_units, BearingUnits::Degrees);
                assert_eq!(f.length_units, LengthUnits::DecimalFeet);
            }
            _ => panic!("Expected Format11"),
        }
    }

    #[test]
    fn test_parse_survey_format_12() {
        let (_, format) = parse_survey_format("FORMAT: DDDDUDLRLADN").unwrap();
        match format {
            SurveyFormat::Format12(f) => {
                assert_eq!(f.backsight, RedundantBackSight::NoRedundantBacksight);
            }
            _ => panic!("Expected Format12"),
        }

        let (_, format) = parse_survey_format("FORMAT: DDDDUDLRLADB").unwrap();
        match format {
            SurveyFormat::Format12(f) => {
                assert_eq!(f.backsight, RedundantBackSight::RedundantBacksight);
            }
            _ => panic!("Expected Format12"),
        }
    }

    #[test]
    fn test_parse_survey_format_13() {
        let (_, format) = parse_survey_format("FORMAT: DDDDUDLRLADNF").unwrap();
        match format {
            SurveyFormat::Format13(f) => {
                assert_eq!(f.lrud_association, LRUDAssociation::FromStation);
            }
            _ => panic!("Expected Format13"),
        }

        let (_, format) = parse_survey_format("FORMAT: DDDDUDLRLADNT").unwrap();
        match format {
            SurveyFormat::Format13(f) => {
                assert_eq!(f.lrud_association, LRUDAssociation::ToStation);
            }
            _ => panic!("Expected Format13"),
        }
    }

    #[test]
    fn test_parse_survey_format_15() {
        let (_, format) = parse_survey_format("FORMAT: DDDDUDLRLADadNF").unwrap();
        match format {
            SurveyFormat::Format15(f) => {
                assert_eq!(f.shot_item_order[3], ShotItem::BackAzimuth);
                assert_eq!(f.shot_item_order[4], ShotItem::BackInclination);
            }
            _ => panic!("Expected Format15"),
        }
    }

    #[test]
    fn test_parse_survey_format_invalid_length() {
        // 10 characters - not a valid format length
        let result = parse_survey_format("FORMAT: DDDDUDLRLA");
        assert!(result.is_err());
    }

    // Correction factors tests
    #[test]
    fn test_parse_correction_factors() {
        let (remaining, cf) = parse_correction_factors("CORRECTIONS:  1.5 -2.0 0.5  rest").unwrap();
        assert_eq!(remaining, "rest");
        assert_float_eq!(cf.azimuth, 1.5, abs <= 1e-10);
        assert_float_eq!(cf.inclination, -2.0, abs <= 1e-10);
        assert_float_eq!(cf.length, 0.5, abs <= 1e-10);
    }

    #[test]
    fn test_parse_correction_factors_zero() {
        let (_, cf) = parse_correction_factors("CORRECTIONS: 0.0 0.0 0.0").unwrap();
        assert_float_eq!(cf.azimuth, 0.0, abs <= 1e-10);
        assert_float_eq!(cf.inclination, 0.0, abs <= 1e-10);
        assert_float_eq!(cf.length, 0.0, abs <= 1e-10);
    }

    #[test]
    fn test_parse_backsight_correction_factors() {
        let (_, bcf) = parse_backsight_correction_factors("CORRECTIONS2: 1.0 -1.0").unwrap();
        assert_float_eq!(bcf.azimuth, 1.0, abs <= 1e-10);
        assert_float_eq!(bcf.inclination, -1.0, abs <= 1e-10);
    }

    // Full survey parameters tests
    #[test]
    fn test_parse_survey_parameters_minimal() {
        let input = "DECLINATION: 11.5\n";
        let (_, params) = parse_survey_parameters(input).unwrap();
        assert_float_eq!(params.declination, 11.5, abs <= 1e-10);
        assert!(matches!(params.format_parameters, SurveyFormat::None));
        assert!(params.correction_factors.is_none());
        assert!(params.backsight_correction_factors.is_none());
    }

    #[test]
    fn test_parse_survey_parameters_with_format() {
        let input = "DECLINATION: 11.18  FORMAT: DDDDUDLRLADN\n";
        let (_, params) = parse_survey_parameters(input).unwrap();
        assert_float_eq!(params.declination, 11.18, abs <= 1e-10);
        assert!(matches!(params.format_parameters, SurveyFormat::Format12(_)));
    }

    #[test]
    fn test_parse_survey_parameters_with_corrections() {
        let input = "DECLINATION: 11.18  FORMAT: DDDDUDLRLADN  CORRECTIONS: 1.0 2.0 3.0\n";
        let (_, params) = parse_survey_parameters(input).unwrap();
        let cf = params.correction_factors.unwrap();
        assert_float_eq!(cf.azimuth, 1.0, abs <= 1e-10);
        assert_float_eq!(cf.inclination, 2.0, abs <= 1e-10);
        assert_float_eq!(cf.length, 3.0, abs <= 1e-10);
    }

    #[test]
    fn test_parse_survey_parameters_with_backsight_corrections() {
        let input = "DECLINATION: 11.18  FORMAT: DDDDUDLRLADN  CORRECTIONS: 1.0 2.0 3.0  CORRECTIONS2: 0.5 -0.5\n";
        let (_, params) = parse_survey_parameters(input).unwrap();
        let bcf = params.backsight_correction_factors.unwrap();
        assert_float_eq!(bcf.azimuth, 0.5, abs <= 1e-10);
        assert_float_eq!(bcf.inclination, -0.5, abs <= 1e-10);
    }

    #[test]
    fn test_parse_survey_parameters_negative_declination() {
        let input = "DECLINATION: -5.0\n";
        let (_, params) = parse_survey_parameters(input).unwrap();
        assert_float_eq!(params.declination, -5.0, abs <= 1e-10);
    }

    #[test]
    fn test_parse_survey_parameters_different_units() {
        let input = "DECLINATION: 0.0  FORMAT: QMMDUDLRLAD\n";
        let (_, params) = parse_survey_parameters(input).unwrap();
        match params.format_parameters {
            SurveyFormat::Format11(f) => {
                assert_eq!(f.bearing_units, BearingUnits::Quads);
                assert_eq!(f.length_units, LengthUnits::Meters);
                assert_eq!(f.passage_units, LengthUnits::Meters);
            }
            _ => panic!("Expected Format11"),
        }
    }
}
