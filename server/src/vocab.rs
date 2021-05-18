use anyhow::Result;

#[derive(Clone, Debug, Default)]
pub struct Vocabs {
    pub en_zh: Vocab,
    pub zh_en: Vocab,
}

#[derive(Clone, Debug, Default)]
pub struct Vocab(Vec<VocabWord>);

impl Vocab {
    pub fn load_from_path(vocab_tsv: impl AsRef<std::path::Path>) -> Result<Self> {
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .from_path(vocab_tsv)?;
        let words: Vec<VocabWord> = rdr.deserialize().collect::<std::result::Result<_, _>>()?;
        assert!(!words.is_empty());
        Ok(Self(words))
    }
}

impl std::ops::Deref for Vocab {
    type Target = [VocabWord];

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

#[serde_with::serde_as]
#[derive(Clone, Debug, Default, serde::Deserialize)]
pub struct VocabWord {
    pub word: String,
    pub word_detail: String,
    #[serde_as(as = "serde_with::StringWithSeparator::<SlashSeparator, String>")]
    pub definition: Vec<String>,
}

struct SlashSeparator;

impl serde_with::Separator for SlashSeparator {
    fn separator() -> &'static str {
        "/"
    }
}
