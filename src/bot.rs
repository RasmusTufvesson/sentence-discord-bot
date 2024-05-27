use rand::{rngs::OsRng, seq::SliceRandom, Rng};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use crate::words::{Bestämd, Category, Genus, VerbExpects, Words};

#[derive(PartialEq, Clone)]
pub enum Part {
    Begin,
    HasVerb,
}

pub struct Info {
    words: Words,
    part: Part,
    bestämd: bool,
    verb: Option<usize>,
}

struct Handler {
    info: Mutex<Info>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.channel_id == include!("../CHANNEL") && msg.author.id != include!("../BOT_ID") {
            let mut info = self.info.lock().await;
            let guess = info.words.guess_word(&msg.content);
            println!("{}: {:?}", msg.content, guess);
            let mut part = info.part.clone();
            let mut bestämd = info.bestämd;
            let mut verb = info.verb;
            let word = match guess.0 {
                Category::Substantiv => info.words.could_verb(&mut part, &mut verb),
                Category::Adjektiv => {
                    let i = guess.1.unwrap();
                    if info.words.adjektiv[i].0 == msg.content {
                        &info.words.random_gendered_substantiv(Genus::N).0
                    } else if info.words.adjektiv[i].1 == msg.content {
                        &info.words.random_gendered_substantiv(Genus::T).0
                    } else {
                        &info.words.random_substantiv().1
                    }
                }
                Category::Pronomen => info.words.could_verb(&mut part, &mut verb),
                Category::Namn => info.words.could_verb(&mut part, &mut verb),
                Category::Verb => {
                    part = Part::HasVerb;
                    verb = guess.1;
                    if let Some(i) = guess.1 {
                        let mut rng = OsRng::default();
                        let (group, expects) = info.words.verb[i].1.choose(&mut rng).unwrap();
                        let choice = group.choose(&mut rng).unwrap();
                        if choice != "" {
                            choice
                        } else {
                            match expects {
                                VerbExpects::None => info.words.end_of_part(),
                                VerbExpects::NoneOrSub => {
                                    if info.words.rng.gen_bool(0.5) {
                                        info.words.end_of_part()
                                    } else {
                                        info.words.random_objekt(&mut bestämd)
                                    }
                                }
                                VerbExpects::Sub => info.words.random_objekt(&mut bestämd),
                                VerbExpects::SubOrAdj => {
                                    if info.words.rng.gen_bool(0.5) {
                                        &info.words.random_adjektiv().0
                                    } else {
                                        info.words.random_objekt(&mut bestämd)
                                    }
                                }
                            }
                        }
                    } else {
                        info.words.random_objekt(&mut bestämd)
                    }
                }
                Category::Bindeord => {
                    info.part = Part::Begin;
                    info.words.random_subjekt(&mut bestämd)
                }
                Category::Tidsord => info.words.could_verb(&mut part, &mut verb),
                Category::Punkt => {
                    info.part = Part::Begin;
                    if info.words.rng.gen_bool(0.5) {
                        &info.words.random_tidsord().0
                    } else {
                        info.words.random_subjekt(&mut bestämd)
                    }
                }
                Category::PronomenObjekt => {
                    info.part = Part::Begin;
                    if info.words.rng.gen_bool(0.5) {
                        &info.words.random_tidsord().0
                    } else {
                        info.words.random_subjekt(&mut bestämd)
                    }
                }
                Category::PronomenPossessiv => {
                    let i = guess.1.unwrap();
                    let gender = info.words.pronomen_possessiv[i].1.clone();
                    &info.words.random_gendered_substantiv(gender).0
                }
                Category::Komma => {
                    info.part = Part::Begin;
                    if info.words.rng.gen_bool(0.5) {
                        info.words.random_subjekt(&mut bestämd)
                    } else {
                        &info.words.random_bindeord().0
                    }
                }
                Category::Preposition => {
                    if let Some(index) = verb {
                        let mut value = None; 
                        'outer: for (group, expects) in &info.words.verb[index].1 {
                            for string in group {
                                if string == &msg.content {
                                    value = Some(match expects {
                                        VerbExpects::None => info.words.end_of_part(),
                                        VerbExpects::NoneOrSub => {
                                            if info.words.rng.gen_bool(0.5) {
                                                info.words.end_of_part()
                                            } else {
                                                info.words.random_objekt(&mut bestämd)
                                            }
                                        }
                                        VerbExpects::Sub => info.words.random_objekt(&mut bestämd),
                                        VerbExpects::SubOrAdj => {
                                            if info.words.rng.gen_bool(0.5) {
                                                &info.words.random_adjektiv().0
                                            } else {
                                                info.words.random_objekt(&mut bestämd)
                                            }
                                        }
                                    });
                                    break 'outer;
                                }
                            }
                        }
                        if let Some(value) = value {
                            value
                        } else {
                            info.words.random_objekt(&mut bestämd)
                        }
                    } else {
                        info.words.random_objekt(&mut bestämd)
                    }
                }
                Category::Artikel => {
                    let i = guess.1.unwrap();
                    let gender = info.words.artiklar[i].1.clone();
                    match info.words.artiklar[i].2 {
                        Bestämd::Definite => {
                            bestämd = true;
                            &info.words.random_gendered_substantiv(gender).1
                        }
                        Bestämd::Indefinite => {
                            bestämd = false;
                            &info.words.random_gendered_substantiv(gender).0
                        }
                    }
                }
            };
            if let Err(why) = msg.channel_id.say(&ctx.http, word).await {
                println!("Error sending message: {why:?}");
            }
            info.part = part;
            info.bestämd = bestämd;
            info.verb = verb;
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

pub async fn run(token: &str, words: Words) {
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client =
        Client::builder(token, intents).event_handler(Handler { info: Mutex::new(Info { words, part: Part::Begin, bestämd: false, verb: None }) }).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}