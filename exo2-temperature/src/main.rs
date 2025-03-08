use std::fmt::Display;

enum Category {
    Cold(f32),
    Cool(f32),
    Warm(f32)
}

struct AverageTemperature {
    category: Category,
    value: f32,
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let cat = match self {
            Self::Cold(_) => "Cold",
            Self::Cool(_) => "Cool",
            Self::Warm(_) => "Warm",
        };
        write!(f, "{} ", cat)
    }
}

impl Category {
    fn temperature(&self) -> f32 {
        match self {
            Self::Cold(t) | Self::Cool(t) | Self::Warm(t) => *t
        }
    }

    fn display(&self) {
        println!("Average temperature now is {}, {}", self, self.temperature());
    }
}

fn main() {
    let mut temperatures : [f32; 7] = [22.0, 19.5, 21.0, 23.5, 20.0, 18.0, 25.0];
    let average = calculate_average(&temperatures);
    println!("Average temperature is {}", average);

    let category = categorise_temperature(average);
    category.display();

    category.display();

    let average_temperature = AverageTemperature{
        category,
        value: average
    };

    temperatures.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("temperatures: {:?}", temperatures);
}

fn categorise_temperature(temp: f32) -> Category {
    let mut category = Category::Cool(temp);
    if temp < 10.0 {
        category = Category::Cold(temp);
    } else if temp > 20.0 {
        category = Category::Warm(temp);
    }

    category
}

fn calculate_average(temperatures: &[f32; 7]) -> f32 {
   temperatures.iter().sum::<f32>() / temperatures.len() as f32
}