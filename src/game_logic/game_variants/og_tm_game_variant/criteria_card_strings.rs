// use crate::{add_criteria_card, game_logic::game_variants::criteria_boilerplate::CriteriaCard};

// pub fn criteria_card_strings() -> Vec<CriteriaCard> {
//     let mut vec_criteria_cards: Vec<CriteriaCard> = Vec::new();

//     add_criteria_card!(
//         vec_criteria_cards,
//         1,
//         "the Blue number compared to 1",
//         "Blue == 1;",
//         "Blue > 1;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         2,
//         "the Blue number compared to 3",
//         "Blue < 3;",
//         "Blue == 3;",
//         "Blue > 3;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         3,
//         "the Yellow number compared to 3",
//         "Yellow < 3;",
//         "Yellow == 3;",
//         "Yellow > 3;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         4,
//         "the Yellow number compared to 4",
//         "Yellow < 4;",
//         "Yellow == 4;",
//         "Yellow > 4;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         5,
//         "if Blue is even or odd",
//         "Blue is even;",
//         "Blue is odd;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         6,
//         "if Yellow is even or odd",
//         "Yellow is even;",
//         "Yellow is odd;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         7,
//         "if Purple is even or odd",
//         "Purple is even;",
//         "Purple is odd;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         8,
//         "the number of 1's in the code",
//         "zero 1's;",
//         "one 1;",
//         "two 1's;",
//         "three 1's;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         9,
//         "the number of 3's in the code",
//         "zero 3's;",
//         "one 3;",
//         "two 3's;",
//         "three 3's;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         10,
//         "the number of 4's in the code",
//         "zero 4's;",
//         "one 4;",
//         "two 4's;",
//         "three 4's;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         11,
//         "the Blue number compared to the Yellow number",
//         "Blue < Yellow;",
//         "Blue == Yellow;",
//         "Blue > Yellow;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         12,
//         "the Blue number compared to the Purple number",
//         "Blue < Purple;",
//         "Blue == Purple;",
//         "Blue > Purple;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         13,
//         "the Yellow number compared to the Purple number",
//         "Yellow < Purple;",
//         "Yellow == Purple;",
//         "Yellow > Purple;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         14,
//         "which color's number is smaller than either of the others",
//         "Blue < Yellow && Blue < Purple;",
//         "Yellow < Blue && Yellow < Purple;",
//         "Purple < Blue && Purple < Yellow;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         15,
//         "which color's number is larger than either of the others",
//         "Blue > Yellow && Blue > Purple;",
//         "Yellow > Blue && Yellow > Purple;",
//         "Purple > Blue && Purple > Yellow;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         16,
//         "the number of even numbers compared to the number of odd numbers",
//         "even > odd;",
//         "even < odd;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         17,
//         "how many even numbers there are in the code",
//         "zero even numbers;",
//         "one even number;",
//         "two even numbers;",
//         "three even numbers;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         18,
//         "if the sum of all the numbers is even or odd",
//         "Blue + Yellow + Purple == even;",
//         "Blue + Yellow + Purple == odd;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         19,
//         "the sum of Blue and Yellow compared to 6",
//         "Blue + Yellow < 6;",
//         "Blue + Yellow == 6;",
//         "Blue + Yellow > 6;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         20,
//         "if a number repeats itself in the code",
//         "a triple number;",
//         "a double number;",
//         "no repetition;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         21,
//         "if there is a number present exactly twice",
//         "no pairs || three of a kind;",
//         "a pair;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         22,
//         "if the three numbers in the code are in ascending order, descending order, or no order",
//         "ascending order;",
//         "descending order;",
//         "no order;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         23,
//         "the sum of all numbers compared to 6",
//         "Blue + Yellow + Purple < 6;",
//         "Blue + Yellow + Purple == 6;",
//         "Blue + Yellow + Purple < 6;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         24,
//         "if there is a sequence of ascending numbers",
//         "three numbers in ascending order;",
//         "two numbers in ascending order;",
//         "no numbers in ascending order;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         25,
//         "if there is a sequence of ascending or descending numbers",
//         "three numbers in ascending or descending order;",
//         "two numbers in ascending or descending order;",
//         "no numbers in ascending or descending order;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         26,
//         "that a specific color is less than 3",
//         "Blue < 3;",
//         "Yellow < 3;",
//         "Purple < 3;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         27,
//         "that a specific color is less than 4",
//         "Blue < 4;",
//         "Yellow < 4;",
//         "Purple < 4;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         28,
//         "that a specific color is equal to 1",
//         "Blue == 1;",
//         "Yellow == 1;",
//         "Purple == 1;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         29,
//         "that a specific color is equal to 3",
//         "Blue == 3;",
//         "Yellow == 3;",
//         "Purple == 3;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         30,
//         "that a specific color is equal to 4",
//         "Blue == 4;",
//         "Purple == 4;",
//         "Yellow == 4;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         31,
//         "that a specific color is greater than 1",
//         "Blue > 1;",
//         "Yellow > 1;",
//         "Purple > 1;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         32,
//         "that a specific color is greater than 3",
//         "Blue > 3;",
//         "Yellow > 3;",
//         "Purple > 3;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         33,
//         "that a specific color is even or odd",
//         "Blue is even;",
//         "Blue is odd;",
//         "Yellow is even;",
//         "Yellow is odd;",
//         "Purple is even;",
//         "Purple is odd;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         34,
//         "which color has the smallest number (or is tied for the smallest number)",
//         "Blue <= Yellow && Blue <= Purple;",
//         "Yellow <= Blue && Yellow <= Purple;",
//         "Purple <= Blue && Purple <= Yellow;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         35,
//         "which color has the largest number (or is tied for the largest number)",
//         "Blue >= Yellow && Blue >= Purple;",
//         "Yellow >= Blue && Yellow >= Purple;",
//         "Purple >= Blue && Purple >= Yellow;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         36,
//         "the sum of all the numbers is a multiple of 3 or 4 or 5",
//         "Blue + Yellow + Purple % 3 == 0;",
//         "Blue + Yellow + Purple % 4 == 0;",
//         "Blue + Yellow + Purple % 5 == 0;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         37,
//         "the sum of two specific colors is equal to 4",
//         "Blue + Yellow == 4;",
//         "Blue + Purple == 4;",
//         "Yellow + Purple == 4;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         38,
//         "the sum of two specific colors is equal to 6",
//         "Blue + Yellow == 6;",
//         "Blue + Purple == 6;",
//         "Yellow + Purple == 6;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         39,
//         "the number of one specific color compared to 1",
//         "Blue == 1;",
//         "Blue > 1;",
//         "Yellow == 1;",
//         "Yellow > 1;",
//         "Purple == 1;",
//         "Purple > 1;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         40,
//         "the number of one specific color compared to 3",
//         "Blue < 3;",
//         "Blue == 3;",
//         "Blue > 3;",
//         "Yellow < 3;",
//         "Yellow == 3;",
//         "Yellow > 3;",
//         "Purple < 3;",
//         "Purple == 3;",
//         "Purple > 3;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         41,
//         "the number of one specific color compared to 4",
//         "Blue < 4;",
//         "Blue == 4;",
//         "Blue > 4;",
//         "Yellow < 4;",
//         "Yellow == 4;",
//         "Yellow > 4;",
//         "Purple < 4;",
//         "Purple == 4;",
//         "Purple > 4;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         42,
//         "which color is the smallest or the largest",
//         "Blue < Yellow && Blue < Purple;",
//         "Blue > Yellow && Blue > Purple;",
//         "Yellow < Blue && Yellow < Purple;",
//         "Yellow > Blue && Yellow > Purple;",
//         "Purple < Blue && Purple < Yellow;",
//         "Purple > Blue && Purple > Yellow;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         43,
//         "the Blue number compared to the number of another specific color",
//         "Blue < Yellow;",
//         "Blue < Purple;",
//         "Blue == Yellow;",
//         "Blue == Purple;",
//         "Blue > Yellow;",
//         "Blue > Purple;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         44,
//         "the Yellow number compared to the number of another specific color",
//         "Yellow < Blue;",
//         "Yellow < Purple;",
//         "Yellow == Blue;",
//         "Yellow == Purple;",
//         "Yellow > Blue;",
//         "Yellow > Purple;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         45,
//         "how many 1's or how many 3's there are in the code",
//         "zero 1's;",
//         "zero 3's;",
//         "one 1;",
//         "one 3;",
//         "two 1's;",
//         "two 3's;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         46,
//         "how many 3's or how many 4's there are in the code",
//         "zero 3's;",
//         "zero 4's;",
//         "one 3;",
//         "one 4;",
//         "two 3's;",
//         "two 4's;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         47,
//         "how many 1's or how many 4's there are in the code",
//         "zero 1's;",
//         "zero 4's;",
//         "one 1;",
//         "one 4;",
//         "two 1's;",
//         "two 4's;"
//     );
//     add_criteria_card!(
//         vec_criteria_cards,
//         48,
//         "one specific color compared to another specific color",
//         "Blue < Yellow;",
//         "Blue == Yellow;",
//         "Blue > Yellow;",
//         "Blue < Purple;",
//         "Blue == Purple;",
//         "Blue > Purple;",
//         "Yellow < Purple;",
//         "Yellow == Purple;",
//         "Yellow > Purple;"
//     );

//     return vec_criteria_cards;
// }
