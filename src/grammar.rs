#[derive(Clone, PartialEq)]
pub enum Gender {
    Masculine,
    Feminine,
    Neutral,
}

#[derive(Clone, PartialEq)]
pub enum Number {
    Singular,
    Plural,
}

#[derive(Clone, PartialEq)]
pub enum Case {
    Nominative,
    Accusative,
}

pub fn get_definite_article(gender: Gender, number: Number, case: Case) -> String {
    match number {
        Number::Plural => "die",
        Number::Singular => match gender {
            Gender::Masculine => match case {
                Case::Nominative => "der",
                Case::Accusative => "den",
            },
            Gender::Feminine => match case {
                Case::Nominative | Case::Accusative => "die",
            },
            Gender::Neutral => match case {
                Case::Nominative | Case::Accusative => "das",
            },
        },
    }
    .to_owned()
}
