
//Now that hand and card structs are in place,
//**fix the print both hands method so that it prints the pretty display**

/*How will it work?
Upon run, ask user if they are ready to play.
-> y, start game
-> n, quit game

**Game play loop**
Start by dealing a card to the house. Notify the user about this card.
**Deal loop**
Next, deal a card to the user.
Check for a bust. **
-> if there is a bust, notify the user they lost this hand.
    -> Ask if they would like to play again.
    -> If yes start from the beginning
-> if there is no bust, prompt if they would like to hit or stay
    ->if stay, move to house loop.
    ->if hit, restart deal loop.
**House loop**
Deal cards to the house until they are above a certain value.
if the house busts, the user wins
->prompt user if they would like to play again
    ->If y, restart game loop
    ->If n, quit application.
END OF APPLICATION
*/

/* What architecture will we need?
Cards can be represented as a number, suit pair. Predefined tuples or struct?
The deck will be an array wrapped in a module.
-> needs shuffle
-> dealFrom
-> display
Hands can just be tuples or arrays?
-> display
-> clear
*/
//REMEMBER: because of crazy rust things 49 == 1 for debug mode
    //50 == 2 for debug mode
use std::io;
use rand::{thread_rng, Rng};
use std::fs;
use std::collections::HashMap;
///Type alias refering to u8 for uniformity through program
type Cardvalue = u8;

///Card struct that holds a cards value as well as its suit as a static string literal
#[derive(Debug,Clone)]
struct Card{
    value: Cardvalue,
    name: &'static str,
    suit: &'static str,
}
impl Card{
    fn get_card_name(value:Cardvalue)-> &'static str{
        match value{
            1 => return "Ace",
            2 => return "2",
            3 => return "3",
            4 => return "4",
            5 => return "5",
            6 => return "6",
            7 => return "7",
            8 => return "8",
            9 => return "9",
            10 => return "10",
            11 => return "Jack",
            12 => return "Queen",
            13 => return "King",
            _=> return "Number too high",
        }
    }
    fn print_card(&self){
        println!("{} of {}", self.name, self.suit);
    }
}
///Hand struct that contains a vector of Card structs
#[derive(Debug,Clone)]
struct Hand{
    cards: Vec<Card>,
}
///Implementations for the Hand struct
impl Hand{
    ///Iterates through all cards in a Hand and returns the sum of the values.
    fn calculate_total(&self) -> Cardvalue{
        let mut sum:Cardvalue = 0;
        for x in self.cards.iter(){
            if x.name == "Ace" && sum + 11 <= 21{
                sum += 11;
            }else if x.name == "Ace" && sum + 11 >21{
                sum += x.value;
            }else{
                sum += x.value;
            }
        }
        return sum;
    }
    ///Takes a Card and adds it to the Hand's current vector of Cards.
    fn add_card(&mut self, card: Card){
        self.cards.push(card);
    }
    ///Prints information about the cards held in the Hand card by card.
    fn print_hand(&self){
        for (x,card) in self.cards.iter().enumerate(){
            println!("Card {}, {} of {}",x,card.value, card.suit);
        }
    }
    
}
///Retrieves an argument passed in at the command line to dictate the debug mode the program should be run in
/// 
/// 0 (or no input) means no debug messages will be printed
/// 1 means debug messages will be printed.
fn get_debug_mode()->u8{
    DEBUG_MODE.with(|value|{
        if !(value.as_ref() == None){
            return value.as_ref().unwrap().as_str().as_bytes()[0] - 48
        }else{
            return 0;
        }
    })
}
thread_local!(static DEBUG_MODE: Option<String> = std::env::args().skip(1).next(););
///Returns the value stored in a hashmap for a given key.
/// Hashmap must be set up to have keys as String objects and values as u8's
fn get_map_value(key: &str, map: &mut HashMap<String,u8>) ->u8{
    return map.get(&String::from(key)).copied().unwrap();
}

///Loads all the stats from stats.txt into two different Vectors
/// Produces data variable of type Vec<u8>
/// and
/// Produces data_desc variable of type Vec<String>
/// data -> Actual values of the stored stats
/// data_desc -> String to identify what type of value is being loaded.
fn load_stats() -> HashMap<String,u8>{
    //Pattern to use when splitting input
    let split_pattern:&str = ": ";
    //String contents of file
    let contents = fs::read_to_string("stats.txt").unwrap();
    //Vector to hold data values
    //let mut data:Vec<u8> = Vec::new();
    //Vector to hold which data field description
    //let mut data_desc:Vec<String> = Vec::new();
    //Loop to unpack contents line by line and populate values into data and fields into data_desc
    let mut map = HashMap::<String, u8>::new();
    for line in contents.lines(){
        let split_line: Vec<String> = line.to_string().split(split_pattern).map(String::from).collect::<Vec<String>>();
        //println!("{:?}","Hello there".find("ta"));
        //data_desc.push(split_line[0].clone());
        //data.push(split_line[1].parse().unwrap());
        map.insert(split_line[0].clone(),split_line[1].parse().unwrap());
    }
    map.clone()
}
///Acts as a driver that runs the game loop function to create an endless loop
/// 
/// Loops while bool variable replay is true. Replay will be set to false in other areas
/// of the program to signal the user wanting to quit.
/// 
/// Once replay is set to false, the loop exits and a leaving message is displayed.
fn main() {
    //DEBUG_MODE.with(||{println!("{}",)};
    let mut replay = true;
    //Exists for the lifetime of the program.
    //Pass this into gamebody.
    let mut loaded_values = load_stats();
    //println!("{:?}", loaded_values.entry("Player wins".to_string()).and_modify(|val| *val+=1).or_insert(0));
    while replay {
        game_body(&mut replay, &mut loaded_values);
    }
    println!("Thanks for playing!");
    
}
fn print_type_of<T>(_:&T){
    println!("{}",std::any::type_name::<T>());
}
///Retrieve input from the user after the program has started. This is called whenever input from the user is necessary.
/// 
/// Arguments:
/// * expected_answers: &Vec<char>. Vector of characters that take_input will accept as valid input.
/// * prompt: &str. Message to be displayed to the user each time input is attempted.
/// 
/// Returns:
/// * char. User's chosen option to be used by the calling function.
fn take_input(expected_answers: &Vec<char>,prompt:&str)->char{
    loop{
        //Declare new mutable string object for use in read_line
        let mut buffer = String::new();
        //Print the prompt message with expected inputs
        println!("{}, or (q) to quit.",prompt);
        //Assign user input to buffer string
        match io::stdin().read_line(&mut buffer){
            Ok(n) => n,
            Err(err) => panic!("{}", err),
        };
        //Trim whitespace and newlines from buffer
        buffer = buffer.trim().to_string();
        //extract the first character from user input
        if buffer.len() <=0{
            println!("Bad input recieved");
            continue;
        }
        let c: char = buffer.chars().collect::<Vec<char>>()[0];
        //Check user input against all expected inputs
        if c == 'q'{
            panic!("Thanks for playing!");
        }
        if expected_answers.contains(&c){
            //If input is good, return to calling function with user selection.
            return c;
        }else{
            //If input is bad, continue looping until good input.
            println!("Bad input recieved");
            continue;
        }//end of expected_answers if
    }//end of input loop
}//end of take_input

///Main gameplay loop that houses logic for the game.
/// 
/// Arguments:
/// * replay: &mut bool. Boolean value that reports wether or not the user would like to play again
/// once game hand is finished.
fn game_body(replay: &mut bool, stats: &mut HashMap<String,u8>){
    //initialize variables and such
    println!("Welcome to blackjack! Press (q) at any time to quit");
    
    take_input(&vec!['r'], "Press (r) to begin!");
    
    //Game starts, dealer gets 1 card, then user gets to take their entire turn.
    let mut dealer_hand: Hand = Hand{cards: Vec::<Card>::new()};
    let mut player_hand: Hand = Hand{cards: Vec::<Card>::new()};
    dealer_hand.add_card(get_random_card());
    println!("Dealing the dealers hand.");
    dealer_hand.print_hand();
    player_hand.add_card(get_random_card());
    player_hand.add_card(get_random_card());
    println!("Dealing your hand.");
    player_hand.print_hand();
    //Player control. Hit or stay
    loop{
        let answer: char = take_input(&vec!['h','s'], "Would you like to (h) hit or (s) stay?");
        if answer == 'h'{
            //hit
            let new_card = get_random_card();
            new_card.print_card();
            player_hand.add_card(new_card);
            player_hand.print_hand();
            //THIS IS WHERE PRINT BOTH HANDS GOES
            if player_hand.calculate_total() > 21 {
                println!("Bust!");
                *replay = play_again();
                return;
            }
        }else{
            //stay
            //Dealers turn. Implement "AI" for dealer turn.
            //For now just see who's total is bigger without being over.
            //Theres your winner.
            println!("Dealing card to the dealer.");
            let new_card = get_random_card();
            new_card.print_card();
            dealer_hand.add_card(new_card);
            let p_total:Cardvalue = player_hand.calculate_total();
            let d_total:Cardvalue = dealer_hand.calculate_total();
            if p_total > d_total{        
                
                player_win(&p_total, stats);
            }else if p_total == d_total {
                
                tie(&p_total, &d_total,stats);
            }else{
                
                dealer_win(&d_total,stats);
            }
            stats.entry(String::from("Total games")).and_modify(|val| *val+=1);
            *replay = play_again();
            return;
        }
    }
}
///Asks user if they would like to play again and returns true or false.
/// 
/// Returns:
/// * Boolean indicating whether or not to play another hand.
fn play_again() -> bool{
    let answer = take_input(&vec!['y','n'],"Play again? (y) or (n)");
    if answer == 'y'{
        true
    }else{
        false
    }
}
fn player_win(p_total:&u8,stats:&mut HashMap<String,u8>){
    println!("You win!");        
    stats.entry(String::from("Player wins")).and_modify(|val| *val+=1);
    if *p_total == 21{
        println!("\tWith a BlackJack!!");
        stats.entry(String::from("Player Blackjacks")).and_modify(|val| *val += 1);
    }
}
fn dealer_win(d_total:&u8,stats:&mut HashMap<String,u8>){
    println!("Dealer Wins!");
    stats.entry(String::from("Dealer wins")).and_modify(|val| *val+=1);
    if *d_total == 21{
        println!("\tWith a BlackJack!!");
        stats.entry(String::from("Dealer Blackjacks")).and_modify(|val| *val += 1);
    }
}
fn tie(p_total:&u8,d_total:&u8,stats:&mut HashMap<String,u8>){
    println!("Tie game!");
    stats.entry(String::from("Tie games")).and_modify(|val| *val+=1);
}

///Prints the contents of both hands given in a more readable and "game friendly" format.
///  **Currently being reworked to support Hand and Card structs.**
///
/// Arguments:
/// * player_hand: &Vec<Cardvalue> Players hand
/// * dealer_hand: &Vec<Cardvalue> Dealers hand
fn print_both_hands(player_hand: &Vec<Cardvalue>,dealer_hand: &Vec<Cardvalue>){
    //let player_iter = player_hand.iter();
    //let dealer_iter = dealer_hand.iter();
    let mut larger_hand = Vec::<u8>::new();
    let mut other_hand = Vec::<u8>::new();
    let mut size:usize = 0;
    let mut hand_info:Vec<String> = Vec::new();
    //Populate hand_info with player_hand first, then dealer_hand.
    for x in player_hand.iter(){
        hand_info.push(format!("{} of {}",x, "Spades"));
    }
    //We got um bois
    for y in hand_info.iter_mut(){
        println!("{}",y);
    }
    //I could turn hand into a struct that has a method that will return
    //an element at that index if it exists or return "" if not.
}

/*
    deal_card: returns a random number of type Cardvalue
*/
///Returns a new card instance with a random value and suit.
/// 
///Returns:
///* Card struct with random values.
fn deal_card() -> Card{
    //This will return a Card struct now that it will get from get_random_card INSTEAD of get_random_num
    return get_random_card();
}
///Generates a random number between 1 and a given upper bound.
/// 
///Arguments:
///* limit: u8. The upper bound (non-inclusive) used for number generation.  
fn get_random_num(limit: u8)->Cardvalue{
    let mut rng = thread_rng();
    // Exclusive range
    let n: Cardvalue = rng.gen_range(1..limit);
    if get_debug_mode() == 1{
        println!("Number generated: {}",n);
    }
    n
}

///Generates a Card with random values for attributes value, name, and suit.
///
///Returns:
///* Card struct with generated values.
fn get_random_card() -> Card{
    let all_suits:[&str;4] = ["Clubs","Spades","Hearts","Diamonds"];
        //Implement fn parse_value into card to handle reducing down 11,12,13 into 10s
    let this_value = get_random_num(13);
    let suit_num:usize = (get_random_num(4) - 1).into();
    let suit:&str = all_suits[suit_num];
    return Card{value:this_value,name:Card::get_card_name(this_value),suit:suit}
}

//TODO: REWORK QUIT(q in input and saying no to replay) TO EXIT SAFELY AND WRITE NEW VALUES TO stats.txt
//Simplest way is to recieve a 'q' input and return to (or call) the main function with a 
//new quit flag set to true.

//Add an option to look at stats after or before a game.
    //Maybe a function that returns the map as a string to double as both
    //the display and the writing string.
