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
                let mut items: Vec<&str> = vec![];
                let (mut basic_skills, mut skills, mut talents, mut points) = apply_homeworld(&mut attributes);
                apply_birthright(&mut attributes, &mut basic_skills, &mut skills, &mut talents, &mut points);
                apply_lure_of_the_void(&mut attributes, &mut basic_skills, &mut skills, &mut talents, &mut items, &mut points);
                println!("");
                println!("");
                for name in ATTRIBUTE_NAMES{
                    let val:&i32 = attributes.get(name).expect("An error occured");
                    println!("{}:{}", name, val);
                }
                let toughness_bonus = attributes.get("Toughness").expect("An error occured") / 10;
                println!("");
                println!("Starting Wounds : {}", 2 * toughness_bonus + &points[0]);
                println!("Fate Points: {}", &points[1]);
                println!("Insanity Points: {}", &points[2]);
                println!("Corruption Points: {}", &points[3]);
                println!("XP to spend: {}", &points[4]);
                println!("");
                println!("Basic Skills: {:?}", basic_skills);
                println!("Trained Skills: {:?}", skills);
                println!("Talents and Traits: {:?}", talents);
                println!("Items: {:?}", items);
                println!("");
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
    let mut swaps = 0;
    let mut roll = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
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

    loop {
        if swaps < 2 {
        println!("Swap two Attributes? You have {} swaps left", 2-swaps);
        println!("[1] Swap Attributes [Enter] Contine with current Attributes");
        let mut swap_input = String::new();
            io::stdin()
                .read_line(&mut swap_input)
                .expect("Failed to read the input!");


            match swap_input.trim() {
            "1" => {
                loop { 
                    {
                        println!("Choose the first Attribute and confirm with enter");
                        for i in 0..9 {
                        print!("[{}] {} ", i+1, ATTRIBUTE_NAMES[i]);
                        }
                        println!("");
                    }
                    let mut attribute_1 = String::new();

                    io::stdin()
                        .read_line(&mut attribute_1)
                        .expect("Failed to read line.");

                    let attribute_1: i32 = match attribute_1.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Please enter a number!");
                            continue;
                        }
                    };
                    if attribute_1 > 9 || attribute_1 < 1 {
                        println!("Please enter a valid number");
                        break;
                    }
                    println!("Choose the second Attribute and confirm with enter");
                    for i in 0..9 {
                        print!("[{}] {} ", i+1, ATTRIBUTE_NAMES[i]);
                    }
                    println!("");
                    let mut attribute_2 = String::new();

                    io::stdin()
                        .read_line(&mut attribute_2)
                        .expect("Failed to read line.");

                    let attribute_2: i32 = match attribute_2.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Please enter a number!");
                            continue;
                        }
                    };
                    if attribute_2 > 9 || attribute_2 < 1 {
                        println!("Please enter a valid number");
                        break;
                    }
                let index_1: usize = (attribute_1 - 1) as usize;
                let index_2: usize = (attribute_2 -1) as usize;
                roll.swap(index_1, index_2);
                swaps += 1;
                for r in 0..9 {
                    println!("{} was {}", ATTRIBUTE_NAMES[r], roll[r]);
                }
                break;
            }
            },
            _=> break,
            }
        } else {
            break;
        }
    }

    for i in 0..9 {
        attributes.insert(ATTRIBUTE_NAMES[i], 25 + roll[i]);
    }

    return attributes;
}

fn apply_homeworld(x: &mut HashMap<&str, i32>) -> (Vec<&'static str>, Vec<&'static str>, Vec<&'static str>, [i32; 5]) {
    let homeworlds = ["Death World", "Void Born", "Forge World", "Hive World", "Imperial World", "Noble Born"];
    let mut points = [0; 5];
    points[4] = 500;
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

fn apply_birthright(attributes: &mut HashMap<&str, i32>, basic_skills: &mut Vec<&'static str>, skills: &mut Vec<&'static str>, talents: &mut Vec<&'static str>, points: &mut [i32; 5]) {
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
}

fn apply_lure_of_the_void(attributes: &mut HashMap<&str, i32>, _basic_skills: &mut Vec<&'static str>, skills: &mut Vec<&'static str>, talents: &mut Vec<&'static str>,items: &mut Vec<&'static str>, points: &mut [i32; 5]) {
    let lures_of_the_void = ["Tainted", "Criminal", "Renegade", "Duty Bound", "Zealot", "Chosen by Destiny"];
    loop {
        {
            println!("Please choose a Lure of the Void:");
            for i in 0..6 {
                print!("[{}] {} ", i+1, &lures_of_the_void[i]);
            }
            println!("");
        }

        let mut input = String::new();
        io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read the input!");
        
        match input.trim() {
            "1" => {
                let options = vec!["Mutant", "Insane", "Deviant Philosophy"];
                loop {
                    let tainted_choice = chose_option(&options);
                    match tainted_choice.trim() {
                        "1" => {
                            let options_2 = vec!["Roll a random Mutation", "Chose a Mutation between 1-75"];
                            loop {
                                let choice_2 = chose_option(&options_2);
                                match choice_2.trim() {
                                    "1" => {
                                        println!("Your mutation roll is {}", rand::thread_rng().gen_range(0..=99));
                                        break;
                                    },
                                    "2" => {
                                        points[4] -= 200;
                                        println!("Choose a Mutation");
                                        break;
                                    },
                                    _=> invalid()
                                }
                            }
                            break;
                        },
                        "2" => {
                            let options_3 = vec!["Lose 3 Fellowship", "Lose 1 Fate point"];
                            loop {
                                let choice = chose_option(&options_3);
                                match choice.trim() {
                                    "1" => {
                                        update_attribute(attributes, ATTRIBUTE_NAMES[8], -3);
                                        break;
                                    },
                                    "2" => {
                                        points[1] -= 1;
                                        break;
                                    },
                                    _=> invalid()
                                }
                            }
                            update_attribute(attributes, ATTRIBUTE_NAMES[3], 3);
                            talents.push("Peer (The Insane)");
                            points[2] += rand::thread_rng().gen_range(2..=20);
                            break;
                        },
                        "3" => {
                            update_attribute(attributes, ATTRIBUTE_NAMES[7], 3);
                            talents.push("Enemy (Ecclesiarchy)");
                            break;
                        }
                        _=> invalid()
                    }
                }
                break;
            },
            "2" => {
                let options = vec!["Wanted Fugitive", "Hunted", "Judged and Found Wanting"];
                loop {
                    let choice_criminal = chose_option(&options);
                    match choice_criminal.trim() {
                        "1" => {
                            talents.push("Enemy (Adeptus Arbites)");
                            talents.push("Peer (Underworld)");
                            break;
                        },
                        "2" => {
                            update_attribute(attributes, ATTRIBUTE_NAMES[6], 3);
                            talents.push("Enemy (Underworld)");
                            break;
                        },
                        "3" => {
                            let option_judged = vec!["gain poor-Craftsmanship bionic", "Spend 200 xp to upgrade to common-Craftsmanship", "Spend 300 xp to upgrade to good-Craftsmanship"];
                            loop {
                                let choice_judged = chose_option(&option_judged);
                                match choice_judged.trim() {
                                    "1" => {
                                        items.push("poor-Craftsmanship bionic");
                                        break;
                                    },
                                    "2" => {
                                        items.push("common-Craftsmanship bionic");
                                        points[4] -= 200;
                                        break;
                                    },
                                    "3" => {
                                        items.push("good-Craftsmanship bionic");
                                        points[4] -= 300;
                                        break;
                                    }
                                    _=> invalid()
                                }
                            }
                            break;
                        },
                        _=> invalid(),
                    }
                }
                break;
            },
            "3" => {
                let options = vec!["Recidivist", "Free-thinker", "Dark Visionary"];
                loop {
                    let choice_renegade = chose_option(&options);
                    match choice_renegade.trim() {
                        "1" => {
                            talents.push("Enemy (Adeptus Arbites)");
                            talents.push("Resistance (Interogation)");
                            skills.push("Concealment");
                            break;
                        },
                        "2" => {
                            chose_attribute(attributes, ATTRIBUTE_NAMES[5], ATTRIBUTE_NAMES[6], 3);
                            update_attribute(attributes, ATTRIBUTE_NAMES[7], -3);
                            talents.push("Enemy (Ecclisiarchy)");
                            break;
                        },
                        "3" => {
                            chose_points_specific(points, 2, 3, rand::thread_rng().gen_range(1..=5)+1);
                            talents.push("Dark Soul");
                            skills.push("Forbiden Lore (Chose One)");
                            break;
                        }
                        _=> invalid()
                    }
                }
                break;
            },
            "4" => {
                let options = vec!["Duty to the Throne", "Duty to Humanity", "Duty to Your Dynasty"];
                loop {
                    let choice_duty = chose_option(&options);
                    match choice_duty.trim() {
                        "1" => {
                            update_attribute(attributes, ATTRIBUTE_NAMES[7], 3);
                            let willpower = attributes.get(ATTRIBUTE_NAMES[7]).expect("An error occured");
                            if *willpower >= 40 {
                                talents.push("Armor of Contempt");
                            }
                            break;
                        },
                        "2" => {
                            chose_attribute(attributes, ATTRIBUTE_NAMES[6], ATTRIBUTE_NAMES[5], 3);
                            talents.push("-1 to the starting Profit Factor");
                            break;
                        },
                        "3" => {
                            talents.push("Rival (Rogue Trader Family)");
                            update_attribute(attributes, ATTRIBUTE_NAMES[3], -3);
                            talents.push("+1 to the starting Profit Factor");
                        }
                        _=> invalid(),
                    }
                }
                break;
            },
            "5" => {
                let options = vec!["Blessed Scars", "Unnerving Clarity", "Favoured of the Faithful"];
                loop {
                    let choice_zelot = chose_option(&options);
                    match choice_zelot.trim() {
                        "1" => {
                            loop {
                                let option_scar = vec!["gain poor-Craftsmanship bionic", "Spend 200 xp to upgrade to common-Craftsmanship", "Spend 300 xp to upgrade to good-Craftsmanship"];
                                let choice_scar = chose_option(&option_scar);
                                match choice_scar.trim() {
                                    "1" => {
                                        items.push("poor-Craftsmanship bionic");
                                        break;
                                    },
                                    "2" => {
                                        items.push("common-Craftsmanship bionic");
                                        points[4] -= 200;
                                        break;
                                    },
                                    "3" => {
                                        items.push("good-Craftsmanship bionic");
                                        points[4] -= 300;
                                        break;
                                    }
                                    _=> invalid()
                                }
                            }
                            break;
                        },
                        "2" => {
                            update_attribute(attributes, ATTRIBUTE_NAMES[7], 5);
                            let option_clarity = vec!["-5 Fellowship", "1d10 Insanity points"];
                            loop {
                                let choice_clarity = chose_option(&option_clarity);
                                match choice_clarity.trim() {
                                    "1" => {
                                        update_attribute(attributes, ATTRIBUTE_NAMES[8], -5);
                                        break;
                                    },
                                    "2" => {
                                        points[2] += rand::thread_rng().gen_range(1..=10);
                                        break;
                                    },
                                    _=> invalid(),
                                }
                            }
                            break;
                        },
                        "3" => {
                            update_attribute(attributes, ATTRIBUTE_NAMES[8], 5);
                            talents.push("Peer (Ecclisarchy)");
                            update_attribute(attributes, ATTRIBUTE_NAMES[3], -5);
                            break;
                        }
                        _=> invalid(),
                    }
                }
                break;
            },
            "6" => {
                let options = vec!["Seeker of Truth", "Xenophile", "Fated for Greatness"];
                loop {
                    let choice_destiny = chose_option(&options);
                    match choice_destiny.trim() {
                        "1" => {
                            talents.push("Foresight");
                            chose_talent(talents, "Enemy (Academics)", "Enemy (Ecclisiarchy)");
                            break;
                        },
                        "2" => {
                            talents.push("+10 to Fellowsip Tests with xenos");
                            talents.push("-5 penalty to Willpower tests involving xenos artifacts and powers");
                            break;
                        },
                        "3" => {
                            points[1] += 1;
                            points[2] += rand::thread_rng().gen_range(1..=10) + 1;
                            break;
                        },
                        _=> invalid(),
                    }
                }
                break;
            },
            _=> invalid()
        }
    }
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
        println!("Chose {} to [1] {} [2] {}", increase, option_1, option_2);
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

fn chose_points(points: &mut [i32; 5], index_1: usize, index_2: usize) {
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

fn chose_points_specific(points: &mut [i32; 5], index_1: usize, index_2: usize, increase: i32) {
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
                points[index_1] = temp + increase;
                break
            },
            "2" => {
                let temp = points[index_2];
                points[index_2] = temp + increase;
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

fn chose_option(options: &Vec<&str>) -> String {
    {
        println!("Please choose one of the options");
        for i in 0..options.len() {
            print!("[{}] {} ", i+1, &options[i]);
        }
        println!("");
    }

    let mut input = String::new();
    io::stdin()
                .read_line(&mut input)
                .expect("Failed to read the input!");
    input
}

fn invalid(){
    {
        println!("");
        println!("Please enter a valid input");
        println!("");
    }
}