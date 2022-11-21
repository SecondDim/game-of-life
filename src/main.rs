use gol::{world, LineSeed, Pattern, PatternSeed};
use std::thread;
use std::time::Duration;

fn main() {
    // Enable backtraces unless a RUST_BACKTRACE value has already been explicitly provided.
    if std::env::var("RUST_BACKTRACE").is_err() {
        std::env::set_var("RUST_BACKTRACE", "1");
    }
    
    let mut world_width: usize = 0;
    let mut world_hight: usize = 0;

    let path = std::env::args().nth(1);

    let file = match path {
        Some(path) => std::path::PathBuf::from(path),
        None => std::path::PathBuf::from("default.ini"),
    };

    let content = std::fs::read_to_string(file).expect("could not read file");

    let vec: Vec<&str> = content.lines().collect();
    let mut pattern_vec: Vec<PatternSeed> = Vec::new();
    let mut line_vec: Vec<LineSeed> = Vec::new();

    for line in vec.iter() {
        let el: Vec<&str> = line.split(' ').collect();
        match el.first() {
            Some(pattern) => match pattern.to_lowercase().as_str() {
                "world" => {
                    println!("world width:{} hight:{}", el[1], el[2]);
                    world_width = el[1].parse::<usize>().unwrap_or(100);
                    world_hight = el[2].parse::<usize>().unwrap_or(100);
                }
                "block" => {
                    println!("add seed [{}] x:{} y:{}", el[0], el[1], el[2]);
                    pattern_vec.push(PatternSeed {
                        pattern: Pattern::Block,
                        x: el[1].parse::<usize>().unwrap_or(0),
                        y: el[2].parse::<usize>().unwrap_or(0),
                    });
                }
                "beehive" => {
                    println!("add seed [{}] x:{} y:{}", el[0], el[1], el[2]);
                    pattern_vec.push(PatternSeed {
                        pattern: Pattern::Beehive,
                        x: el[1].parse::<usize>().unwrap_or(0),
                        y: el[2].parse::<usize>().unwrap_or(0),
                    });
                }
                "loaf" => {
                    println!("add seed [{}] x:{} y:{}", el[0], el[1], el[2]);
                    pattern_vec.push(PatternSeed {
                        pattern: Pattern::Loaf,
                        x: el[1].parse::<usize>().unwrap_or(0),
                        y: el[2].parse::<usize>().unwrap_or(0),
                    });
                }
                "boat" => {
                    println!("add seed [{}] x:{} y:{}", el[0], el[1], el[2]);
                    pattern_vec.push(PatternSeed {
                        pattern: Pattern::Boat,
                        x: el[1].parse::<usize>().unwrap_or(0),
                        y: el[2].parse::<usize>().unwrap_or(0),
                    });
                }
                "tub" => {
                    println!("add seed [{}] x:{} y:{}", el[0], el[1], el[2]);
                    pattern_vec.push(PatternSeed {
                        pattern: Pattern::Tub,
                        x: el[1].parse::<usize>().unwrap_or(0),
                        y: el[2].parse::<usize>().unwrap_or(0),
                    });
                }
                "blinker" => {
                    println!("add seed [{}] x:{} y:{}", el[0], el[1], el[2]);
                    pattern_vec.push(PatternSeed {
                        pattern: Pattern::Blinker,
                        x: el[1].parse::<usize>().unwrap_or(0),
                        y: el[2].parse::<usize>().unwrap_or(0),
                    });
                }
                "toad" => {
                    println!("add seed [{}] x:{} y:{}", el[0], el[1], el[2]);
                    pattern_vec.push(PatternSeed {
                        pattern: Pattern::Toad,
                        x: el[1].parse::<usize>().unwrap_or(0),
                        y: el[2].parse::<usize>().unwrap_or(0),
                    });
                }
                "beacon" => {
                    println!("add seed [{}] x:{} y:{}", el[0], el[1], el[2]);
                    pattern_vec.push(PatternSeed {
                        pattern: Pattern::Beacon,
                        x: el[1].parse::<usize>().unwrap_or(0),
                        y: el[2].parse::<usize>().unwrap_or(0),
                    });
                }
                "pulsar" => {
                    println!("add seed [{}] x:{} y:{}", el[0], el[1], el[2]);
                    pattern_vec.push(PatternSeed {
                        pattern: Pattern::Pulsar,
                        x: el[1].parse::<usize>().unwrap_or(0),
                        y: el[2].parse::<usize>().unwrap_or(0),
                    });
                }
                "pentadecathlon" | "penta_decathlon" => {
                    println!("add seed [{}] x:{} y:{}", el[0], el[1], el[2]);
                    pattern_vec.push(PatternSeed {
                        pattern: Pattern::PentaDecathlon,
                        x: el[1].parse::<usize>().unwrap_or(0),
                        y: el[2].parse::<usize>().unwrap_or(0),
                    });
                }
                "glider" => {
                    println!("add seed [{}] x:{} y:{}", el[0], el[1], el[2]);
                    pattern_vec.push(PatternSeed {
                        pattern: Pattern::Glider,
                        x: el[1].parse::<usize>().unwrap_or(0),
                        y: el[2].parse::<usize>().unwrap_or(0),
                    });
                }
                "lightweightspaceship" | "light_weight_space_ship" | "lwss" => {
                    println!("add seed [{}] x:{} y:{}", el[0], el[1], el[2]);
                    pattern_vec.push(PatternSeed {
                        pattern: Pattern::LightWeightSpaceShip,
                        x: el[1].parse::<usize>().unwrap_or(0),
                        y: el[2].parse::<usize>().unwrap_or(0),
                    });
                }
                "middleweightspaceship" | "middle_weight_space_ship" | "mwss" => {
                    println!("add seed [{}] x:{} y:{}", el[0], el[1], el[2]);
                    pattern_vec.push(PatternSeed {
                        pattern: Pattern::MiddleWeightSpaceShip,
                        x: el[1].parse::<usize>().unwrap_or(0),
                        y: el[2].parse::<usize>().unwrap_or(0),
                    });
                }
                "heavyweightspaceship" | "heavy_weight_space_ship" | "hwss" => {
                    println!("add seed [{}] x:{} y:{}", el[0], el[1], el[2]);
                    pattern_vec.push(PatternSeed {
                        pattern: Pattern::HeavyWeightSpaceShip,
                        x: el[1].parse::<usize>().unwrap_or(0),
                        y: el[2].parse::<usize>().unwrap_or(0),
                    });
                }
                "vertical_line" | "verticalline" | "vertical" => {
                    println!(
                        "add line [{}] x:{} y:{} len: {}",
                        el[0], el[1], el[2], el[3]
                    );
                    line_vec.push(LineSeed {
                        vertical: true,
                        length: el[3].parse::<usize>().unwrap_or(0),
                        x: el[1].parse::<usize>().unwrap_or(0),
                        y: el[2].parse::<usize>().unwrap_or(0),
                    });
                }
                "horizontal_line" | "horizontalline" | "horizontal" => {
                    println!(
                        "add line [{}] x:{} y:{} len: {}",
                        el[0], el[1], el[2], el[3]
                    );
                    line_vec.push(LineSeed {
                        vertical: false,
                        length: el[3].parse::<usize>().unwrap_or(0),
                        x: el[1].parse::<usize>().unwrap_or(0),
                        y: el[2].parse::<usize>().unwrap_or(0),
                    });
                }
                _ => {}
            },
            None => todo!(),
        }
    }

    thread::sleep(Duration::from_secs(3));

    let mut board = world::init(world_width, world_hight);

    for p in pattern_vec {
        board.add_seed(p.pattern, p.x, p.y);
    }

    for p in line_vec {
        if p.vertical {
            board.add_vertical_line(p.x, p.y, p.length);
        } else {
            board.add_horizontal_line(p.x, p.y, p.length);
        }
    }

    loop {
        board.draw();

        board.next_tick();

        thread::sleep(Duration::from_millis(100));
    }
}
