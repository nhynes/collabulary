use anyhow::Result;
use futures_util::{stream::SplitSink, Sink, SinkExt as _};
use serde::{Deserialize, Serialize};
use tokio::sync::{Mutex, RwLock};
use tracing::debug;
use warp::filters::ws::Message;

use crate::vocab::Vocab;

pub struct Game<S: Sink<Message> + Unpin> {
    vocab: Vocab,
    state: RwLock<GameState>,

    /// The participant learning Chinese.
    en_participant: Mutex<Option<SplitSink<S, Message>>>,

    /// The participant learning English.
    zh_participant: Mutex<Option<SplitSink<S, Message>>>,
}

impl<S> Game<S>
where
    S: Sink<Message> + Unpin,
    S::Error: std::error::Error + Send + Sync + 'static,
{
    pub fn new(vocab: Vocab) -> Self {
        Self {
            state: RwLock::new(GameState {
                round: Round::new_random(&vocab),
            }),
            en_participant: Mutex::new(None),
            zh_participant: Mutex::new(None),
            vocab,
        }
    }

    pub async fn set_participant(
        &self,
        locale: Locale,
        mut participant: SplitSink<S, Message>,
    ) -> Result<(), (anyhow::Error, SplitSink<S, Message>)> {
        let mut p = match locale {
            Locale::English => self.en_participant.lock().await,
            Locale::中文 => self.zh_participant.lock().await,
        };
        // if p.is_some() {
        //     return Err((
        //         anyhow!("{:?} particpant has already joined", locale),
        //         participant,
        //     ));
        // }

        let state = self.state.read().await;
        if let Err(e) = Self::push_state(&mut participant, &*state, locale).await {
            return Err((e, participant));
        }

        *p = Some(participant);

        Ok(())
    }

    async fn push_state(
        participant: &mut SplitSink<S, Message>,
        state: &GameState,
        locale: Locale,
    ) -> Result<()> {
        let GameState {
            round:
                Round {
                    en_card,
                    zh_card,
                    en_score,
                    zh_score,
                    en_ready,
                    zh_ready,
                },
        } = &*state;

        let (your_card, their_card, your_score, their_score, ready) = match locale {
            Locale::English => (zh_card, en_card, zh_score, en_score, zh_ready),
            Locale::中文 => (en_card, zh_card, en_score, zh_score, en_ready),
        };

        participant
            .send(Message::text(serde_json::to_string(&ClientState {
                your_card,
                their_card,
                your_score: *your_score,
                has_score: their_score.is_some(),
                ready: *ready,
            })?))
            .await?;

        Ok(())
    }

    pub async fn get_participant(
        &self,
        locale: Locale,
    ) -> tokio::sync::MutexGuard<'_, Option<SplitSink<S, Message>>> {
        match locale {
            Locale::English => self.en_participant.lock().await,
            Locale::中文 => self.zh_participant.lock().await,
        }
    }

    pub async fn take_participant(&self, locale: Locale) -> Option<SplitSink<S, Message>> {
        let mut p = match locale {
            Locale::English => self.en_participant.lock().await,
            Locale::中文 => self.zh_participant.lock().await,
        };
        p.take()
    }

    pub async fn set_score(&self, locale: Locale, score: u8) -> Result<()> {
        let mut state = self.state.write().await;

        match locale {
            Locale::English => {
                state.round.zh_score = Some(score);
            }
            Locale::中文 => {
                state.round.en_score = Some(score);
            }
        }

        if let Some(p) = &mut *self.get_participant(locale).await {
            Self::push_state(p, &state, locale).await?;
        }
        if let Some(p) = &mut *self.get_participant(locale.other()).await {
            Self::push_state(p, &state, locale.other()).await.ok();
        }

        Ok(())
    }

    pub async fn set_ready(&self, locale: Locale) -> Result<()> {
        let mut state = self.state.write().await;

        match locale {
            Locale::English => {
                state.round.zh_ready = true;
            }
            Locale::中文 => {
                state.round.en_ready = true;
            }
        }

        if state.round.zh_ready && state.round.en_ready {
            debug!("starting new round");
            Self::next_round(&mut state, &self.vocab)?;
        }

        if let Some(p) = &mut *self.get_participant(locale).await {
            Self::push_state(p, &state, locale).await?;
        }
        if let Some(p) = &mut *self.get_participant(locale.other()).await {
            Self::push_state(p, &state, locale.other()).await.ok();
        }

        Ok(())
    }

    fn next_round(state: &mut GameState, vocab: &Vocab) -> Result<()> {
        state.round = Round::new_random(vocab);
        Ok(())
    }
}

#[derive(Default)]
struct GameState {
    round: Round,
}

#[derive(Default)]
struct Round {
    /// The card for the person learning English;
    en_card: Card,

    /// The card for the person learning Chinese;
    zh_card: Card,

    /// The score of the person learning English.
    en_score: Option<u8>,

    /// The score of the person learning Chinese.
    zh_score: Option<u8>,

    /// Whether the person learning English is ready.
    en_ready: bool,

    /// Whether the person learning Chinese is ready.
    zh_ready: bool,
}

impl Round {
    fn new_random(vocab: &Vocab) -> Self {
        Self {
            en_card: Card::random(vocab),
            zh_card: Card::random(vocab),
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    #[serde(flatten)]
    pub word: VocabWord,
    pub side: CardSide,
}

impl Card {
    fn random(vocab: &Vocab) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            word: rand::seq::SliceRandom::choose(&**vocab, &mut rng)
                .unwrap() // Vocab is non-empty.
                .to_owned()
                .into(),
            side: if rand::Rng::gen(&mut rng) {
                CardSide::Word
            } else {
                CardSide::Definition
            },
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CardSide {
    Word,
    Definition,
}

impl Default for CardSide {
    fn default() -> Self {
        Self::Word // arbitrary
    }
}

#[serde_with::serde_as]
#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VocabWord {
    word: String,
    word_detail: String,
    #[serde_as(deserialize_as = "serde_with::OneOrMany<_>")]
    definition: Vec<String>,
}

impl From<crate::vocab::VocabWord> for VocabWord {
    fn from(word: crate::vocab::VocabWord) -> Self {
        Self {
            word: word.word,
            word_detail: word.word_detail,
            definition: word.definition,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Locale {
    English,
    中文,
}

impl Locale {
    fn other(&self) -> Self {
        match self {
            Self::English => Self::中文,
            Self::中文 => Self::English,
        }
    }
}

impl std::str::FromStr for Locale {
    type Err = anyhow::Error;

    fn from_str(locale: &str) -> Result<Self> {
        Ok(if locale.starts_with("en") {
            Self::English
        } else if locale.starts_with("zh") {
            Self::中文
        } else {
            anyhow::bail!("unsupported locale: {}", locale)
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase", untagged)]
pub enum Request {
    Score { score: u8 },
    Action { action: Action },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Action {
    AdvanceRound,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientState<'a> {
    #[serde(rename = "myCard")]
    your_card: &'a Card,
    their_card: &'a Card,

    /// Whether the other person has given you a score.
    has_score: bool,

    #[serde(rename = "scored")]
    your_score: Option<u8>,

    /// Whether the client has submitted the score and is ready for the next round.
    ready: bool,
}
