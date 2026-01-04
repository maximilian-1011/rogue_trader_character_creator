use std::io;
use std::collections::HashMap;

use rand::Rng;

static ATTRIBUTE_NAMES: [&str; 9] = ["Weapon Skill", "Ballistic Skill", "Strength", "Toughness", "Agility", "Intelligence", "Perception", "Willpower", "Fellowship"];
fn main() {
    loop {
        println!("What do you want to do?");
        println!("[1] Generate random Attributes? [2] End the program?");
        println!("Please type the coresponding number and press enter.");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the input!");
        

        match input.trim() {
            "1" => {
                let mut attributes = get_attributes();
                let (mut basic_skills, mut skills, mut talents, mut points) = apply_homeworld(&mut attributes);
                let (attributes, basic_skills, skills, talents, points) = apply_birthrite(&mut attributes, &mut basic_skills, &mut skills, &mut talents, &mut points);
                for name in ATTRIBUTE_NAMES{
                    let val:&i32 = attributes.get(name).expect("An error occured");
                    println!("{}:{}", name, val);
                }
                let toughness_bonus = attributes.get("Toughness").expect("An error occured") / 10;
                println!("Starting Wounds : {}", 2 * toughness_bonus + &points[0]);
                println!("Fate Points: {}", &points[1]);
                println!("Insanity Points: {}", &points[2]);
                println!("Corruption Points: {}", &points[3]);
                println!("Basic Skills: {:?}", basic_skills);
                println!("Trained Skills: {:?}", skills);
                println!("Talents and Traits: {:?}", talents);
                break;
            },
            "2" => return,
            _=> {
                println!("");
                println!("Please enter 1 or 2");
                println!("");
            },
        }
    }

    println!("Press enter to close!");
    let mut close = String::new();

    io::stdin()
        .read_line(&mut close)
        .expect("Failed to read input");

    match close {
        _ => return
    }
}

fn get_attributes() -> HashMap<&'static str, i32> {
    let mut attributes = HashMap::new();

    let mut roll = [0; 9];
    let mut total = 0;

    while total < 100 {
        total = 0;
        for r in 0..9 {
            roll[r] = rand::thread_rng().gen_range(2..=20);
            println!("The roll for {} was {}", ATTRIBUTE_NAMES[r], roll[r]);
            total += roll[r];
        }
        if total < 100 {
            println!("The total was {}... Do you want to re-roll?", total);
            println!("[1] To continue with the current roll, Press [Enter] to re-roll");

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read the input!");


            match input.trim() {
            "1" => break,
            _=> continue,
            }
        } else {
            println!("The total was {}", total);
        }
    }

    for i in 0..9 {
        attributes.insert(ATTRIBUTE_NAMES[i], 25 + roll[i]);
    }

    return attributes;
}

fn apply_homeworld(x: &mut HashMap<&str, i32>) -> (Vec<&'static str>, Vec<&'static str>, Vec<&'static str>, [i32; 4]) {
    let homeworlds = ["Death World", "Void Born", "Forge World", "Hive World", "Imperial World", "Noble Born"];
    let mut points = [0; 4];
    let result = loop {
        {
            println!("Please choose a homeworld:");
            for i in 0..6 {
                print!("[{}] {} ", i+1, &homeworlds[i]);
            }
            println!("");
        }

        let mut input = String::new();
        io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read the input!");

        match input.trim() {
            "1" => {
                update_attribute(x, ATTRIBUTE_NAMES[2], 5);
                update_attribute(x, ATTRIBUTE_NAMES[3], 5);
                update_attribute(x, ATTRIBUTE_NAMES[7], -5);
                update_attribute(x, ATTRIBUTE_NAMES[8], -5);
                let basic_skills = vec![];
                let skills = vec![];
                let talents = vec!["Melee Weapon Training (Primitive)", "Paranoid", "Survivor"];
                points[0] = rand::thread_rng().gen_range(1..=5) + 2;
                points[1] = {
                    let roll = rand::thread_rng().gen_range(1..=10);
                    if roll <= 5 {
                         2
                    } else {
                        3
                    }
                }; 
                break (basic_skills, skills, talents, points)
            },
            "2" => {
                update_attribute(x, ATTRIBUTE_NAMES[2], -5);
                update_attribute(x, ATTRIBUTE_NAMES[7], 5);
                let basic_skills = vec!["Navigation (Stellar)", "Pilot (Spacecraft)"];
                let skills = vec!["Speak Language (Ship Dialect)"];
                let talents = vec!["Charmed", "Ill-omened", "Shipwise", "Void Accustomed"];
                points[0] = rand::thread_rng().gen_range(1..=5);
                points[1] = {
                    let roll = rand::thread_rng().gen_range(1..=10);
                    if roll <= 5 {
                        3
                    } else {
                        4
                    }
                };
                break (basic_skills, skills, talents, points)
            },
            "3" => {
                update_attribute(x, ATTRIBUTE_NAMES[0], -5);
                update_attribute(x, ATTRIBUTE_NAMES[5], 5);
                let basic_skills = vec!["Common Lore (Tech)", "Common Lore (Machine Cult)"];
                let skills = vec![];
                let talents = vec!["Technical Knock", "Stranger to the Cult"];
                loop {
                    {
                        println!("Please choose an Attribute to increase by 3:");
                        for i in 0..9 {
                            print!("[{}] {} ", i+1, &ATTRIBUTE_NAMES[i]);
                        }
                        println!("");
                    }

                    let mut temp = String::new();
                    io::stdin()
                                .read_line(&mut temp)
                                .expect("Failed to read the input!");

                    match temp.trim() {
                        "1" => {
                            update_attribute(x, ATTRIBUTE_NAMES[0], 3);
                            break;
                        },
                        "2" => {
                            update_attribute(x, ATTRIBUTE_NAMES[1], 3);
                            break;
                        },
                        "3" => {
                            update_attribute(x, ATTRIBUTE_NAMES[2], 3);
                            break;
                        },
                        "4" => {
                            update_attribute(x, ATTRIBUTE_NAMES[3], 3);
                            break;
                        },
                        "5" => {
                            update_attribute(x, ATTRIBUTE_NAMES[4], 3);
                            break;
                        },
                        "6" => {
                            update_attribute(x, ATTRIBUTE_NAMES[5], 3);
                            break;
                        },
                        "7" => {
                            update_attribute(x, ATTRIBUTE_NAMES[6], 3);
                            break;
                        },
                        "8" => {
                            update_attribute(x, ATTRIBUTE_NAMES[7], 3);
                            break;
                        },
                        "9" => {
                            update_attribute(x, ATTRIBUTE_NAMES[8], 3);
                            break;
                        },
                        _=> {
                            println!("");
                            println!("Please enter a valid value");
                            println!("");
                        }
                    }
                };
                points[0] = rand::thread_rng().gen_range(1..=5) + 1;
                points[1] = {
                    let roll = rand::thread_rng().gen_range(1..=10);
                    if roll <= 5 {
                        2
                    } else if roll >= 6 && roll <= 9 {
                        3
                    } else {
                        4
                    }
                };
                break (basic_skills, skills, talents, points)
            },
            "4" => {
                update_attribute(x, ATTRIBUTE_NAMES[3], -5);
                update_attribute(x, ATTRIBUTE_NAMES[8], 5);
                let basic_skills = vec!["Speak Language (Hive Dialect)", "Tech-Use"];
                let skills = vec![];
                let talents = vec!["Accustomed to Crowds", "Hivebound", "Wary"];
                points[0] = rand::thread_rng().gen_range(1..=5) + 1;
                points[1] = {
                    let roll = rand::thread_rng().gen_range(1..=10);
                    if roll <= 5 {
                        2
                    } else if roll >= 6 && roll <= 8 {
                        3
                    } else {
                        4
                    }
                };
                break (basic_skills, skills, talents, points)
            },
            "5" => {
                update_attribute(x, ATTRIBUTE_NAMES[7], 3);
                let basic_skills = vec!["Common Lore (Imperium)", "Common Lore (Imperial Creed)", "Common Lore (War)", "Literacy", "Speak Language (High Gothic)"];
                let skills = vec![];
                let talents = vec!["Blessed Ignorance"];
                points[0] = rand::thread_rng().gen_range(1..=5);
                points[1] = {
                    let roll = rand::thread_rng().gen_range(1..=10);
                    if roll <= 8 {
                        3
                    } else {
                        4
                    }
                };
                break (basic_skills, skills, talents, points)
            },
            "6" => {
                update_attribute(x, ATTRIBUTE_NAMES[7], -5);
                update_attribute(x, ATTRIBUTE_NAMES[8], 5);
                let basic_skills = vec!["Literacy", "Speak Language (High Gothic)", "Speak Language (Low Gothic)"];
                let skills = vec![];
                let talents = vec!["Etiquette", "Legacy of Wealth", "Supremely Connected", "Vendetta"];
                points[0] = rand::thread_rng().gen_range(1..=5);
                points[1] = {
                    let roll = rand::thread_rng().gen_range(1..=10);
                    if roll <= 3 {
                        2
                    } else if roll >= 4 && roll <= 9 {
                        3
                    } else {
                        4
                    }
                };
                break (basic_skills, skills, talents, points)
            },
            _=> {
                println!("");
                println!("Please enter a valid value!");
                println!("");
            },
        }
    };

    result
}

fn apply_birthrite<'a>(attributes: &'a mut HashMap<&'a str, i32>, basic_skills: &'a mut Vec<&'static str>, skills: &'a mut Vec<&'static str>, talents: &'a mut Vec<&'static str>, points: &'a mut [i32; 4]) -> (&'a mut HashMap<&'a str, i32>, &'a mut Vec<&'static str>, &'a mut Vec<&'static str>, &'a mut Vec<&'static str>, &'a [i32; 4]) {
    let birthrights = ["Scavenger", "Scapegrace", "Stubjack", "Child of the Creed", "Savant", "Vaunted"];
    loop {
        {
            println!("Please choose a birthright:");
            for i in 0..6 {
                print!("[{}] {} ", i+1, &birthrights[i]);
            }
            println!("");
        }

        let mut input = String::new();
        io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read the input!");
        
        match input.trim() {
            "1" => {
                chose_talent(talents, "Unremarkable", "Resistance (Fear)");
                chose_attribute(attributes, ATTRIBUTE_NAMES[7], ATTRIBUTE_NAMES[4], 3);
                chose_points(points, 2, 3);
                break;
            },
            "2" => {
                basic_skills.push("Sleight of Hand");
                chose_attribute(attributes, ATTRIBUTE_NAMES[5], ATTRIBUTE_NAMES[6], 3);
                chose_points(points, 2, 3);
                break;
            },
            "3" => {
                talents.push("Quick Draw");
                skills.push("Intimidate");
                chose_attribute(attributes, ATTRIBUTE_NAMES[0], ATTRIBUTE_NAMES[1], 5);
                update_attribute(attributes, ATTRIBUTE_NAMES[8], -5);
                points[2] += rand::thread_rng().gen_range(1..=5);
                break;
            },
            "4" => {
                talents.push("Unshakable Faith");
                chose_attribute(attributes, ATTRIBUTE_NAMES[7], ATTRIBUTE_NAMES[8], 3);
                update_attribute(attributes, ATTRIBUTE_NAMES[0], -3);
                break;
            },
            "5" => {
                chose_talent_or_skill(talents, skills, "Peer (Academic)", "Logic");
                chose_attribute(attributes, ATTRIBUTE_NAMES[5], ATTRIBUTE_NAMES[8], 3);
                update_attribute(attributes, ATTRIBUTE_NAMES[3], -3);
                break;
            },
            "6" => {
                talents.push("Decadence");
                chose_attribute(attributes, ATTRIBUTE_NAMES[4], ATTRIBUTE_NAMES[8], 3);
                update_attribute(attributes, ATTRIBUTE_NAMES[6], -3);
                points[3] += rand::thread_rng().gen_range(1..=5);
                break;
            },
            _=> {
                println!("");
                println!("Please enter a valid input");
                println!("");
            }
        };

    };
    return (attributes, basic_skills, skills, talents, points)
}

fn chose_talent(talents: &mut Vec<&'static str>, option_1: &'static str, option_2: &'static str) {
    loop {
        println!("Chose either [1] {} [2] {}", option_1, option_2);
        let mut choice = String::new();
        io::stdin()
                    .read_line(&mut choice)
                    .expect("Failed to read input");
        match choice.trim() {
            "1" => {
                talents.push(option_1);
                break
            },
             "2" => {
                talents.push(option_2);
                break
            },
            _=> {
                println!("");
                println!("Please enter a valid input");
                println!("");
            }
        };
    };
}

fn chose_talent_or_skill(talents: &mut Vec<&'static str>, skills: &mut Vec<&'static str>, option_1: &'static str, option_2: &'static str) {
    loop {
        println!("Chose either [1] {} (Talent) [2] {} (Skill)", option_1, option_2);
        let mut choice = String::new();
        io::stdin()
                    .read_line(&mut choice)
                    .expect("Failed to read input");
        match choice.trim() {
            "1" => {
                talents.push(option_1);
                break
            },
             "2" => {
                skills.push(option_2);
                break
            },
            _=> {
                println!("");
                println!("Please enter a valid input");
                println!("");
            }
        };
    };
}

fn chose_attribute(attributes: &mut HashMap<&str, i32>, option_1: &'static str, option_2: &'static str, increase: i32) {
    loop {
        println!("Chose either [1] {} [2] {}", option_1, option_2);
        let mut choice = String::new();
        io::stdin()
                    .read_line(&mut choice)
                    .expect("Failed to read input");
        match choice.trim() {
            "1" => {
                update_attribute(attributes, option_1, increase);
                break
            },
             "2" => {
                update_attribute(attributes, option_2, increase);
                break
            },
            _=> {
                println!("");
                println!("Please enter a valid input");
                println!("");
            }
        };
    };
}

fn chose_points(points: &mut [i32; 4], index_1: usize, index_2: usize) {
    let point_names = ["Wounds", "Fate Points", "Insanity Points", "Corruption Points"];
    loop {
        println!("Gain [1] {} or [2] {}", point_names[index_1], point_names[index_2]);
        let mut choice = String::new();
        io::stdin()
                    .read_line(&mut choice)
                    .expect("Failed to read input");
        match choice.trim() {
            "1" => {
                let temp = points[index_1];
                points[index_1] = temp + rand::thread_rng().gen_range(1..=5);
                break
            },
            "2" => {
                let temp = points[index_2];
                points[index_2] = temp + rand::thread_rng().gen_range(1..=5);
                break
            },
            _=> {
                println!("");
                println!("Please enter a valid input");
                println!("");
            }
        };
    };
}

fn update_attribute(attributes: &mut HashMap<&str, i32>, name: &'static str, number: i32) {
    if let Some(temp) = attributes.get(name) {
        attributes.insert(name, temp + number);
    }
}