use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    IResult, Parser,
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, dirs) = directions.parse(input).unwrap();

    // dbg!(dirs);
    let mut dial = 50;
    let mut counter = 0;

    for dir in dirs {
        let num = match dir {
            Direction::Left(i) => -i,
            Direction::Right(i) => i,
        };
        let (new_dial, additional_counter) =
            spin(dial, num);
        dial = new_dial;
        counter += additional_counter;
    }

    Ok(counter.to_string())
}

fn spin(dial: i32, num: i32) -> (i32, i32) {
    let dial_long = dial + num;
    let mut revolutions = (dial_long / 100).abs();
    let new_dial = dial_long.rem_euclid(100);

    if dial != 0 && dial_long <= 0 {
        revolutions += 1;
    }

    (new_dial, revolutions)
}

#[derive(Debug)]
enum Direction {
    Left(i32),
    Right(i32),
}

fn directions(
    input: &str,
) -> IResult<&str, Vec<Direction>> {
    separated_list1(line_ending, direction).parse(input)
}

fn direction(input: &str) -> IResult<&str, Direction> {
    let (input, dir) =
        alt((tag("L"), tag("R"))).parse(input)?;
    let (input, num) = complete::i32(input)?;

    let d = match dir {
        "L" => Direction::Left(num),
        "R" => Direction::Right(num),
        x => panic!("unknown {x}"),
    };

    Ok((input, d))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!("6", process(input)?);
        Ok(())
    }

    use rstest::rstest;

    #[rstest]
    #[case((20, 0), 50, -30)]
    #[case((0, 1), 55, -55)]
    #[case((32,1), 14, -82)]
    fn spin_test(
        #[case] expected: (i32, i32),
        #[case] starting: i32,
        #[case] rotation: i32,
    ) {
        assert_eq!(expected, spin(starting, rotation))
    }
}
