use std::borrow::Cow;
use std::collections::HashSet;
use regex::Regex;


#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub enum Literal<'a> {
    Plain(Cow<'a, str>),
    Regex(Cow<'a, str>),
}
impl From<&String> for Literal<'static> {
    fn from(value: &String) -> Self {
        Literal::Plain(value.to_owned().into())
    }
}

impl<'a> Default for Literal<'a> {
    fn default() -> Self {
        Self::Plain(Cow::Borrowed(""))
    }
}

impl<'a> Literal<'a> {
    fn join_as_regex(self, rhs: Self) -> Self {
        let lhs = self.regex_str();
        let rhs = rhs.regex_str();
        Self::Regex(Cow::Owned(format!("{lhs}\\s+{rhs}")))
    }

    fn join_as_plain(self, rhs: Self) -> Option<Self> {
        let lhs = self.as_plain()?;
        let rhs = rhs.as_plain()?;
        Some(Self::Plain(Cow::Owned(format!("{lhs} {rhs}"))))
    }

    /// Convert this literal into a regex string.
    ///
    /// If this literal is a regex, it is returned as-is. If it is a plain text literal, it is
    /// escaped first before returning.
    pub fn regex_str(&self) -> Cow<'a, str> {
        match self {
            Self::Plain(text) => regex::escape(text).into(),
            Self::Regex(r) => r.clone(),
        }
    }

    pub fn regex(&self) -> Result<Regex, regex::Error> {
        Regex::new(&self.regex_str())
    }

    pub fn as_plain(&self) -> Option<Cow<'a, str>> {
        match self {
            Self::Plain(p) => Some(p.clone()),
            Self::Regex(..) => None,
        }
    }

    /// Force this literal into the `Regex` variant.
    fn make_regex(&mut self) {
        *self = match std::mem::take(self) {
            Self::Plain(s) | Self::Regex(s) => Self::Regex(s),
        }
    }

    pub fn unwrap(self) -> Cow<'a, str> {
        match self {
            Literal::Plain(v) => v,
            Literal::Regex(v) => v,
        }
    }

    pub fn into_owned(self) -> Literal<'static> {
        match self {
            Literal::Plain(cow) => Literal::Plain(Cow::Owned(cow.into_owned())),
            Literal::Regex(cow) => Literal::Regex(Cow::Owned(cow.into_owned())),
        }
    }
}

// impl<'a> From<Pair<'a, Rule>> for Literal<'a> {
//     fn from(pair: Pair<'a, Rule>) -> Self {
//         match pair.as_rule() {
//             Rule::unquoted_literal => Self::Plain(pair.as_str().trim().into()),
//             Rule::quoted_literal => Self::Plain(unescape(pair.as_str(), '"').into()),
//             Rule::single_quoted_literal => Self::Plain(unescape(pair.as_str(), '\'').into()),
//             Rule::regex_quoted_literal => Self::Regex(unescape(pair.as_str(), '/').into()),
//             Rule::raw_text => Self::Plain(pair.as_str().trim().into()),
//             _ => unreachable!(),
//         }
//     }
// }

#[derive(Default, Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SemanticQuery<'a> {
    pub repos: HashSet<Literal<'a>>,
    pub paths: HashSet<Literal<'a>>,
    pub langs: HashSet<Cow<'a, str>>,
    pub branch: HashSet<Literal<'a>>,
    pub target: Option<Literal<'a>>,
}

impl<'a> SemanticQuery<'a> {
    pub fn repos(&'a self) -> impl Iterator<Item=Cow<'a, str>> {
        self.repos.iter().filter_map(|t| t.as_plain())
    }

    pub fn paths(&'a self) -> impl Iterator<Item=Cow<'a, str>> {
        self.paths.iter().filter_map(|t| t.as_plain())
    }

    pub fn langs(&'a self) -> impl Iterator<Item=Cow<'a, str>> {
        self.langs.iter().cloned()
    }

    pub fn target(&self) -> Option<Cow<'a, str>> {
        self.target.as_ref().and_then(|t| t.as_plain())
    }

    pub fn branch(&'a self) -> impl Iterator<Item=Cow<'a, str>> {
        self.branch.iter().filter_map(|t| t.as_plain())
    }

    // TODO (@calyptobai): This is a quirk of the current conversation logic. We take only the
    // first branch because the UX operates on a single "current" branch. We can likely update
    // `SemanticQuery` to remove multiple branches altogether.
    pub fn first_branch(&self) -> Option<Cow<'_, str>> {
        self.branch.iter().next().map(|t| t.clone().unwrap())
    }

    pub fn from_str(query: String, repo_ref: String) -> Self {
        Self {
            target: Some(Literal::Plain(Cow::Owned(query))),
            repos: [Literal::Plain(Cow::Owned(repo_ref))].into(),
            ..Default::default()
        }
    }

    pub fn into_owned(self) -> SemanticQuery<'static> {
        SemanticQuery {
            repos: self.repos.into_iter().map(Literal::into_owned).collect(),
            paths: self.paths.into_iter().map(Literal::into_owned).collect(),
            langs: self
                .langs
                .into_iter()
                .map(|c| c.into_owned().into())
                .collect(),
            branch: self.branch.into_iter().map(Literal::into_owned).collect(),
            target: self.target.map(Literal::into_owned),
        }
    }
}
