pub struct CriteriaCard {
    pub card_num: u8,
    pub description: String,
    pub tests: Vec<String>,
}

#[macro_export]
macro_rules! add_criteria_card {
    ($a:expr, $b:expr, $c:expr, $($d:expr),*) => {
        $a.push(CriteriaCard {
                card_num: $b,
                description: String::from($c),
                tests: vec![$(String::from($d)),*],
        });
    };
}