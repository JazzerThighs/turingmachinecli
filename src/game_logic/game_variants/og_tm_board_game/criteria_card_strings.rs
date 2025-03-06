use colored::*;

pub fn criteria_card_strings() -> Vec<String> {
    let blue: ColoredString = "blue".to_string().bright_blue();
    let yellow: ColoredString = "yellow".to_string().yellow();
    let purple: ColoredString = "purple".to_string().purple();
    let mut vec_criteria_cards: Vec<String> = vec![];
    
    vec_criteria_cards.push(format!("the {blue} number compared to 3: \n I:    {blue} < 3; \n II:   {blue} == 3; \n III:  {blue} > 3;"));
    vec_criteria_cards.push(format!("the {blue} number compared to 1: \n I:    {blue} == 1; \n II:   {blue} > 1;"));
    vec_criteria_cards.push(format!("the {yellow} number compared to 3: \n I:    {yellow} < 3; \n II:   {yellow} == 3; \n III:  {yellow} > 3;"));
    vec_criteria_cards.push(format!("the {yellow} number compared to 4: \n I:    {yellow} < 4; \n II:   {yellow} == 4; \n III:  {yellow} > 4;"));
    vec_criteria_cards.push(format!("if {blue} is even or odd: \n I:    {blue} is even; \n II:   {blue} is odd;"));
    vec_criteria_cards.push(format!("if {yellow} is even or odd: \n I:    {yellow} is even; \n II:   {yellow} is odd;"));
    vec_criteria_cards.push(format!("if {purple} is even or odd: \n I:    {purple} is even; \n II:   {purple} is odd;"));
    vec_criteria_cards.push(format!("the number of 1's in the code: \n I:    zero 1's; \n II:   one 1; \n III:  two 1's; \n IV:   three 1's;"));
    vec_criteria_cards.push(format!("the number of 3's in the code: \n I:    zero 3's; \n II:   one 3; \n III:  two 3's; \n IV:   three 3's;"));
    vec_criteria_cards.push(format!("the number of 4's in the code: \n I:    zero 4's; \n II:   one 4; \n III:  two 4's; \n IV:   three 4's;"));
    vec_criteria_cards.push(format!("the {blue} number compared to the {yellow} number: \n I:    {blue} < {yellow}; \n II:   {blue} == {yellow}; \n III:  {blue} > {yellow};"));
    vec_criteria_cards.push(format!("the {blue} number compared to the {purple} number: \n I:    {blue} < {purple}; \n II:   {blue} == {purple}; \n III:  {blue} > {purple};"));
    vec_criteria_cards.push(format!("the {yellow} number compared to the {purple} number: \n I:    {yellow} < {purple}; \n II:   {yellow} == {purple}; \n III:  {yellow} > {purple};"));
    vec_criteria_cards.push(format!("which color's number is smaller than either of the others: \n I:    {blue} < {yellow} && {blue} < {purple}; \n II:   {yellow} < {blue} && {yellow} < {purple}; \n III:  {purple} < {blue} && {purple} < {yellow};"));
    vec_criteria_cards.push(format!("which color's number is larger than either of the others: \n I:    {blue} > {yellow} && {blue} > {purple}; \n II:   {yellow} > {blue} && {yellow} > {purple}; \n III:  {purple} > {blue} && {purple} > {yellow};"));
    vec_criteria_cards.push(format!("the number of even numbers compared to the number of odd numbers: \n I:    even > odd; \n II:   even < odd;"));
    vec_criteria_cards.push(format!("how many even numbers there are in the code: \n II:   one even number; \n III:  two even numbers; \n IV:   three even numbers;"));
    vec_criteria_cards.push(format!("if the sum of all the numbers is even or odd: \n I:    {blue} + {yellow} + {purple} == even; \n II:   {blue} + {yellow} + {purple} == odd;"));
    vec_criteria_cards.push(format!("the sum of {blue} and {yellow} compared to 6: \n I:    {blue} + {yellow} < 6; \n II:   {blue} + {yellow} == 6; \n III:  {blue} + {yellow} > 6;"));
    vec_criteria_cards.push(format!("if a number repeats itself in the code: \n I:    a triple number; \n II:   a double number; \n III:  no repetition;"));
    vec_criteria_cards.push(format!("if there is a number present exactly twice: \n I:    no pairs || three of a kind; \n II:   a pair;"));
    vec_criteria_cards.push(format!("if the three numbers in the code are in ascending order, descending order, or no order: \n I:    ascending order; \n II:   descending order; \n III:  no order;"));
    vec_criteria_cards.push(format!("the sum of all numbers compared to 6: \n I:    {blue} + {yellow} + {purple} < 6; \n II:   {blue} + {yellow} + {purple} == 6; \n III:  {blue} + {yellow} + {purple} > 6;"));
    vec_criteria_cards.push(format!("if there is a sequence of ascending numbers: \n I:    three numbers in ascending order; \n II:   two numbers in ascending order; \n III:  no numbers in ascending order;"));
    vec_criteria_cards.push(format!("if there is a sequence of ascending or descending numbers: \n I:    three numbers in ascending or descending order; \n II:   two numbers in ascending or descending order; \n III:  no numbers in ascending or descending order;"));
    vec_criteria_cards.push(format!("that a specific color is less than 3: \n I:    {blue} < 3; \n II:   {yellow} < 3; \n III:  {purple} < 3;"));
    vec_criteria_cards.push(format!("that a specific color is less than 4: \n I:    {blue} < 4; \n II:   {yellow} < 4; \n III:  {purple} < 4;"));
    vec_criteria_cards.push(format!("that a specific color is equal to 1: \n I:    {blue} == 1; \n II:   {yellow} == 1; \n III:  {purple} == 1;"));
    vec_criteria_cards.push(format!("that a specific color is equal to 3: \n I:    {blue} == 3; \n II:   {yellow} == 3; \n III:  {purple} == 3;"));
    vec_criteria_cards.push(format!("that a specific color is equal to 4: \n I:    {blue} == 4; \n II:   {yellow} == 4; \n III:  {purple} == 4;"));
    vec_criteria_cards.push(format!("that a specific color is greater than 1: \n I:    {blue} > 1; \n II:   {yellow} > 1; \n III:  {purple} > 1;"));
    vec_criteria_cards.push(format!("that a specific color is greater than 3: \n I:    {blue} > 3; \n II:   {yellow} > 3; \n III:  {purple} > 3;"));
    vec_criteria_cards.push(format!("that a specific color is even or odd: \n I:    {blue} is even; \n II:   {blue} is odd; \n III:  {yellow} is even; \n IV:   {yellow} is odd; \n V:    {purple} is even; \n VI:   {purple} is odd;"));
    vec_criteria_cards.push(format!("which color has the smallest number (or is tied for the smallest number): \n I:    {blue} <= {yellow} && {blue} <= {purple}; \n II:   {yellow} <= {blue} && {yellow} <= {purple}; \n III:  {purple} <= {blue} && {purple} <= {yellow};"));
    vec_criteria_cards.push(format!("which color has the largest number (or is tied for the largest number): \n I:    {blue} >= {yellow} && {blue} >= {purple}; \n II:   {yellow} >= {blue} && {yellow} >= {purple}; \n III:  {purple} >= {blue} && {purple} >= {yellow};"));
    vec_criteria_cards.push(format!("the sum of all the numbers is a multiple of 3 or 4 or 5: \n I:    {blue} + {yellow} + {purple} % 3 == 0; \n II:   {blue} + {yellow} + {purple} % 4 == 0; \n III:  {blue} + {yellow} + {purple} % 5 == 0;"));
    vec_criteria_cards.push(format!("the sum of two specific colors is equal to 4: \n I:    {blue} + {yellow} == 4; \n II:   {blue} + {purple} == 4; \n III:  {yellow} + {purple} == 4;"));
    vec_criteria_cards.push(format!("the sum of two specific colors is equal to 6: \n I:    {blue} == 1; \n II:   {blue} > 1; \n III:  {yellow} == 1; \n IV:   {yellow} > 1; \n V:    {purple} == 1; \n VI:   {purple} > 1;"));
    vec_criteria_cards.push(format!("the number of one specific color compared to 1: \n I:    {blue} == 1; \n II:   {blue} > 1; \n III:  {yellow} == 1; \n IV:   {yellow} > 1; \n V:    {purple} == 1; \n VI:   {purple} > 1;"));
    vec_criteria_cards.push(format!("the number of one specific color compared to 3: \n I:    {blue} < 3; \n II:   {blue} == 3; \n III:  {blue} > 3; \n IV:   {yellow} < 3; \n V:    {yellow} == 3; \n VI:   {yellow} > 3; \n VII:  {purple} < 3; \n VIII: {purple} == 3; \n IX:   {purple} > 3;"));
    vec_criteria_cards.push(format!("the number of one specific color compared to 4: \n I:    {blue} < 4; \n II:   {blue} == 4; \n III:  {blue} > 4; \n IV:   {yellow} < 4; \n V:    {yellow} == 4; \n VI:   {yellow} > 4; \n VII:  {purple} < 4; \n VIII: {purple} == 4; \n IX:   {purple} > 4;"));
    vec_criteria_cards.push(format!("which color is the smallest or the largest: \n I:    {blue} < {yellow} && {blue} < {purple}; \n II:   {blue} > {yellow} && {blue} > {purple}; \n III:  {yellow} < {blue} && {yellow} < {purple}; \n IV:   {yellow} > {blue} && {yellow} > {purple}; \n V:    {purple} < {blue} && {purple} < {yellow}; \n VI:   {purple} > {blue} && {purple} > {yellow};"));
    vec_criteria_cards.push(format!("the {blue} number compared to the number of another specific color: \n I:    {blue} < {yellow}; \n II:   {blue} < {purple}; \n III:  {blue} == {yellow}; \n IV:   {blue} == {purple}; \n V:    {blue} > {yellow}; \n VI:   {blue} > {purple};"));
    vec_criteria_cards.push(format!("the {yellow} number compared to the number of another specific color: \n I:    {yellow} < {blue}; \n II:   {yellow} < {purple}; \n III:  {yellow} == {blue}; \n IV:   {yellow} == {purple}; \n V:    {yellow} > {blue}; \n VI:   {yellow} > {purple};"));
    vec_criteria_cards.push(format!("how many 1's or how many 3's there are in the code: \n I:    zero 1's; \n II:   zero 3's; \n III:  one 1; \n IV:   one 3; \n V:    two 1's; \n VI:   two 3's;"));
    vec_criteria_cards.push(format!("how many 3's or how many 4's there are in the code: \n I:    zero 3's; \n II:   zero 4's; \n III:  one 3; \n IV:   one 4; \n V:    two 3's; \n VI:   two 4's;"));
    vec_criteria_cards.push(format!("how many 1's or how many 4's there are in the code: \n I:    zero 1's; \n II:   zero 4's; \n III:  one 1; \n IV:   one 4; \n V:    two 1's; \n VI:   two 4's;"));
    vec_criteria_cards.push(format!("one specific color compared to another specific color: \n I:    {blue} < {yellow}; \n II:   {blue} == {yellow}; \n III:  {blue} > {yellow}; \n IV:   {blue} < {purple}; \n V:    {blue} == {purple}; \n VI:   {blue} > {purple}; \n VII:  {yellow} < {purple}; \n VIII: {yellow} == {purple}; \n IX:   {yellow} > {purple};"));
    
    vec_criteria_cards
}
