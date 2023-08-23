use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Tool {
    name: &'static str,
    description: &'static str,
    schema: &'static str,
    examples: &'static str,
}

impl fmt::Display for Tool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}:", self.name)?;
        writeln!(f, "\tdescription: {}", self.description)?;
        writeln!(f, "\tschema: {}", self.schema)?;
        writeln!(f, "\texamples: {:?}", self.examples)?;
        Ok(())
    }
}

pub fn tools_list() -> [Tool; 4] {
    [
        Tool {
            name: "codeSearch",
            description: "Search the contents of files in a codebase semantically. Results will not necessarily match search terms exactly, but should be related.",
            schema: "{\"name\": \"code\", \"args\": [SEARCH_TERMS // str]}",
            examples: "[[\"code\", \"backend error types\"], [\"code\", \"react functional components\"]]"
        },
        Tool {
            name: "pathSearch",
            description: "Search the pathnames in a codebase. Results may not be exact matches, but will be similar by some edit-distance. Use when you want to find a specific file or directory.",
            schema: "{\"name\": \"path\", \"args\": [SEARCH_TERMS // str]}",
            examples: "[[\"path\", \"server/src\"], [\"path\", \".tsx\"], [\"path\", \"examples/android\"]]"
        },
        Tool {
            name: "processFiles",
            description: "Read one or more files and extract the line ranges which are relevant to the search terms.",
            schema: "{\"name\": \"proc\", \"args\": [SEARCH_TERMS // str, ARRAY_OF_PATH_ALIASES // int[]}",
            examples: "[[\"proc\", \"find all the functional react components\", [2,5]], [\"proc\", \"where are error types\", [0]], [\"proc\", \"gitoxide initialisation\", [2,5,8]]]"
        },
        Tool {
            name: "none",
            description: "You have enough information to answer the user's query. This is the final step, and signals that you have enough information to respond to the user's query. ARRAY_OF_PATH_ALIASES contains the aliases of the paths which are particularly relevant to the query.",
            schema: "{\"name\": \"none\", \"args\": [ARRAY_OF_PATH_ALIASES // int[]]}",
            examples: "[[\"none\", [1]], [\"none\", [3,5]], [\"none\", []]]"
        }]
}
