mod card;

// use std::collections::HashMap;
use std::io;
use card::create_card_info;
use card::Card;
use rand::Rng;
use rand::prelude::SliceRandom;
use ncurses::*;
use std::thread::sleep;
use std::time::Duration;



#[derive(Debug)]
struct Game {
    deck: Vec<Card>,
    field: Vec<Card>,
    u1_hand: Vec<Card>,
    u2_hand: Vec<Card>,
    u1_pts: Vec<Card>,
    u2_pts: Vec<Card>,
}


fn wait(seconds: u64) {
    sleep(Duration::from_secs(seconds));
}

// month to numb func
fn month_to_number(month: &str) -> u8 {
    match month.to_lowercase().as_str() {
        "jan" => 1,
        "feb" => 2,
        "mar" => 3,
        "apr" => 4,
        "may" => 5,
        "jun" => 6,
        "jul" => 7,
        "aug" => 8,
        "sep" => 9,
        "oct" => 10,
        "nov" => 11,
        "dec" => 12,
        _ => 0,
    }
}

fn number_to_month(i: u8) -> &'static str {
    match i {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "NA",
    }
}

/// Removes a card from the hand and shifts the remaining elements.
/// 
/// # Parameters
/// - `hand`: A mutable reference to a vector of `Card`s representing the hand.
fn remove_card_from_hand(hand: &mut Vec<Card>, card: &Card) {
    if let Some(pos) = hand.iter().position(|c| c == card) {
        hand.remove(pos);
    }
}

// Function to count cards on the field with the same month as the given card
fn count_same_month_cards(field: &Vec<Card>, card: &Card) -> usize {
    let mut matching: usize = 0;
    for (_index, neo_card) in field.iter().enumerate() {
        if neo_card.month == card.month {
            matching+=1;
        }
    }
    return matching;

}

// Prompt user to choose between matching month cards on the field
fn prompt_user_to_choose(cards: Vec<Card>) -> Card {
    let mut y_temp = 40;
    mvprintw(y_temp, 0, "Choose a card from the following options:");
    // println!("Choose a card from the following options:");
    for (index, card) in cards.iter().enumerate() {
        // println!("{}: {:?}", index + 1, card);
        let card_info = format!("{}: {:?}", index + 1, card);
        y_temp+=1;
        mvprintw(y_temp, 0, card_info.as_str());
    }

    loop {
        mvprintw(y_temp+1, 0, "\t\t\t\t\t\t");
        let input = get_user_input(y_temp+1,0,"Enter number:");

        // io::stdin().read_line(&mut input).expect("Failed to read line");
        if let Ok(choice) = input.trim().parse::<usize>() {
            if choice > 0 && choice <= cards.len() {
                return cards[choice - 1].clone();
            }
        }

        
        mvprintw(y_temp + 2, 0, &format!("Invalid choice, please enter a number between 1 and {}", cards.len()));
        // println!("Invalid choice, please enter a number between 1 and {}", cards.len());
    }
}

fn process_selected_card_user(card: Card, game: &mut Game) {
    let same_month_count = count_same_month_cards(&game.field, &card);
    // println!("Number of cards on the field with the same month: {}", same_month_count);

    if same_month_count == 3 {
        // Process all same month cards
        let same_month_cards: Vec<Card> = game.field.iter().filter(|c| c.month == card.month).cloned().collect();
        game.field.retain(|c| c.month != card.month);
        // remove_card_from_hand(&mut game.field,&chosen_card);
        wait(3);
        for c in same_month_cards {
            if c.point > 0 {
                game.u1_pts.push(c);
            }
        }
        if card.point > 0 {
            game.u1_pts.push(card.clone());
        }
        game.field.retain(|c| !(c.month == card.month && c.point == 0));
    } else if same_month_count == 2 {
        // Prompt user to choose
        let matching_cards: Vec<Card> = game.field.iter().filter(|c| c.month == card.month).cloned().collect();
        let chosen_card = prompt_user_to_choose(matching_cards);

        
        draw_board(game);
        

        overlay_card(&game.field, &chosen_card, &card);
        refresh();

        remove_card_from_hand(&mut game.field,&chosen_card);
        //Now overlay the matching card

        wait(3);



        //get rid of non point cards
        if card.point > 0 {
            game.u1_pts.push(card);
        }
        if chosen_card.point > 0 {
            game.u1_pts.push(chosen_card);
        }
    } else if same_month_count == 1 {
        // Process the cards
        let chosen_card = game.field.iter().find(|c| c.month == card.month).cloned().unwrap();
        // game.field.retain(|c| c != &chosen_card);
        
        draw_board(game);
        
        //calculate the x, y on the field to do the draw over on
        //y: 0 = even, 1 =  odd
        //x: x*10
        
        overlay_card(&game.field, &chosen_card, &card);
        refresh();
        remove_card_from_hand(&mut game.field,&chosen_card);

        wait(3);

        if card.point > 0 {
            game.u1_pts.push(card);
        }
        if chosen_card.point > 0 {
            game.u1_pts.push(chosen_card);
        }
    } else {
        // Add the card to field
        game.field.push(card);
    }
}

fn process_selected_card_cpu(card: Card, game: &mut Game) {
    let same_month_count = count_same_month_cards(&game.field, &card);
    // println!("Number of cards on the field with the same month: {}", same_month_count);

    if same_month_count == 3 {
        // Process all same month cards
        let same_month_cards: Vec<Card> = game.field.iter().filter(|c| c.month == card.month).cloned().collect();
        game.field.retain(|c| c.month != card.month);
        for c in same_month_cards {
            if c.point > 0 {
                game.u2_pts.push(c);
            }
        }
        if card.point > 0 {
            game.u2_pts.push(card.clone());
        }
        game.field.retain(|c| !(c.month == card.month && c.point == 0));
    } else if same_month_count == 2 {
        // Prompt user to choose
        let matching_cards: Vec<Card> = game.field.iter().filter(|c| c.month == card.month).cloned().collect();
        let chosen_card = matching_cards.choose(&mut rand::thread_rng()).unwrap().clone();


        overlay_card(&game.field, &chosen_card, &card);
        refresh();

        remove_card_from_hand(&mut game.field,&chosen_card);
        //Now overlay the matching card

        wait(3);

        if card.point > 0 {
            game.u2_pts.push(card);
        }
        if chosen_card.point > 0 {
            game.u2_pts.push(chosen_card);
        }
    } else if same_month_count == 1 {
        // Process the cards
        let chosen_card = game.field.iter().find(|c| c.month == card.month).cloned().unwrap();

        overlay_card(&game.field, &chosen_card, &card);
        refresh();

        remove_card_from_hand(&mut game.field,&chosen_card);
        //Now overlay the matching card

        wait(3);

        if card.point > 0 {
            game.u2_pts.push(card);
        }
        if chosen_card.point > 0 {
            game.u2_pts.push(chosen_card);
        }
    } else {
        // Add the card to field
        game.field.push(card);
    }
}

fn get_valid_card_input(hand: &Vec<Card>) -> Card {

    // mvprintw(40, 0, "Enter card in mon, val format: ");



    loop {
        mvprintw(40,0, "\t\t\t\t\t\t");
        let input = get_user_input(40,0,"Enter card in mon, val format:");
        let input = input.trim();

        let parts: Vec<&str> = input.split(',').collect();
        if parts.len() == 2 {
            let month = parts[0].trim();
            let value = parts[1].trim();

            let month_num = month_to_number(month);
            if month_num != 0 {
                if let Ok(value) = value.parse::<u8>() {
                    if let Some(card) = hand.iter().find(|card| card.month == month_num && card.point == value.into()) {
                        return card.clone();
                    } else {
                        mvprintw(41,0,"Card not found in your hand. Try again.                              ");
                    }
                } else {
                    mvprintw(41,0,"Invalid input format. Please enter in mon, val format.                              ");
                }
            } else {
                mvprintw(41,0,"Invalid month abbreviation. Please enter a valid 3-letter month abbreviation.         ");
            }
        } else {
            mvprintw(41,0,"Invalid input format. Please enter in mon, val format.                              ");
        }
    }
}




fn overlay_card(field: &Vec<Card>, bot_card: &Card, card: &Card) {
    if let Some(pos) = field.iter().position(|c| c == bot_card) {
        let half_size = (field.len() + 1) / 2;
        let (x, y) = if pos < half_size {
            (pos as i32 * 10, 0)
        } else {
            ((pos - half_size) as i32 * 10, 6)
        };
        draw_over_card(y +13, x , card);
    }
}




/// Takes a u8 option and converts it to int

fn option_to_int(opt: Option<u8>) -> i32 {
    match opt {
        Some(value) => value as i32,
        None => 0,
    }
}


/// Counts the total points in a players hand
/// 
/// # Parameters
/// - `hand`: The users hand to count
/// # Returns
/// - `u8`: the calculated score
fn count_pts(hand: &Vec<Card>) -> i32 {
    let mut score: i32 = 0;
    let mut yaku_sets: [i32; 12] = [0; 12];

    //count the points and add yaku count to yaku_sets
    for card in hand {
        score += card.point;
        for yaku in card.yaku {
            // println!("{:?}", yaku);
            let i = option_to_int(yaku);
            yaku_sets[i as usize] += 1;
        }
    }

    //Add the bonuses to the total score
    for v in yaku_sets {
        if v == 3 {
            score += 50;
        }
    }

    return score;
}

/// Print the score and who won
fn endgame_msg(u1: i32, u2: i32) {
    println!("Player score: {}\nCpu Score: {}", u1, u2);

    if  u1 > u2 {
        println!("User won!");
    } else if u2 > u1 {
        println!("CPU won!");
    } else {
        println!("Tie!")
    }
}

//Graphics stuff

/*
┌──────┐
│mon va│
│      │
│1    2│
└──────┘
x,y = 8,5

alt:
 ______ 
|mon va|
|      |
|1    2|
|______|

*/

//WARNING: ncurses uses the format (y,x) instead of (x,y)

///x,y of top left
fn draw_card(y: i32, x: i32, card: &Card){
    mvprintw(y, x, " ______ ");
    mvprintw(y+1, x, &format!("|{:<3} {:>2}|", number_to_month(card.month), card.point));
    mvprintw(y+2, x, "|      |");
    mvprintw(y+3, x, &format!("|{:^1}    {:^1}|", 
        card.yaku[0].map_or("".to_string(), |v| v.to_string()), 
        card.yaku[1].map_or("".to_string(), |v| v.to_string())
    ));
    mvprintw(y+4, x, "|______|");
}


fn draw_array(y: i32, x: i32, hand: &Vec<Card>){
    for (i, card) in hand.iter().enumerate() {
        let x1 = i as i32 * 10;
        draw_card(y,x + x1, card);
    }
}

fn draw_cpu_hand(y: i32, x: i32, hand: &Vec<Card>){
    for (i, _) in hand.iter().enumerate() {
        let x1 = i as i32 * 10;
        draw_blank(y,x + x1);
    }
}

fn draw_field(y: i32, x: i32, field: &Vec<Card>) {
    let size: usize = field.len();
    let half_size = (size + 1) / 2; // Calculate the half size, rounding up if odd

    for (i, card) in field.iter().enumerate() {
        if i < half_size {
            // Draw the top row
            draw_card(y, x + (i as i32 * 10), card);
        } else {
            // Draw the bottom row
            draw_card(y + 6, x + ((i - half_size) as i32 * 10), card);
        }
    }
}

fn draw_deck(y: i32, x: i32) {
    mvprintw(y,x," ______ ");
    mvprintw(y+1,x, "|      |\\");
    mvprintw(y+2,x, "|      ||");
    mvprintw(y+3,x, "|      ||");
    mvprintw(y+4,x, "|______||");
    mvprintw(y+5,x, " \\_____\\|");

}

fn draw_blank(y: i32,x: i32) {
    mvprintw(y, x, " ______ ");
    mvprintw(y+1, x, "|      |");
    mvprintw(y+2, x, "|      |");
    mvprintw(y+3, x, "|      |");
    mvprintw(y+4, x, "|______|");
}

fn get_user_input(y: i32, x: i32, prompt: &str) -> String {
    mvprintw(y, x, prompt);
    refresh();
    noecho();
    let mut input = String::new();
    loop {
        let ch = getch();
        match ch {
            10 => break, // Enter key
            127 | KEY_BACKSPACE => {
                if !input.is_empty() {
                    input.pop();
                    mv(y, x + prompt.len() as i32 + input.len() as i32);
                    addch(' ' as u32);
                    mv(y, x + prompt.len() as i32 + input.len() as i32);
                    refresh();
                }
            }
            _ => {
                if ch >= 32 && ch <= 126 {
                    input.push(ch as u8 as char);
                    addch(ch as u32);
                    refresh();
                }
            }
        }
    }
    input.trim().to_string()
}

fn draw_board(game: &Game) {
    clear();
    //Order: u2_pts -> blank u2_hand -> field -> u1 hand -> u1_pts
    //Give x+2 and y+2 spacing on the field
    mvprintw(0, 0, "CPU Points");
    draw_array(1, 0, &game.u2_pts);

    mvprintw(6, 0, "CPU Hand");
    draw_cpu_hand(7, 0, &game.u2_hand);

    // draw_array(14, 0, &game.field);
    draw_field(14, 0, &game.field);

    draw_deck(14, 80);

    mvprintw(28, 0, "Your hand");
    draw_array(29, 0, &game.u1_hand);

    mvprintw(34, 0, "Your Points");
    draw_array(35, 0, &game.u1_pts);

    refresh();
}

fn draw_over_card(y: i32, x: i32, card: &Card){
    //Draw and set the | properly, offset x+7, y+3
    draw_card(y+3, x+2, card);

    mvprintw(y+3, x+7, "|");
}

fn game() {
    let card_info = create_card_info();
    let mut rng = rand::thread_rng();

    let mut game = Game {
        deck: Vec::new(),
        field: Vec::new(),
        u1_hand: Vec::new(),
        u2_hand: Vec::new(),
        u1_pts: Vec::new(),
        u2_pts: Vec::new(),
    };


    for card in card_info.values() {
        game.deck.push(card.clone());
        if card.month != 11 {
            if card.point == 0 {
                let index = rng.gen_range(0..game.deck.len());
                game.deck.insert(index, card.clone());
            }
        } else {
            if card.point == 5 {
                let index = rng.gen_range(0..game.deck.len());
                game.deck.insert(index, card.clone());
            }
        }
    }

   
    // Draw 8 cards for the field
    for _ in 0..8 {
        if let Some(card) = game.deck.pop() {
            game.field.push(card);
        }
    }

    // Draw 8 cards for u1_hand and u2_hand alternately
    for _ in 0..8 {
        if let Some(card) = game.deck.pop() {
            game.u1_hand.push(card);
        }
        if let Some(card) = game.deck.pop() {
            game.u2_hand.push(card);
        }
    }

    //Initialize the curses library

    // Enable extended ASCII characters
    if has_colors() {
        start_color();
    }
    use_default_colors();
    // Enable special keys
    keypad(stdscr(), true);
    // Enable UTF-8 support
    setlocale(LcCategory::all, "");

    initscr();
    draw_board(&game);


    // Main game loop (let user go first)
    loop {
        // Game end condition, both u1_hand and u2_hand are empty
        if game.u1_hand.is_empty() && game.u2_hand.is_empty() {
            break;
        }
    
        // User 1 turn
        draw_board(&game);
        // println!("Enter card in mon, val format: ");
    
        let mut card = get_valid_card_input(&game.u1_hand);
        // println!("Valid card: {:?}", card);
        
        // wait(3);
        remove_card_from_hand(&mut game.u1_hand, &card);
        process_selected_card_user(card, &mut game);
        draw_board(&game);
    
        // Repeat the main function again, but use the top of the deck as the selected card
        card = game.deck.pop().expect("Deck is empty");
        
        //Draw the card here
        draw_card(20, 80, &card);
        refresh();
        wait(3);
        process_selected_card_user(card, &mut game);

        // wait(3);
        draw_board(&game);

        wait(3);
        
        // CPU Turn
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..game.u2_hand.len());
        card = game.u2_hand.remove(random_index);
        remove_card_from_hand(&mut game.u2_hand, &card);
        // println!("\n");
        // println!("CPU selected card: {:?}", card);
        // println!("\n");
        process_selected_card_cpu(card, &mut game);
        draw_board(&game);
        wait(3);

        // Draw the board again
        card = game.deck.pop().expect("Deck is empty");
        draw_card(20, 80, &card);
        refresh();
        wait(3);
        process_selected_card_cpu(card, &mut game);
        wait(3);

        draw_board(&game);
    }

    // Game over
    endwin();
    println!("Game over");
    endgame_msg(count_pts(&game.u1_pts), count_pts(&game.u2_pts));

}

// Main game handler
fn main() {
    // Welcome screen
    println!("Welcome to Hawaiian style Hanafuda\n");
    println!("See the read me for instructions\n");

    // TODO: uncomment when ready to release
    loop {
        println!("Game Menu:");
        println!("1. Start the game");
        println!("2. Quit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim().parse::<i32>() {
            Ok(1) => {
                println!("Starting the game...");
                game();
                break;
            }
            Ok(2) => {
                println!("Quitting the program...");
                break;
            }
            _ => {
                println!("Invalid choice, please enter 1 or 2.");
            }
        }
    }

    // game();
}