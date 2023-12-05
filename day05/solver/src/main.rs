mod lookup;
use lookup::*;
fn main() {
    let seeds = seeds();
    let mut min_loc = u64::MAX;
    for seed_range in seeds {
        for seed in seed_range {
            let soil = seed_to_soil(seed);
            let fert = soil_to_fertilizer(soil);
            let water = fertilizer_to_water(fert);
            let light = water_to_light(water);
            let temp = light_to_temperature(light);
            let humidity = temperature_to_humidity(temp);
            let loc = humidity_to_location(humidity);

            min_loc = min_loc.min(loc);
        }
    }

    println!("{min_loc}");
}
