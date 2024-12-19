# Hawaiian Style Hanafuda

Hawaiian Style Hanafuda is a variation of the traditional Japanese Hanafuda card game. This project is implemented in Rust using the ncurses library for graphical display.


## Running the Game

To run the game, use the following command:

```sh
cargo run
```

## How to Play

The game consists of a deck of 48 cards, each representing a month of the year and having a specific point value and optional yaku (special combinations). The deck is shuffled, and cards are dealt to the field, player, and CPU hands.


[![Link to youtube explanation](https://img.youtube.com/vi/V2p0QFBpsF4/0.jpg)](https://www.youtube.com/watch?v=V2p0QFBpsF4)


#### Card Information

Each card has the following attributes:
- `month`: The month the card represents, being jan-dec. This is in the top left of the card.
- `point`: The point value of the card, these are ether: 0, 5, 10, 20. This is in the top right of the card.
- `yaku`: The bottom left and right numbers on the cards, these are the bonus combos called yakus.

![image](https://github.com/user-attachments/assets/c213356d-8abf-429e-8b04-1e4747427fc8)



### Game play

#### Parts of the feild
![Colord boxes](https://github.com/user-attachments/assets/719401be-69f2-44e2-b124-d5ce0ef827f6)

- Red is are the popints cards, these get couted at the end of the game to see who won
- Yellow are each players hand
- Green is the feild and all the cards on the feild
- Purple is where the card that is drawn from the deck is shown


1. **Initialization**:
   - The deck is shuffled.
   - 8 cards are dealt to the field.
   - 8 cards are dealt to the player's hand.
   - 8 cards are dealt to the CPU's hand.

2. **Player Turn**:
   - The player selects a card from their hand to play.
   - If there are 3 cards on the field with the same month as the selected card, all matching cards are collected.
   - If there are 2 cards on the field with the same month, the player is prompted to choose one of the matching cards.
   - If there is 1 card on the field with the same month, the matching card is collected.
   - If there are no matching cards, the selected card is added to the field.

3. **CPU Turn**:
   - The CPU selects a random card from its hand to play.
   - The same matching rules as the player's turn are applied.

4. **Drawing from Deck**:
   - After each turn, a card is drawn from the deck and played using the same matching rules.

5. **End of Game**:
   - The game ends when both the player's and CPU's hands are empty.
   - The scores are calculated based on the collected cards and yaku combinations.
   - The player with the higher score wins.

### Scoring

- Each card has a point value that contributes to the total score.
- Special yaku combinations provide bonus points.
- If a yaku combination is completed (3 cards with the same yaku), an additional 50 points are awarded. (see link below for sets)

### Controls

- Use the keyboard to enter the card you want to play in the format `mon, val` (e.g., `jan, 10`).
- Follow the on-screen prompts to make selections.

### Still lost?

* For further explanation of the game, refer to the wiki page for this, note that the game is played without the use of the "Lightning/Gaiji card" and that is instead a 0 point card. 
* Also the card sets and yaku combos are listed on the wiki page as well.

https://en.wikipedia.org/wiki/Sakura_(card_game)



## Data types

### `Card`
Defined in [card.rs](card.rs), the `Card` struct represents a single card in the game. Each card has the following attributes:
- `month`: The month the card represents (1-12). it uses the U8 datatype, a unsigned 8bit intiger.
- `point`: The point value of the card (0-20). It uses the i32 datatype, a 32 bit intiger, used to make it easer to caculate.
- `yaku`: An array of optional yaku values. Uses a U8 datatype to store what yaku set it is a part of (0-8).

### `Game`
Defined in [main.rs](main.rs), the `Game` struct holds the state of the game. It includes:
- `deck`: A vector of `Card`s representing the deck.
- `field`: A vector of `Card`s representing the cards on the field.
- `u1_hand`: A vector of `Card`s representing the player's hand.
- `u2_hand`: A vector of `Card`s representing the CPU's hand.
- `u1_pts`: A vector of `Card`s representing the player's collected cards.
- `u2_pts`: A vector of `Card`s representing the CPU's collected cards.

### `HashMap<u8, Card>`
Defined in [card.rs](card.rs), this hash map is used to store card information with the month as the key and the `Card` struct as the value. It is created using the `create_card_info` function.

### Functions
- `month_to_number` and `number_to_month`: Convert between month names and numbers.
- `remove_card_from_hand`: Removes a card from a hand.
- `count_same_month_cards`: Counts cards on the field with the same month as a given card.
- `prompt_user_to_choose`: Prompts the user to choose a card from a list.
- `process_selected_card_user` and `process_selected_card_cpu`: Process the selected card for the player and CPU, respectively.
- `get_valid_card_input`: Gets valid card input from the user.
- `overlay_card`: Overlays one card on top of another on the field.
- `option_to_int`: Converts an `Option<u8>` to an `i32`.
- `count_pts`: Counts the total points in a player's hand.
- `endgame_msg`: Prints the score and who won.
- `draw_card`, `draw_array`, `draw_cpu_hand`, `draw_field`, `draw_deck`, `draw_blank`, `draw_over_card`, `draw_board`: Functions for drawing the game elements using ncurses.
- `get_user_input`: Gets user input from the terminal.

### These data types and functions are used to manage the game state, handle user input, and render the game interface.


### `u8`
An unsigned 8-bit integer used to represent the month of a card and other small non-negative values. It is used because the month values range from 1 to 12, which fits within the range of `u8` (0 to 255).

### `i32`
A signed 32-bit integer used to represent the `point` value of a card and other numerical values that may be negative. It is used for `point` values because it provides a sufficient range for the game's scoring system.

### `Vec<T>`
A growable array type provided by Rust's standard library. It is used to store collections of `Card` structs for the deck, field, hands, and collected cards. `Vec` is chosen for its flexibility and dynamic resizing capabilities.

### `Array`
A static array type. It is used to store the `yaku` array due to yakus being best described using a 2 index array. The definition of `[x,y]` is used for storing values while `[expr; N]` is used when defining the array in the `Card` struct.


### `Option<T>`
A type that represents either a value or the absence of a value. It is used for the `yaku` attribute of a card to indicate whether a yaku is present. `Option` is used to handle cases where a yaku may or may not be assigned to a card.

### `String`
A growable, mutable string type provided by Rust's standard library. It is used for handling user input and displaying messages in the game. `String` is chosen for its ability to handle dynamic text content.


## Difficulties and solutions

When designing this game in rust, the original plan was to use a hashmap for the cards so that if I ever needed to index it, it would be easy. But since the hashmap stored the Card struct that stored all the card data, It was no loner needed. However it was a nice to have when shuffling the cards, as I did not need to do that when putting the cards into the dec vector. However I was wrong as I only assigned one card per unique combination of month, point, and yaku. Every month has one duplicate card in each, but since it is a hashmap, I can't double insert it as i/3 = insert twice. I need to check each one every time and need to randomly place it into the deck with getting a random index from the deck and inserting the duplicate there.

Rust data types can be at times a bit annoying> This game predominantly uses 2 different number data types, u8 and i32. To reduce the memory usage, the card point was stored as a u8 datatype. This worked most of the time, and when counting up all the points at the end, the variable used to store it was also u8. However since i8 only stores values from 0-255, the game crashed due to variable overflow when testing. Due to this, all u8 variables for points have been converted to i32 to not overflow and to make data conversion easy. 

The yaku array in the card struct needed something so that the yaku set could be stored, but not all cards have yakus, and some have 2. Because of this, the card itself needed something that could store "nothing" or or a number in a array with max 2 elements. The solution to this was making the 2 wide array have optional values that could be something `some` or nothing `none`. That greatly simplified writing the card hashmap as it became vary easy to read if a card was a part of a yaku set or not.


## The Good, Bad, and Ugly

### The Good

I always wanted to make this game as I couldn't find a computer version of this game that I liked and that used the Hawaiian style rules, so this game now exists, even if not in the format I want it to. Ideally, this project is a good stepping stone to learn about how to structure and make this game so when I come back to revisit it using a game engine, most of the logic kinks are worked out and this can be ported directly. 

Using the `_` to ignore unused return values when doing a for loop in rust is nice, I have only used it in Lua before and it keeps the unnecessary variable usage to a minimum.

### The Bad

Rust documentation, more specifically using the rust-lang website to find anything. The explanation is good for most stuff, but what is the hardest and most annoying is finding what I'm looking for. It was easer for me to look up what I needed on google search rather than rust's own documentation. But this is a problem in pretty much all documentation, so I learned to deal with it.

Side note, ncurses library uses the (y,x) format for coordinates, so I often had it swapped for using the (x,y) format. I get that using a curses library, the y axis is the most modified axis, but still annoys me that it uses this format.

### The ugly

Naming conventions are fine in most languages, as they support a wide variety of them. However, in rust, they only use snake_case for variables and this drives me insane. I used to camelCase, but in rust that produces a lot of warnings and it wants us to use snake_case to not have them, so it is just straight up a nuance at times with my naming conventions. But at least this preps me for dealing with annoying naming conventions when I have to work on someone else’s code.

Ncurses, not the library itself, but my implantation of the Gui for the game using it. I take for granted game engines that I typically use so I can just move a element around and everything updates. Doing it in ncurses, I need to constantly redraw the screen to update elements and it can get messy in code at times. There is probably a better way to do it, but the way I did it is kinda ugly.


## Learning experience

This project was for the most part fairly good, and made me appreciate some of the strictness of rust variables, what at first seemed bad. Coming from languages that are both loosely typed and not as loosely (like c++, java, and Lua), the thought of needing to even declare a variable as mutable `mut` to manipulate it was unheard of for me. Over the course of the lab, it became evident that this was good as it taught me that some variables used don’t need to be changed and others I thought that I would keep constant do end up changing. 

This project also was nice to look at how to learn a language to do a project in a quick amount of time. Most of my side projects I have 

Going into this project, I have heard some stuff about rust. Mainly that rust is a memory safe language, and that the community around it is a "gatekeeper" in regard to the knowledge around it and acting elitest. It may be true to some degree, but it was not a problem in my experience as forms such as stack overflow have vary useful responses to questions asked from the community, So the summery of this is not to judge a language by its stereotype.




