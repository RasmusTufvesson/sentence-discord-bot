use rand::Rng;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use crate::words::{Category, Words};

#[derive(PartialEq)]
pub enum Part {
    Begin,
    HasVerb,
}

struct Handler {
    words: Mutex<Words>,
    part: Mutex<Part>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.channel_id == include!("../CHANNEL") && msg.author.id != include!("../BOT_ID") {
            let mut words = self.words.lock().await;
            let mut part = self.part.lock().await;
            let guess = words.guess_word(&msg.content);
            println!("{}: {:?}", msg.content, guess);
            let word = match guess.0 {
                Category::Substantiv => words.could_verb(&mut part),
                Category::Adjektiv => words.random_objekt(),
                Category::Pronomen => words.could_verb(&mut part),
                Category::Namn => words.could_verb(&mut part),
                Category::Verb => {
                    *part = Part::HasVerb;
                    words.random_objekt()
                }
                Category::Bindeord => {
                    *part = Part::Begin;
                    words.random_objekt()
                }
                Category::Tidsord => words.could_verb(&mut part),
                Category::Punkt => {
                    *part = Part::Begin;
                    if words.rng.gen_bool(0.5) {
                        &words.random_tidsord().0
                    } else {
                        words.random_subjekt()
                    }
                }
                Category::PronomenObjekt => {
                    *part = Part::Begin;
                    if words.rng.gen_bool(0.5) {
                        &words.random_tidsord().0
                    } else {
                        words.random_subjekt()
                    }
                }
                Category::PronomenPossessiv => {
                    let i = guess.1.unwrap();
                    let gender = words.pronomen_possessiv[i].1.clone();
                    &words.random_gendered_substantiv(gender).0
                }
            };
            if let Err(why) = msg.channel_id.say(&ctx.http, word).await {
                println!("Error sending message: {why:?}");
            }
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
        Client::builder(token, intents).event_handler(Handler { words: Mutex::new(words), part: Mutex::new(Part::Begin) }).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}