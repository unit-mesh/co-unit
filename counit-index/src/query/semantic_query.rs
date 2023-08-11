use std::borrow::Cow;
use std::collections::HashSet;
use crate::semantic::literal::Literal;

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
