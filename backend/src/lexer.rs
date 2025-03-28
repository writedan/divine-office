use lazy_static::lazy_static;
use regex::Regex;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

lazy_static! {
    /// A regular expression used for matching the whole command-line with its arguments.
    /// It is used to isolate the command portion of a command-line.
    static ref WHOLE_RE: Regex = Regex::new(
        r#"#([\w-]+)(?:\s+"([^"]*)")*"#
    ).unwrap();

    /// A regular expression used to isolate the arguments of a command.
    /// Each group corresponds to one argument passed in.
    static ref COMMAND_RE: Regex = Regex::new(r#""([^"]*)""#).unwrap();
}

#[derive(Debug, Clone, std::hash::Hash)]
pub enum Token {
    /// The `amen` command is a specialized form of Verse which requires two melodies be provided for its two syllables.
    Amen(String, String),

    /// The `antiphon` command requires the name of the antiphon be provided, which must correspond to a `antiphon/<name>.gabc` file.
    ///
    /// It optionally accepts a second argument which will be used in place of the first antiphon on repeat.
    Antiphon(String, Option<String>),

    /// Indicates the beginning of a `div.boxed` element.
    BeginBox,

    /// Indicates the beginning of a hymn.
    BeginHymn,

    /// Indicates a section which is not printed until a request to resume it comes in, providing for non-linear documents. The argument is the name whereby it can be resumed.
    BeginResumable(String),

    /// The `clef` command requires the clef of a hymn be identified. This token legally appears only in hymn environments.
    Clef(String),

    /// The `define` command requires both a key and a value be set. These will set fields in the parser's preprocessor for use in `Include` tokens.
    Define(String, String),

    /// Indicates the end of a hymn or box.
    End,

    /// This token is generated only by the lexer when recoverable errors occur during lexing.
    Error(String),

    /// The `gabc` command requires GABC music and lyrics be provided.
    Gabc(String),

    /// The `gloria` command may take a tone to be printed in. If no tone is provided, it will default to the last used tone. If no tone has been used yet and no tone is provided in the command, it will be compiled into an error.
    Gloria(Option<String>),

    /// The `heading` and `subheading` commands require a text argument. The two commands respectively correspond to level `1` and `2` which will then be compiled into `h1` and `h2` elements.
    Heading(String, u8),

    /// Equivalent to `Include` but is transformed into an `Empty` token if the provided key has no corresponding value.
    IfInclude(String),

    /// The `import` command requires a relative path to another `.lit` or a `.gabc` file. In the first case, the file will be lexed, parsed, and compiled; in the second it will be transformed into a `RawGabc` directive.
    Import(String),

    /// The `include` command requires the name of a field. The preprocessor will find the corresponding value of this provided key which is a path to be supplied to an `Import` directive which will then follow the proper course as given above.
    Include(String),

    /// The `instruction` command requires a text to be printed as a rubric.
    Instruction(String),

    /// This token is generated by the lexer when it encounters a line which does not begin with '#'.
    Text(String),

    /// The `title` command requires a text to be printed as a small heading before chants.
    Title(String),

    /// The `tone` command requires the name of a tone be provided in the format "`TONE`-`ENDING`". It will be transformed into an `Import` directive for the corresponding value in `tone/TONE-ENDING.gabc` and set this as the last used tone as used by the `Gloria` and `Psalm` tokens.
    Tone(String),

    /// The `melody` command indicates the melody used in the Nth verse of a stanza. It legally occurs only in hymn environments.
    Melody(Vec<String>),

    /// The `no-gloria` command indicates that the next `Gloria` token encountered will be rendered as nothing.
    NoGloria,

    /// The `psalm` command requires the name of a psalm or other canticle to be included, which must correspond to the name of a file as `psalter/PSALM/TONE.lit`. It depends upon the last tone used to resolve and if none was used will be transformed into an error.
    Psalm(String),

    /// This token is generated by the parser preprocessor. It refers to a raw GABC file contents (whether real or imaginary).
    RawGabc(String),

    /// The `resume` command requires the name of a BeginResumable block which will be placed where this is called.
    Resume(String),

    /// The `repeat-antiphon` command indicates that the last used antiphon is now repeated but without its decorations.
    RepeatAntiphon,

    /// The `repeat antiphon "half"` command is used in invitatories to repeat only the latter half of the previous antiphon from a "+(;)" sequence.
    ///
    /// If the value is "half", it prints the half, if it is "full" it pritns the whole, it each case prepending the "R." symbol beforehand.
    RepeatHalfAntiphon(String),

    /// The `repeat-tone` command is added for convenience when many psalms occur under the same tone.
    RepeatTone,

    /// The `verse` command indicates the syllables in a verse of a stanza. It legally occurs only in hymn enviroments.
    Verse(Vec<String>),

    /// Indicates a token which has been deleted or transformed without meaning left.
    Empty,
}

/// The lexer transforms text into tokens which are to be fed into the parser.
pub struct Lexer {
    /// The lines with which this lexer has been initialized to transform into tokens.
    /// `Rc<RefCell>` is used as a container around the ExactSizeIterator trait which is used to allow token streaming.
    lines: Rc<RefCell<dyn ExactSizeIterator<Item = String>>>,
}

pub fn read_file<P>(path: P) -> Result<String, String>
where
    P: AsRef<std::path::Path> + std::fmt::Debug,
{
    let file = match crate::asset::Asset::get(&path.as_ref().to_string_lossy()) {
        Some(file) => file.data,
        None => return Err(format!("No such file exists: {:?}", path)),
    };

    match std::str::from_utf8(file.as_ref()) {
        Ok(string) => Ok(string.to_string()),
        Err(why) => Err(why.to_string()),
    }
}

/// newtype used for ease of writing
type LR<T> = Result<T, String>;

impl Lexer {
    /// Consumes the next line and transforms it into a Token. We can guarantee the result will be a Token since any errors which occur at this point should be exposed to the user.
    fn next_token(&mut self) -> Token {
        let line = match Rc::clone(&self.lines).borrow_mut().next() {
            Some(line) => line,
            None => return Token::Error("No lines left to read".into()), // This is only triggered when a lexer has been initialized on an empty file.
        };

        if !line.starts_with('#') {
            Token::Text(line)
        } else {
            self.parse_token(&line)
        }
    }

    /// if a line was not plaintext, parse the passed command.
    fn parse_token(&self, line: &String) -> Token {
        if let Some(captures) = WHOLE_RE.captures(&line) {
            let command = captures.get(1).map_or("", |m| m.as_str());

            let mut args = Vec::new();
            for arg in COMMAND_RE.find_iter(&line) {
                args.push((line[arg.start() + 1..arg.end() - 1]).to_string().clone());
            }

            match (command, args.len()) {
                ("amen", 2) => Token::Amen(args[0].to_owned(), args[1].to_owned()),

                ("antiphon", 1) => Token::Antiphon(args[0].to_owned(), None),

                ("antiphon", 2) => Token::Antiphon(args[0].to_owned(), Some(args[1].to_owned())),

                ("begin-box", 0) => Token::BeginBox,

                ("begin-hymn", 0) => Token::BeginHymn,

                ("begin-resumable", 1) => Token::BeginResumable(args[0].to_owned()),

                ("clef", 1) => Token::Clef(args[0].to_owned()),

                ("define", 2) => Token::Define(args[0].to_owned(), args[1].to_owned()),

                ("end" | "end-box" | "end-hymn", 0) => Token::End,

                ("gabc", 1) => Token::Gabc(args[0].to_owned()),

                ("gloria", 0) => Token::Gloria(None),
                ("gloria", 1) => Token::Gloria(Some(args[0].to_owned())),

                ("heading", 1) => Token::Heading(args[0].to_owned(), 1),
                ("subheading", 1) => Token::Heading(args[0].to_owned(), 2), // included here for ease of access
                // other headings could theoretically exist but do not at this time
                ("import", 1) => Token::Import(args[0].to_owned()),

                ("if-include", 1) => Token::IfInclude(args[0].to_owned()),

                ("include", 1) => Token::Include(args[0].to_owned()),

                ("instruction", 1) => Token::Instruction(args[0].to_owned()),

                // text is not included here since it is the default
                ("title", 1) => Token::Title(args[0].to_owned()),

                ("tone", 2) => Token::Tone(args[0].to_owned()),

                ("melody", _) => Token::Melody(args.clone()),

                ("no-gloria", 0) => Token::NoGloria,

                ("psalm", 1) => Token::Psalm(args[0].to_owned()),

                ("resume", 1) => Token::Resume(args[0].to_owned()),

                ("repeat-antiphon", 0) => Token::RepeatAntiphon,

                ("repeat-antiphon", 1) => Token::RepeatHalfAntiphon(args[0].to_owned()),

                ("repeat-tone", 0) => Token::RepeatTone,

                ("verse", _) => Token::Verse(args.clone()),

                _ => Token::Error(format!(
                    "Failed to parse line \"{}\": Unknown command \"{}\"",
                    line, command
                )),
            }
        } else {
            Token::Error(format!("Failed to parse line \"{}\": malformed", line))
        }
    }

    /// Initializes a `Lexer` from a provided path to a file.
    pub fn from_path<P>(path: P) -> LR<Lexer>
    where
        P: AsRef<Path> + std::fmt::Debug + Copy,
    {
        let content = read_file(path)?;

        let lines = content.lines();

        let mut res = Vec::new();
        for line in lines {
            let line = line.trim();
            if !line.is_empty() {
                res.push(line.to_string());
            }
        }

        Ok(Lexer {
            lines: Rc::new(RefCell::new(res.into_iter())),
        })
    }

    /// Transforms all lines into Tokens.
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while self.lines.borrow().len() > 0 {
            tokens.push(self.next_token());
        }

        tokens
    }
}
