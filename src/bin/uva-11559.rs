use std::io;
use std::str::FromStr;

fn read_stdin() -> Result<Vec<String>, io::Error> {
    let mut buf = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buf)?;
    Ok(buf.split_whitespace().map(|v| v.to_string()).collect())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut output_strings = vec![];

    while let [participants, budget, hotels, _weeks] = &read_stdin().unwrap()[..] {
        let participants = u32::from_str(participants)?;
        let budget = budget.parse()?;
        let hotels = hotels.parse()?;

        let mut costs = vec![];
        for _ in 0..hotels {
            // let hotel_price = u32::from_str(&read_stdin()?.pop().expect("wrong hotel price"))?;
            let hotel_price = read_stdin()?.pop().unwrap().parse::<u32>()?;

            let availability = read_stdin()?
                .into_iter()
                .map(|x| x.parse().unwrap_or(0))
                .max()
                .unwrap_or(0);

            // Too expensive anyway?
            if hotel_price * participants > budget {
                continue;
            }

            // Has it ever has enough room?
            if availability < participants {
                continue;
            }

            costs.push(participants * hotel_price);
        }

        output_strings.push(match costs.into_iter().max() {
            Some(v) => v.to_string(),
            None => "stay home".to_string(),
        });
    }

    for string in output_strings {
        println!("{}", string);
    }

    Ok(())
}
