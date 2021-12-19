use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    line = line.trim_end().to_string();

    let lantern_fish_list: Vec<u64> = line.split(",").map(|x| x.parse::<u64>().unwrap()).collect();
    let mut lantern_fish: HashMap<u64,u64> = HashMap::new();
    for fish in lantern_fish_list {
        *lantern_fish.entry(fish).or_insert(0) += 1;
    }

    lantern_fish.insert(0, 0);
    lantern_fish.insert(6, 0);
    lantern_fish.insert(7, 0);
    lantern_fish.insert(8, 0);


    let num_days = 256;
    for _ in 0..num_days {
        let births = lantern_fish[&0];

        let mut lantern_fish_new = HashMap::new();
        for day in lantern_fish.keys() {
            match day {
                8 => lantern_fish_new.insert(*day, births),
                6 => lantern_fish_new.insert(*day, lantern_fish[&0] + lantern_fish[&7]),
                x => lantern_fish_new.insert(*x, lantern_fish[&(*x+1)]),
            };
        }
        lantern_fish = lantern_fish_new;

    }


    println!("{:?}", lantern_fish.values().sum::<u64>());
    Ok(())
}
