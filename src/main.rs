use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{self, disable_raw_mode, enable_raw_mode, ClearType};
use ctrlc;
use prettytable::table;
use prettytable::{Cell, Row, Table};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io::stdout;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    if running.load(Ordering::SeqCst) {
        println!("What single, simple question are you trying to answer?");
        let mut mainq = String::new();
        std::io::stdin().read_line(&mut mainq).unwrap_or_default();

        /*
        println!("What further question would help break a tie?");
        let mut tiebreaker = String::new();
        std::io::stdin().read_line(&mut tiebreaker).unwrap_or_default();
        */
        println!("Enter items to rank separated by commas:");
        let mut items_input = String::new();
        std::io::stdin().read_line(&mut items_input).unwrap();
        if items_input.trim() == "" {
            println!("No items entered. Ciao.");
            std::process::exit(0);
        }
        let mut items: Vec<&str> = items_input.trim().split(',').collect();
        if items.len() == 1 {
            println!(
                "You entered one item: {}. Looks like an easy call!",
                items[0]
            );
            std::process::exit(0);
        }

        //println!("{}\n{}\n{:?}", mainq, tiebreaker, items);

        let mut rankings: Vec<(&str, i32)> = Vec::new();

        for i in &items {
            rankings.push((i, 0));
        }

        //this is a test
        let mut pairs = Vec::new();
        for i in 0..items.len() {
            for j in i + 1..items.len() {
                pairs.push((items[i], items[j]));
            }
        }
        let mut rng = thread_rng();
        pairs.shuffle(&mut rng);

        //println!("{:?}", rankings);
        //println!("{:?}", pairs);

        for (k, l) in pairs {
            //print!("\x1B[2J\x1B[1;1H");
            let _ = execute!(stdout(), terminal::Clear(ClearType::All));
            /*
            let box_k = format!(
                "+--------+\n\
                |   {}   |\n\
                +--------+\n\
                |   k   |\n\
                +-------+",
                k
            );

            let box_l = format!(
                "+--------+\n\
                |   {}   |\n\
                +--------+\n\
                |   l   |\n\
                +-------+",
                l
            );

            let lines_k: Vec<&str> = box_k.split('\n').collect();
            let lines_l: Vec<&str> = box_l.split('\n').collect();

            for (line_k, line_l) in lines_k.iter().zip(lines_l.iter()) {
                println!("{}   {}", line_k, line_l);
            }
            */
            println!("{}", mainq);
            let mut table = table!([k, l], ["press k", "press l"]);
            table.printstd();

            let mut choice = ' ';
            let _ = enable_raw_mode();
            loop {
                let event_result = read();
                match event_result {
                    Ok(event) => match event {
                        Event::Key(event) => match event.code {
                            KeyCode::Char('k') => {
                                choice = 'k';
                                break;
                            }
                            KeyCode::Char('l') => {
                                choice = 'l';
                                break;
                            }
                            KeyCode::Char('q') => {
                                println!("early quit...");
                                std::process::exit(0);
                            }
                            _ => (),
                        },
                        _ => (),
                    },
                    Err(e) => {
                        eprintln!("Failed to read event: {}", e);
                        continue;
                    }
                }
            }
            let _ = disable_raw_mode();

            match choice {
                'k' => {
                    for i in 0..rankings.len() {
                        if rankings[i].0 == k {
                            rankings[i].1 += 1;
                        }
                    }
                }
                'l' => {
                    for i in 0..rankings.len() {
                        if rankings[i].0 == l {
                            rankings[i].1 += 1;
                        }
                    }
                }
                _ => {
                    println!("Invalid choice. Please hit 'k' or 'l'. But also, you shouldn't see this message.");
                    continue;
                }
            }
        }
        let _ = execute!(stdout(), terminal::Clear(ClearType::All));
        println!("{}", mainq);
        rankings.sort_by(|a, b| b.1.cmp(&a.1));
        for (g, h) in rankings {
            println!("{}: {}", g, h);
        }
        println!(" ");
        println!("ciao.");
    }
}

/*
TODO:
- fix tiebreaker question - it should flash if a delay is detected
- add a way to store results in a file
-add a way to store inputs in a file so you can redo
- add a way to compare the two runs above
- play around with ratatui to make it look better
- find a way to implement ranked choice kind of - how to rank when you pull out the worst or best

-modularize it. need:
-input func for question and items
-function to generate shuffled pairs
-function to take preferences on all pairs and store that as a complete list
-function to take that list and generate scoring
-later, a function which can modify the complete results and sieve out irrelevant pairs
*/
