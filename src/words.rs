use std::fs;
use serde::{Deserialize, Serialize};
use rand::{rngs::OsRng, seq::SliceRandom, Rng};

#[derive(Serialize, Deserialize)]
pub struct Words {
    pub substantiv: Vec<Substantiv>,
    pub verb: Vec<Ord>,
    pub adjektiv: Vec<Adjektiv>,
    pub pronomen: Vec<Ord>,
    pub namn: Vec<Namn>,
    pub bindeord: Vec<Ord>,
    pub tidsord: Vec<Ord>,
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

    pub fn random_verb(&mut self) -> &Ord {
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

    pub fn random_objekt(&mut self) -> &str {
        if self.rng.gen_bool(0.4) {
            if self.rng.gen_bool(0.6) {
                &self.pronomen.choose(&mut self.rng).unwrap().0
            } else {
                &self.namn.choose(&mut self.rng).unwrap().0
            }
        } else {
            &self.substantiv.choose(&mut self.rng).unwrap().0
        }
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
pub struct Substantiv (pub String, pub String, pub String, pub Genus);

#[derive(Serialize, Deserialize)]
pub struct Adjektiv (pub String, pub String, pub String);

#[derive(Serialize, Deserialize)]
pub struct Namn (pub String, pub Gender);

#[derive(Serialize, Deserialize)]
pub struct Ord (pub String);

#[derive(Debug)]
pub enum Category {
    Substantiv,
    Verb,
    Adjektiv,
    Pronomen,
    Namn,
    Bindeord,
    Tidsord,
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