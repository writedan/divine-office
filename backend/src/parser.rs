pub mod ast;

use crate::lexer::{Lexer, Token};
use crate::parser::ast::*;
use std::collections::HashMap;

/// Hymns are expressed as a struct since they contain more information than a typical directive.
#[derive(Debug, Clone)]
pub struct Hymn {
    pub clef: String,
    pub tone: String,
    /// Corresponds to a `Vec<Melody>` since melodies are expressed as vector of individual neumes.
    pub melody: Vec<Vec<String>>,
    /// Corresponds to a `Vec<Verse>` since verses are expressed as a vector of individual syllables.
    /// The number of stanzas is equal to the number of verses divided by the number of melodies.
    ///
    /// For example, given 4 melodies and 8 verses, there are two stanzas of 4 verses each.
    // verses[0] would be the first verse of the first stanza, verses[1] the second verse of the first stanza, ... verses[4] the first verse of the second stanza.
    pub verses: Vec<Vec<String>>,
    /// The two nuemes of the final "Amen."
    pub amen: (String, String),
}

impl Hymn {
    /// Returns the absolute index out of `verses` given the requested stanza and verse indices.
    pub fn verse_idx(&self, stanza: usize, verse: usize) -> usize {
        stanza + verse * (self.verses.len() / self.melody.len())
    }
}

/// A directive, unlike a token, can be compiled directly into HTML or other targets.
/// Refer to the `Token` enum for information on these enums.
#[derive(Debug, Clone)]
pub enum Directive {
    /// Indicates a tree which compiles into a `div.boxed` element. This directive occurs legally only as the root of a tree.
    Box,
    Error(String),
    Empty,
    Heading(String, u8),
    /// Indicates a hymn environment. See the `Hymn` struct for more information.
    Hymn(Hymn),
    RawGabc(String),
    Instruction(String),
    Text(String),
    Title(String),
}

pub struct Parser {
    resumables: HashMap<String, ASTNode<Directive>>,
    iter: Box<dyn ExactSizeIterator<Item = Token>>,
}

impl Parser {
    #[cfg(feature = "lua_support")]
    pub fn from_file(path: std::path::PathBuf) -> ASTree<Directive> {
        let preprocessor = Preprocessor::from_path(path.as_path(), HashMap::new());
        let mut base = ASTree::<Directive>::new();

        if let Ok(mut preprocessor) = preprocessor {
            preprocessor.preprocess();
            let mut parser = Parser {
                resumables: HashMap::new(),
                iter: preprocessor.iter()
            };
            base.add_node(ASTNode::Tree(parser.parse_tokens()));
        } else if let Err(why) = preprocessor {
            base.add_node(ASTNode::Node(Directive::Error(format!(
                "Failed to initialize preprocessor: {}",
                why
            ))));
        }

        base
    }

    pub fn from_hour(propers: HashMap<&'static str, std::path::PathBuf>) -> ASTree<Directive> {
        let mut base = ASTree::<Directive>::new();

        let mut store: HashMap<String, String> = HashMap::new();
        for (key, val) in propers.iter() {
            store.insert(key.to_string(), val.display().to_string());
        }

        let preprocessor = Preprocessor::from_path(
            match propers.get("order") {
                Some(path) => path,
                None => {
                    base.add_node(ASTNode::Node(Directive::Error(format!(
                        "Cannot parse from hour: field \"order\" was not set."
                    ))));
                    return base;
                }
            },
            store,
        );

        if let Ok(mut preprocessor) = preprocessor {
            preprocessor.preprocess();
            let mut parser = Parser {
                resumables: HashMap::new(),
                iter: preprocessor.iter(),
            };
            base.add_node(ASTNode::Tree(parser.parse_tokens()));
        } else if let Err(why) = preprocessor {
            base.add_node(ASTNode::Node(Directive::Error(format!(
                "Failed to initialize preprocessor: {}",
                why
            ))));
        }

        base
    }

    fn parse_token(&mut self, token: Token) -> ASTNode<Directive> {
        use Token::*;
        match token {
            BeginBox => self.read_box(),

            BeginHymn(tone) => self.read_hymn(tone),

            BeginResumable(key) => {
                let tree = self.read_resumable();
                self.resumables.insert(key, tree);
                ASTNode::Node(Directive::Empty)
            }

            Define(_, _) => ASTNode::Node(Directive::Empty), // definitions are left in the preprocessor to enable dynamic resolutions

            Error(why) => ASTNode::Node(Directive::Error(why)),

            Heading(text, level) => ASTNode::Node(Directive::Heading(text, level)),

            IfInclude(_) => ASTNode::Node(Directive::Empty), // if-includes transform to nothing if their key is missing

            Include(key) => ASTNode::Node(Directive::Error(format!("Field not set: {}", key))), // includes transform to an error if they key is missing

            Instruction(text) => ASTNode::Node(Directive::Instruction(text)),

            NoGloria => ASTNode::Node(Directive::Empty),

            Text(text) => ASTNode::Node(Directive::Text(text)),

            Title(text) => ASTNode::Node(Directive::Title(text)),

            RawGabc(gabc) => ASTNode::Node(Directive::RawGabc(gabc)),

            Resume(key) => match self.resumables.remove(&key) {
                Some(tree) => tree,
                None => ASTNode::Node(Directive::Error(format!(
                    "No resumable defined with name \"{}\"",
                    key
                ))),
            },

            _ => ASTNode::Node(Directive::Error(format!(
                "Unexpected token while parsing: {:?}",
                token
            ))),
        }
    }

    fn parse_tokens(&mut self) -> ASTree<Directive> {
        let mut base = ASTree::<Directive>::new();
        while self.iter.len() > 0 {
            let token = self.iter.next().unwrap();
            base.add_node(self.parse_token(token));
        }

        base
    }

    fn read_box(&mut self) -> ASTNode<Directive> {
        use Token::*;

        let mut base = ASTree::<Directive>::from_root(Directive::Box);
        while self.iter.len() > 0 {
            let token = self.iter.next().unwrap();
            match token {
                BeginBox => base.add_node(self.read_box()),
                End => return ASTNode::Tree(base),
                _ => base.add_node(self.parse_token(token)),
            }
        }

        ASTNode::Node(Directive::Error(format!("Box began without termination")))
    }

    fn read_resumable(&mut self) -> ASTNode<Directive> {
        use Token::*;

        let mut base = ASTree::<Directive>::new();
        while self.iter.len() > 0 {
            let token = self.iter.next().unwrap();
            match token {
                End => return ASTNode::Tree(base),
                _ => base.add_node(self.parse_token(token)),
            }
        }

        ASTNode::Node(Directive::Error(format!("Box began without termination")))
    }

    fn read_hymn(&mut self, tone: String) -> ASTNode<Directive> {
        let mut melody: Vec<Vec<String>> = Vec::new();
        let mut verses: Vec<Vec<Vec<String>>> = Vec::new();
        let mut clef = String::new();
        let mut amen = (String::new(), String::new());

        while self.iter.len() > 0 {
            use Token::*;
            let token = self.iter.next().unwrap();
            match token {
                Amen(s1, s2) => amen = (s1, s2),

                Clef(val) => clef = val,

                End => break,

                Melody(syllales) => {
                    melody.push(syllales);
                    verses.push(Vec::new())
                }

                Verse(syllales) => match verses.last_mut() {
                    Some(vec) => vec.push(syllales),
                    None => {
                        return ASTNode::Node(Directive::Error(format!(
                            "No melody was declared but tried to provide verse."
                        )))
                    }
                },

                _ => {
                    return ASTNode::Node(Directive::Error(format!(
                        "Illegal token while parsing hymn: {:?}",
                        token
                    )))
                }
            }
        }

        // verify the hymn is well-formed
        if melody.len() == 0 || verses.len() == 0 {
            return ASTNode::Node(Directive::Error(format!("Hymn has no melody or verses")));
        }

        let standard_len = verses[0].len();
        for (idx, melody) in melody.iter().enumerate() {
            if verses[idx].len() == 0 {
                return ASTNode::Node(Directive::Error(format!(
                    "Melody has no corresponding verses for stanza {}",
                    idx + 1
                )));
            }

            for verse in verses[idx].iter() {
                if verse.len() != melody.len() {
                    return ASTNode::Node(Directive::Error(format!("Melody and verse have differing syllable counts for stanza {} on verse {:?}", idx + 1, verse)));
                }
            }

            if verses[idx].len() != standard_len {
                return ASTNode::Node(Directive::Error(format!(
                    "Stanza {} has differing number of verses from first stanza.",
                    idx + 1
                )));
            }
        }

        // verses are flattened one layer since they are initially grouped by stanza
        ASTNode::Node(Directive::Hymn(Hymn {
            melody,
            verses: verses.into_iter().flatten().collect::<Vec<_>>(),
            clef,
            tone,
            amen,
        }))
    }
}

/// A helper method to transform tones to their stress-patterns.
fn resolve_tone(tone: &String) -> String {
    match tone.as_str() {
        "1a" => "1",    // tone 1
        "1a2" => "1",
        "1d" => "1",
        "1d2" => "1",
        "1d3" => "1",
        "1f" => "1",
        "1g" => "1",
        "1g2" => "1",
        "1g3" => "1",
        "2d" => "2",    // tone 2 (2* undecided)
        "3b" => "3",    // tone 3
        "3a" => "3",
        "3g" => "3",
        "3a2" => "3",
        "4a" => "4-i",  // tone 4 (4* considered 4e)
        "4g" => "4-ii",
        "4e" => "4-iii",
        "4*a" => "4-ii",// irregular tone
        "5a" => "5",    // tone 5
        "5a2" => "5",
        "6f" => "6",    // tone 6
        "6c" => "6",
        "7d" => "7",    // tone 7
        "7c" => "7",
        "7c2" => "7",
        "7a" => "7",
        "8c" => "8",    // tone 8
        "8g" => "8",
        "8g2" => "8",
        _ => tone
    }.to_string()
}

/// The processor expands certain tokens so as to ease the work of the parser.
#[derive(Debug)]
struct Preprocessor {
    tokens: Vec<Token>,
    /// A dynamic map of fields and paths which can update during parsing.
    store: HashMap<String, String>,
}

impl Preprocessor {
    /// Creates a preprocessor out of a path and given store of values.
    pub fn from_path<P>(path: P, store: HashMap<String, String>) -> Result<Preprocessor, String>
    where
        P: AsRef<std::path::Path> + std::fmt::Debug + Copy,
    {
        Ok(Preprocessor {
            tokens: Lexer::from_path(path)?.tokenize(),
            store,
        })
    }

    /// Provides an iterator on the tokens in the preprocessor.
    pub fn iter(&self) -> Box<dyn ExactSizeIterator<Item = Token>> {
        Box::new(self.tokens.clone().into_iter())
    }

    /// Helper method to get an integer representation of `tokens` to check when the Vec has been modified.
    fn calculate_hash(&self) -> u64 {
        use std::hash::{DefaultHasher, Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        self.tokens.hash(&mut hasher);
        hasher.finish()
    }

    /// Executes passes until no more changes have occured.
    pub fn preprocess(&mut self) {
        let mut hash = self.calculate_hash();
        loop {
            self.pass();

            let new_hash = self.calculate_hash();
            if new_hash == hash {
                break;
            } else {
                hash = new_hash;
            }
        }
    }

    /// Execute one pass of the preprocessor.
    fn pass(&mut self) {
        let mut insertions: Vec<(usize, Vec<Token>)> = Vec::new();
        let mut temp_store: HashMap<String, String> = HashMap::new();

        self.tokens = self
            .tokens
            .clone()
            .into_iter()
            .enumerate()
            .map(|(idx, token)| {
                use std::path::PathBuf;
                use Token::*;

                match token {
                    Antiphon(name, repeat) => {
                        let repeat = repeat.unwrap_or(name.to_owned());

                        self.store
                            .insert("internal:last-antiphon".into(), repeat.to_owned());
                        let mut new_tokens = Vec::<Token>::new();
                        new_tokens.push(Token::Title("Antiphon".into()));
                        new_tokens.push(Token::Import(format!("antiphon/{}.gabc", name)));
                        new_tokens.push(Token::Define(
                            "internal:last-antiphon".into(),
                            repeat.to_owned(),
                        ));
                        insertions.push((idx, new_tokens));
                        Token::Empty
                    }

                    Define(key, value) => {
                        self.store.insert(key.clone(), value.clone());
                        Token::Define(key.clone(), value.clone())
                    }

                    Gabc(gabc) => Token::RawGabc(format!(
                        "initial-style: 0;\ncentering-scheme: english;\n%%\n{}",
                        gabc
                    )),

                    // if the tone is set, will resolve to Gloria(Some(_)), but if not will remain Gloria(None)
                    Gloria(None) => Token::Gloria(self.store.get("internal:last-tone").cloned()),

                    Gloria(Some(tone)) => {
                        if let Some("true") = temp_store
                            .get(&<&str as Into<String>>::into("no-gloria"))
                            .map(String::as_str)
                        {
                            temp_store.remove(&<&str as Into<String>>::into("no-gloria"));
                            insertions.push((idx, Vec::new()));
                            Token::Empty
                        } else {
                            Token::Import(format!("commons/gloria/{}.lit", resolve_tone(&tone)))
                        }
                    }

                    NoGloria => {
                        temp_store.insert("no-gloria".into(), "true".into());
                        Token::NoGloria
                    }

                    Import(path) => {
                        let path = PathBuf::from(path);

                        let ext = match path.extension() {
                            Some(ext) => ext,
                            None => {
                                return Token::Error(format!(
                                    "Failed to resolve import {:?}: missing extension",
                                    path
                                ))
                            }
                        };

                        if ext == "lit" {
                            // we cannot directly add new tokens from within this mapping, so we add the new tokens to an insertations table
                            let new_tokens = match Lexer::from_path(&path) {
                                Ok(mut lexer) => lexer.tokenize(),
                                Err(why) => {
                                    return Token::Error(format!(
                                        "Failed to resolve import {:?}: {}",
                                        path, why
                                    ))
                                }
                            };

                            insertions.push((idx, new_tokens));
                            return Token::Empty;
                        } else if ext == "gabc" {
                            return match crate::lexer::read_file(&path) {
                                Ok(gabc) => Token::RawGabc(gabc),
                                Err(why) => Token::Error(format!(
                                    "Failed to resolve import {:?}: {}",
                                    path, why
                                )),
                            };
                        } else {
                            return Token::Error(format!(
                                "Failed to resolve import {:?}: unknown extension {:?}",
                                path, ext
                            ));
                        }
                    }

                    IfInclude(ref key) | Include(ref key) => {
                        match self.store.get(key) {
                            Some(val) => Token::Import(val.to_owned()),
                            None => match token {
                                // we will only reserve to Empty or Error at the parsing stage (once all tokens have been expanded)
                                Token::IfInclude(_) => Token::IfInclude(key.to_owned()),
                                Token::Include(_) => Token::Include(key.to_owned()),
                                _ => unreachable!(),
                            },
                        }
                    }

                    Tone(tone) => {
                        self.store
                            .insert("internal:last-tone".into(), tone.to_owned());
                        insertions.push((
                            idx,
                            vec![
                                Token::Define("internal:last-tone".into(), tone.to_owned()),
                                Token::Import(format!("tone/{}.gabc", tone)),
                            ],
                        ));
                        Token::Empty
                    }

                    Psalm(iden) => {
                        let mut new_tokens: Vec<Token> = Vec::new();

                        if let Some(tone) = self.store.get("internal:last-tone") {
                            if iden.parse::<u8>().is_ok() {
                                new_tokens.push(Token::Title(format!("Psalm {}", iden.clone())));
                            }

                            let pattern = resolve_tone(tone);
                            new_tokens
                                .push(Token::Import(format!("psalter/{}/{}.lit", iden, pattern)));

                            insertions.push((idx, new_tokens));
                            Token::Empty
                        } else {
                            Token::Psalm(iden)
                        }
                    }

                    RepeatAntiphon => {
                        if let Some(antiphon) = self.store.get("internal:last-antiphon") {
                            let gabc = match crate::lexer::read_file(format!(
                                "antiphon/{}.gabc",
                                antiphon
                            )) {
                                Ok(gabc) => gabc,
                                Err(why) => {
                                    return Token::Error(format!(
                                        "Failed to get repeat-antiphon {:?}: {}",
                                        antiphon, why
                                    ))
                                }
                            };

                            return Token::RawGabc(gabc)

                            // let re = regex::Regex::new(r"\r?\n").unwrap();
                            // let gabc = re.split(&gabc).collect::<Vec<_>>();

                            // return Token::Gabc(gabc[1].to_string().replace("<sp>*</sp>", ""));
                        } else {
                            Token::RepeatAntiphon
                        }
                    }

                    RepeatHalfAntiphon(amount) => {
                        if let Some(antiphon) = self.store.get("internal:last-antiphon") {
                            let gabc = match crate::lexer::read_file(format!(
                                "antiphon/{}.gabc",
                                antiphon
                            )) {
                                Ok(gabc) => gabc,
                                Err(why) => {
                                    return Token::Error(format!(
                                        "Failed to get repeat-antiphon {:?}: {}",
                                        antiphon, why
                                    ))
                                }
                            };

                            let gabc = gabc.split("%%").collect::<Vec<_>>();

                            if amount == "half" {
                                return Token::Gabc(format!(
                                    "<sp>r</sp> {}",
                                    gabc[1].to_string().split("+(;)").collect::<Vec<_>>()[1]
                                        .to_string()
                                ));
                            } else if amount == "full" {
                                return Token::Gabc(format!("<sp>r</sp> {}", gabc[1].to_string()));
                            } else {
                                return Token::Error(format!("Unknown repeat type {}", amount));
                            }
                        } else {
                            Token::RepeatAntiphon
                        }
                    }

                    RepeatTone => {
                        if let Some(tone) = self.store.get("internal:last-tone") {
                            Token::Tone(tone.to_owned())
                        } else {
                            Token::RepeatTone
                        }
                    }

                    _ => token,
                }
            })
            .collect::<Vec<_>>();

        // we can now add any new tokens which occurred
        // we go in reverse so as not to disturb the indices
        for (idx, new_tokens) in insertions.into_iter().rev() {
            self.tokens.splice(idx..idx + 1, new_tokens);
        }
    }
}
