use miette::miette;
use std::ops::RangeInclusive;

use nom::{
    bytes::complete::tag, character::complete,
    multi::separated_list1, sequence::separated_pair,
    IResult, Parser,
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, id_ranges) = ranges
        .parse(input)
        .map_err(|e| miette!("failed to parse {e}"))?;

    let total = id_ranges
        .into_iter()
        .map(|ids| {
            let mut total = 0;
            for id in ids.into_iter() {
                let id_str = id.to_string();
                let half = id_str.len() / 2;
                for limit in 0..half {
                    if id_str.len().rem_euclid(limit + 1)
                        == 0
                    {
                        let all_match = id_str[0..=limit]
                            .chars()
                            .cycle()
                            .zip(id_str.chars())
                            .all(|(a, b)| {
                                a == b
                            });
                        if all_match {
                            total += id;
                            break;
                        }
                    }
                }
            }
            total
        })
        .sum::<u64>();

    Ok(total.to_string())
}

fn ranges(
    input: &str,
) -> IResult<&str, Vec<RangeInclusive<u64>>> {
    separated_list1(
        tag(","),
        separated_pair(
            complete::u64,
            tag("-"),
            complete::u64,
        )
        .map(|(start, end)| start..=end),
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
";
        assert_eq!("4174379265", process(input)?);
        Ok(())
    }
}
