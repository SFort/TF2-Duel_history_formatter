extern crate tabwriter;
use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{self, Read, Write},
};
use tabwriter::TabWriter;

fn main() {
    if let Some(input1) = env::args().nth(1) {
        match input1.as_ref() {
            "parse" => {
                if let Some(html_path) = env::args().nth(2) {
                    let duels = parse_html(html_path);
                    println!("{:?}", duels);
                }
            }
            html_path => {
                let get_input = |text: &str| -> String {
                    let mut command = String::new();
                    print!("{}>", text);
                    io::stdout().flush().unwrap();
                    match io::stdin().read_line(&mut command) {
                        Ok(0) => command = "exit".into(),
                        Ok(_) => {}
                        Err(e) => panic!(e),
                    }
                    command.trim().to_string()
                };
                let duels = parse_html(html_path.into());
                loop {
                    let comm = get_input("");
                    match comm.to_lowercase().as_ref() {
                        "exit" | "quit" => break,
                        "result" => {
                            let mut s_duels: Vec<(String, u32, u32, u32, String)> =
                                group_end(&duels);
                            match get_input("Sort by?\n\t(name, win, loss, other)")
                                .to_lowercase()
                                .as_ref()
                            {
                                "name" => s_duels.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap()),
                                "win" => s_duels.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap()),
                                "loss" => s_duels.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap()),
                                "other" => s_duels.sort_by(|a, b| b.3.partial_cmp(&a.3).unwrap()),
                                _ => {}
                            }
                            let mut str_output =
                                "Name\tVictory\tLoss\tOther\t\tProfile Link\n".to_string();
                            for k in s_duels {
                                str_output +=
                                    &format!("{}\t{}\t{}\t{}\t\t{}\n", k.0, k.1, k.2, k.3, k.4);
                            }

                            let mut tw = TabWriter::new(vec![]);
                            write!(&mut tw, "{}", str_output).unwrap();
                            tw.flush().unwrap();
                            println!("{}", String::from_utf8(tw.into_inner().unwrap()).unwrap());
                        }
                        "kd" => {
                            let s_duels = group_duels(&duels, false);
                            let mut duels: Vec<(String, u32, u32, f32, String)> = s_duels
                                .iter()
                                .map(|x| {
                                    (
                                        x.name.to_string(),
                                        x.score,
                                        x.score2,
                                        kd(x.score, x.score2),
                                        x.link.to_string(),
                                    )
                                })
                                .collect();
                            match get_input("Sort by?\n\t(name, kill, death, kd)")
                                .to_lowercase()
                                .as_ref()
                            {
                                "name" => duels.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap()),
                                "kill" => duels.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap()),
                                "death" => duels.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap()),
                                "kd" => duels.sort_by(|a, b| b.3.partial_cmp(&a.3).unwrap()),
                                _ => {}
                            }
                            let mut str_output =
                                "Name\tYou\tThem\tKD\t\tProfile Link\n".to_string();
                            for k in duels {
                                str_output +=
                                    &format!("{}\t{}\t{}\t{}\t\t{}\n", k.0, k.1, k.2, k.3, k.4);
                            }

                            let mut tw = TabWriter::new(vec![]);
                            write!(&mut tw, "{}", str_output).unwrap();
                            tw.flush().unwrap();
                            println!("{}", String::from_utf8(tw.into_inner().unwrap()).unwrap());
                            //TODO
                        }
                        _ => println!("Available options:\nkd\nresult\n"),
                        //Stuff
                    }
                }
            }
        }
    } else {
        println!("Program requires at last 1 argument");
    }

    fn parse_html(s_path: String) -> Vec<Duel> {
        let mut file = File::open(s_path).unwrap();
        let mut text_html = String::new();
        file.read_to_string(&mut text_html).unwrap();
        text_html = text_html
            .split_at(text_html.find("<tbody>").unwrap() + 7)
            .1
            .to_string();
        text_html = text_html
            .split_at(text_html.rfind("</tbody>").unwrap())
            .0
            .to_string();
        let mut text_html = text_html;
        let mut output: Vec<Duel> = Vec::new();
        loop {
            //I realise this looks bad but it cuts down on processing time as the the previous
            //alternative
            //Time value
            let (time, link, name, score, score2, end);
            text_html = match text_html.find("<td>") {
                Some(a) => text_html.split_at(a + 4).1.to_string(),
                None => break,
            };
            match text_html.find("</td>") {
                Some(a) => {
                    time = Some(text_html.clone().split_at(a).0.to_string());
                }
                None => break,
            };
            //TODO Oponent Name / Link
            text_html = match text_html.find("<td>") {
                Some(a) => text_html.split_at(a + 4).1.to_string(),
                None => break,
            };
            match text_html.find("</td>") {
                Some(a) => {
                    let usr = text_html.clone().split_at(a).0.to_string();
                    let (left, u_name) = usr.split_at(usr.rfind("\">").unwrap() + 2);
                    //TODO names are currently URL encoded
                    name = u_name.split_at(u_name.rfind("</a>").unwrap()).0.to_string();
                    let u_link = left.split_at(left.find("<a href=\"").unwrap() + 9).1;
                    link = u_link.split_at(u_link.find("\">").unwrap()).0.to_string();
                    //TODO set name, link
                }
                None => break,
            };
            //score
            text_html = match text_html.find("<td>") {
                Some(a) => text_html.split_at(a + 4).1.to_string(),
                None => break,
            };
            match text_html.find("</td>") {
                Some(a) => {
                    score = text_html.clone().split_at(a).0.parse::<u32>().unwrap();
                }
                None => break,
            };
            //score 2
            text_html = match text_html.find("<td>") {
                Some(a) => text_html.split_at(a + 4).1.to_string(),
                None => break,
            };
            match text_html.find("</td>") {
                Some(a) => {
                    score2 = text_html.clone().split_at(a).0.parse::<u32>().unwrap();
                }
                None => break,
            };
            //end
            text_html = match text_html.find("<td>") {
                Some(a) => text_html.split_at(a + 4).1.to_string(),
                None => break,
            };
            match text_html.find("</td>") {
                Some(a) => {
                    end = Some(text_html.clone().split_at(a).0.parse::<u8>().unwrap());
                }
                None => break,
            };
            output.push(Duel {
                time,
                link,
                name,
                score,
                score2,
                end,
            });
        }
        return output;
    }
    #[derive(Debug, Clone)]
    struct Duel {
        time: Option<String>,
        link: String,
        name: String, // Opponent Name
        score: u32,   //Player Score
        score2: u32,  //Opponent Score
        end: Option<u8>,
    }
    fn group_duels(duels: &Vec<Duel>, ended_games_only: bool) -> Vec<Duel> {
        let mut hash_duels: HashMap<(&String, &String), (u32, u32)> = HashMap::new();
        for duel in duels {
            if ended_games_only {
                if duel.end != Some(0) {
                    continue;
                }
            }
            if !hash_duels.contains_key(&(&duel.name, &duel.link)) {
                hash_duels.insert((&duel.name, &duel.link), (duel.score, duel.score2));
            } else {
                let (s1, s2) = hash_duels[&(&duel.name, &duel.link)];
                hash_duels.insert(
                    (&duel.name, &duel.link),
                    (duel.score + s1, duel.score2 + s2),
                );
            }
        }
        hash_duels
            .iter()
            .map(|(key, num)| Duel {
                name: key.0.to_string(),
                link: key.1.to_string(),
                score: num.0,
                score2: num.1,
                time: None,
                end: if ended_games_only { Some(0) } else { None },
            })
            .collect()
    }
    fn group_end(duels: &Vec<Duel>) -> Vec<(String, u32, u32, u32, String)> {
        let mut hash_duels: HashMap<(&String, &String), (u32, u32, u32)> = HashMap::new();
        for duel in duels {
            //TODO I dont know what the end values mean so anything not 0 just is sent under Other
            //not won or lost
            let (s0, s1, s2) = if duel.end == Some(0) {
                if duel.score > duel.score2 {
                    (1, 0, 0)
                } else {
                    (0, 1, 0)
                }
            } else {
                (0, 0, 1)
            };
            if !hash_duels.contains_key(&(&duel.name, &duel.link)) {
                hash_duels.insert((&duel.name, &duel.link), (s0, s1, s2));
            } else {
                let (sa0, sa1, sa2) = hash_duels[&(&duel.name, &duel.link)];
                hash_duels.insert((&duel.name, &duel.link), (s0 + sa0, s1 + sa1, s2 + sa2));
            }
        }
        hash_duels
            .iter()
            .map(|(key, num)| (key.0.to_string(), num.0, num.1, num.2, key.1.to_string()))
            .collect()
    }
    fn kd(score: u32, score2: u32) -> f32 {
        let (s, s2) = (score as f32, score2 as f32);
        if s == s2 {
            0.0
        } else {
            if s > s2 {
                if s2 == 0.0 {
                    s
                } else {
                    s / s2
                }
            } else {
                if s == 0.0 {
                    -s2
                } else {
                    -(s2 / s)
                }
            }
        }
    }
}
