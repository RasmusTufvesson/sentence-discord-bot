use std::fs;
use serde::{Deserialize, Serialize};
use rand::{rngs::OsRng, seq::SliceRandom, Rng};
use crate::bot::Part;

#[derive(Serialize, Deserialize)]
pub struct Words {
    pub substantiv: Vec<Substantiv>,
    pub verb: Vec<Verb>,
    pub adjektiv: Vec<Adjektiv>,
    pub pronomen: Vec<Ord>,
    pub pronomen_objekt: Vec<Ord>,
    pub pronomen_possessiv: Vec<Possessiv>,
    pub namn: Vec<Namn>,
    pub bindeord: Vec<Ord>,
    pub tidsord: Vec<Ord>,
    pub prepositioner: Vec<Ord>,
    pub artiklar: Vec<Artikel>,
    #[serde(skip)]
    #[serde(default)]
    pub rng: OsRng,
}

impl Words {
    pub fn load(path: &str) -> Self {
        ron::from_str(&fs::read_to_string(path).unwrap()).unwrap()
    }

    pub fn random_adjektiv(&mut self) -> &Adjektiv {
        self.adjektiv.choose(&mut self.rng).unwrap()
    }

    pub fn random_substantiv(&mut self) -> &Substantiv {
        self.substantiv.choose(&mut self.rng).unwrap()
    }

    pub fn random_verb(&mut self) -> &Verb {
        self.verb.choose(&mut self.rng).unwrap()
    }

    pub fn random_pronomen(&mut self) -> &Ord {
        self.pronomen.choose(&mut self.rng).unwrap()
    }

    pub fn random_namn(&mut self) -> &Namn {
        self.namn.choose(&mut self.rng).unwrap()
    }

    pub fn random_bindeord(&mut self) -> &Ord {
        self.bindeord.choose(&mut self.rng).unwrap()
    }

    pub fn random_tidsord(&mut self) -> &Ord {
        self.tidsord.choose(&mut self.rng).unwrap()
    }

    pub fn random_pronomen_objekt(&mut self) -> &Ord {
        self.pronomen_objekt.choose(&mut self.rng).unwrap()
    }

    pub fn random_possessiv(&mut self) -> &Possessiv {
        self.pronomen_possessiv.choose(&mut self.rng).unwrap()
    }

    pub fn random_gendered_substantiv(&mut self, gender: Genus) -> &Substantiv {
        if gender == Genus::B {
            self.random_substantiv()
        } else {
            let mut rng = OsRng::default();
            let mut ord = self.substantiv.choose(&mut rng).unwrap();
            while ord.3 != gender {
                ord = self.substantiv.choose(&mut rng).unwrap();
            }
            ord
        }
    }

    pub fn random_objekt(&mut self, bestämd: &mut bool) -> &str {
        if self.rng.gen_bool(0.4) {
            if self.rng.gen_bool(0.6) {
                &self.pronomen_objekt.choose(&mut self.rng).unwrap().0
            } else {
                &self.namn.choose(&mut self.rng).unwrap().0
            }
        } else if self.rng.gen_bool(0.5) {
            &self.random_possessiv().0
        } else if self.rng.gen_bool(0.5) {
            &self.substantiv.choose(&mut self.rng).unwrap().1
        } else {
            let artikel = self.artiklar.choose(&mut self.rng).unwrap();
            *bestämd = artikel.2 == Bestämd::Definite;
            &artikel.0
        }
    }

    pub fn random_subjekt(&mut self, bestämd: &mut bool) -> &str {
        if self.rng.gen_bool(0.4) {
            if self.rng.gen_bool(0.6) {
                &self.pronomen.choose(&mut self.rng).unwrap().0
            } else {
                &self.namn.choose(&mut self.rng).unwrap().0
            }
        } else if self.rng.gen_bool(0.5) {
            &self.random_possessiv().0
        } else if self.rng.gen_bool(0.5) {
            &self.substantiv.choose(&mut self.rng).unwrap().1
        } else {
            let artikel = self.artiklar.choose(&mut self.rng).unwrap();
            *bestämd = artikel.2 == Bestämd::Definite;
            &artikel.0
        }
    }

    pub fn could_verb(&mut self, part: &mut Part, verb: &mut Option<usize>) -> &str {
        if *part == Part::Begin {
            *part = Part::HasVerb;
            let index = self.rng.gen_range(0..self.verb.len());
            *verb = Some(index);
            &self.verb[index].0
        } else {
            *part = Part::Begin;
            self.end_of_part()
        }
    }

    pub fn end_of_part(&mut self) -> &str {
        if self.rng.gen_bool(0.3) {
            &self.random_bindeord().0
        } else if self.rng.gen_bool(0.5) {
            "."
        } else {
            ","
        }
    }

    pub fn guess_word(&self, query_pure: &str) -> (Category, Option<usize>) {
        let query = query_pure.to_lowercase();
        if query == "." {
            return (Category::Punkt, None);
        } else if query == "," {
            return (Category::Komma, None);
        }
        for (i, word) in self.substantiv.iter().enumerate() {
            if word.0 == query || word.1 == query {
                return (Category::Substantiv, Some(i))
            }
        }
        for (i, word) in self.verb.iter().enumerate() {
            if word.0 == query {
                return (Category::Verb, Some(i))
            }
        }
        for (i, word) in self.adjektiv.iter().enumerate() {
            if word.0 == query || word.1 == query || word.2 == query {
                return (Category::Adjektiv, Some(i))
            }
        }
        for (i, word) in self.pronomen.iter().enumerate() {
            if word.0 == query {
                return (Category::Pronomen, Some(i))
            }
        }
        for (i, word) in self.namn.iter().enumerate() {
            if word.0 == query_pure {
                return (Category::Namn, Some(i))
            }
        }
        for (i, word) in self.bindeord.iter().enumerate() {
            if word.0 == query {
                return (Category::Bindeord, Some(i))
            }
        }
        for (i, word) in self.tidsord.iter().enumerate() {
            if word.0 == query {
                return (Category::Tidsord, Some(i))
            }
        }
        for (i, word) in self.pronomen_objekt.iter().enumerate() {
            if word.0 == query {
                return (Category::PronomenObjekt, Some(i))
            }
        }
        for (i, word) in self.pronomen_possessiv.iter().enumerate() {
            if word.0 == query {
                return (Category::PronomenPossessiv, Some(i))
            }
        }
        for (i, word) in self.prepositioner.iter().enumerate() {
            if word.0 == query {
                return (Category::Preposition, Some(i))
            }
        }
        for (i, word) in self.artiklar.iter().enumerate() {
            if word.0 == query {
                return (Category::Artikel, Some(i))
            }
        }
        if let Some(chr) = query_pure.chars().next() {
            if chr.is_uppercase() {
                return (Category::Namn, None);
            }
        }
        if query.ends_with("de") {
            return (Category::Verb, None);
        }
        return (Category::Substantiv, None);
    }
}

#[derive(Serialize, Deserialize)]
pub struct Substantiv (pub String, pub String, pub String, pub Genus);

#[derive(Serialize, Deserialize)]
pub struct Adjektiv (pub String, pub String, pub String);

#[derive(Serialize, Deserialize)]
pub struct Namn (pub String, pub Gender);

#[derive(Serialize, Deserialize)]
pub struct Ord (pub String);

#[derive(Serialize, Deserialize)]
pub struct Possessiv (pub String, pub Genus);

#[derive(Serialize, Deserialize)]
pub struct Verb (pub String, pub Vec<(Vec<String>, VerbExpects)>);

#[derive(Serialize, Deserialize)]
pub struct Artikel (pub String, pub Genus, pub Bestämd);

#[derive(Debug)]
pub enum Category {
    Substantiv,
    Verb,
    Adjektiv,
    Pronomen,
    Namn,
    Bindeord,
    Tidsord,
    Punkt,
    PronomenObjekt,
    PronomenPossessiv,
    Komma,
    Preposition,
    Artikel,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum Genus {
    N,
    T,
    B, // båda
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum Bestämd {
    Definite,
    Indefinite,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum VerbExpects {
    None,
    Sub,
    SubOrAdj,
    NoneOrSub,
}