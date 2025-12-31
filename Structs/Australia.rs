struct Australia {
    gdp: f32,
    inflation: f32,
    unemplyed: f32,
}

fn main() {
    let info = Australia {
        gdp: 3.44,
        inflation: 3.1416,
        unemplyed: 4.07,
    };

    println!(" Australia GDP {}%", info.gdp);
    println!("Inflation rate: {} %", info.inflation);
    println!("Unemplyed {} %", info.unemplyed);

    let Australia {
        gdp,
        inflation,
        unemplyed,
    } = info;
    println!("Summary : GDP is {} inflation is {}", gdp, inflation);
}
