use ndarray::{Array1,Array,Axis,concatenate};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    line = line.trim_end().to_string();

    let lantern_fish = line.split(",").map(|x| x.parse::<u32>().unwrap()).collect();
    let mut lantern_fish = Array1::from_vec(lantern_fish);

    let spawn_time = 7;
    let new_adult = spawn_time - 1;
    let newborn = spawn_time + 1;

    let num_days = 80;

    for _ in 0..num_days {
        let spawn_number = lantern_fish.mapv(|x| (x == 0) as u32).sum();
        lantern_fish.mapv_inplace(|x| if x == 0 { new_adult } else { x - 1 } );
        lantern_fish = concatenate![Axis(0), lantern_fish, Array::ones(spawn_number as usize) * newborn];
    }

    println!("{}", lantern_fish.dim());
    Ok(())
}
