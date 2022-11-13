use rand::Rng;
use std::io;

fn main() {
    println!("<<<<<<<<Time to play Rusty Rock Paper Scissors!>>>>>>>>>>>>");
    println!("Commands: R -> Rock | P - Paper | S -> Scissors | Q -> Quit");

    let mut wins = 0;
    let mut losses = 0;
    let mut games = 0;

    loop {
        println!("Prepare your move, type: R || P || S ");
        let mut human_play = String::new();
        let compare_moves;
        io::stdin()
            .read_line(&mut human_play)
            .expect("Failed to read line");
        let human_play: String = human_play.trim().parse().expect("Please type a move retard!1!!!");
        match Some(human_play) {
            Some(x) if x == "R" => {
                compare_moves = 1;
                println!("You played Rock");
            }
            Some(x) if x == "P" => {
                compare_moves = 2;
                println!("You played Paper");
            }
            Some(x) if x == "S" => {
                compare_moves = 3;
                println!("You played Scissors");
            }
            Some(x) if x == "Q" => {
                println!("Quitter noob");
                return
            }
            None => continue,
            _ => {println!("Let's try something else shall we? (Type Q to Quit)"); continue},
        }
        
        let cpu_play = rand::thread_rng().gen_range(1..=3);
        match cpu_play {
            x if x == 1 => {
                println!("Cpu played Rock");
            }
            x if x == 2 => {
                println!("Cpu played Paper");
            }
            x if x == 3 => {
                println!("Cpu played Scissors");
            }
            _ => return,
        }
        match cpu_play {
            x if x==1 && compare_moves == 2 => {
                println!("Paper wraps Rock! ");
                println!("Player Wins!");
                wins+=1;
            }
            x if x==3 && compare_moves == 1=> {
                println!("Rock smashes Scissors!");
                println!("Player Wins!");
                wins+=1;
            }
            x if x == 2 && compare_moves == 3=> {
                println!("Scissors cut Paper!");
                println!("Player Wins!");
                wins+=1;
                
            }
            x if x==2 && compare_moves == 1 => {
                println!("Paper wraps Rock!");
                println!("CPU Wins!");
                losses += 1;
                
            }
            x if x==1 && compare_moves == 3=> {
                println!("Rock smashes Scissors!");
                println!("CPU Wins!");
                losses += 1;
            }
            x if x == 3 && compare_moves == 2=> {
                println!("Scissors cut Paper!");
                println!("CPU Wins!");
                losses += 1;
                
            }
            x if x == compare_moves => {
                println!("It's a draw!");
            }
            _ => return,
        }
        
        games += 1;
        println!("Player {wins} vs. CPU {losses} on {games} ROUNDS!");
    }
}
