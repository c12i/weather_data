use regex::Regex;
use std::fs;
use std::str::FromStr;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct WeatherData {
    time: String,
    temperature: f64,
    humidity: f64,
}

impl FromStr for WeatherData {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re_time = Regex::new(r"Time: ([^'$,]+)").unwrap();
        let re_temp = Regex::new(r"Temp: ([^'$,]+)").unwrap();
        let re_humidity = Regex::new(r"Humidity: ([^'$]+)").unwrap();

        let time = re_time.captures(s).unwrap().get(1).unwrap().as_str();
        let temperature = re_temp.captures(s).unwrap().get(1).unwrap().as_str();
        let humidity = re_humidity.captures(s).unwrap().get(1).unwrap().as_str();

        let data = WeatherData {
            time: time.to_string(),
            temperature: temperature.parse().unwrap(),
            humidity: humidity.parse().unwrap(),
        };

        Ok(data)
    }
}

fn main() {
    let res = fs::read_to_string("sample.txt").unwrap();
    let res: Vec<&str> = res.split("\n").collect();

    let mut parsed_weather_data: Vec<WeatherData> = Vec::new();

    // let's assume we are reading from a stream
    for line in res {
        let w: WeatherData = line.parse().unwrap();

        parsed_weather_data.push(w.clone());

        parsed_weather_data.sort_by(|a, b| {
            return a.temperature.partial_cmp(&b.temperature).unwrap();
        });

        if parsed_weather_data.len() > 2 {
            if w.temperature >= parsed_weather_data[(parsed_weather_data.len() - 2)].temperature
                && w.temperature < parsed_weather_data[(parsed_weather_data.len() - 1)].temperature
            {
                println!("Done");
                break;
            }
        }
    }

    println!("{:#?}", parsed_weather_data);
}
