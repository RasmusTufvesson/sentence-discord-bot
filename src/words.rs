use std::fs;
use serde::{Deserialize, Serialize};
use rand::{seq::SliceRandom, rngs::OsRng};

#[derive(Serialize, Deserialize)]
pub struct Words {
    pub substantiv: Vec<Substantiv>,
    pub verb: Vec<Verb>,
    pub adjektiv: Vec<Adjektiv>,
    pub pronomen: Vec<Pronomen>,
    pub namn: Vec<Namn>,
    #[serde(skip)]
    #[serde(default)]
    rng: OsRng,
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

    pub fn random_pronomen(&mut self) -> &Pronomen {
        self.pronomen.choose(&mut self.rng).unwrap()
    }

    pub fn random_namn(&mut self) -> &Namn {
        self.namn.choose(&mut self.rng).unwrap()
    }

    pub fn guess_word(&self, query: &str) -> (Category, Option<usize>) {
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
            if word.0 == query {
                return (Category::Namn, Some(i))
            }
        }
        if let Some(chr) = query.chars().next() {
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
pub struct Substantiv (pub String, pub String, pub Genus);

#[derive(Serialize, Deserialize)]
pub struct Verb (pub String);

#[derive(Serialize, Deserialize)]
pub struct Adjektiv (pub String, pub String, pub String);

#[derive(Serialize, Deserialize)]
pub struct Pronomen (pub String);

#[derive(Serialize, Deserialize)]
pub struct Namn (pub String, pub Gender);

#[derive(Debug)]
pub enum Category {
    Substantiv,
    Verb,
    Adjektiv,
    Pronomen,
    Namn,
}

#[derive(Serialize, Deserialize)]
pub enum Genus {
    N,
    T,
}

#[derive(Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
}