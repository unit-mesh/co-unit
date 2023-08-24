use crate::agent::tools;

pub const CONTINUE: &str = "Is there anything else I can help with?";

pub fn hypothetical_document_prompt(query: &str) -> String {
    format!(
        r#"Write a code snippet that could hypothetically be returned by a code search engine as the answer to the query: {query}

- Write the snippets in a programming or markup language that is likely given the query
- The snippet should be between 5 and 10 lines long
- Surround the snippet in triple backticks

For example:

What's the Qdrant threshold?

```rust
SearchPoints {{
    limit,
    vector: vectors.get(idx).unwrap().clone(),
    collection_name: COLLECTION_NAME.to_string(),
    offset: Some(offset),
    score_threshold: Some(0.3),
    with_payload: Some(WithPayloadSelector {{
        selector_options: Some(with_payload_selector::SelectorOptions::Enable(true)),
    }}),
```"#
    )
}


pub fn tool_prompt(paths: &Vec<String>) -> String {
    let tools = tools::tools_list();

    let mut s = "Your job is to answer a question about a codebase. You should use a set of tools to gather information that will help you answer. The following tools are available:\n\n".to_string();
    for tool in tools.iter() {
        s.push_str(&format!("{}\n", tool));
    }

    if !paths.is_empty() {
        s.push_str("## PATHS ##\nalias, path\n");
        for (i, path) in paths.iter().enumerate() {
            s.push_str(&format!("{}, {}\n", i, path));
        }
    }

    s.push_str(
        r#"
Follow these rules at all times:

- If the output of a tool is empty, try the same tool again with different arguments or try using a different tool
- In most cases you'll have to use codeSearch or pathSearch before using 'none'
- Respect action arg types, only types with brackets [] can be used as lists
- Do not assume the structure of the codebase, or the existence of files or folders
- Do NOT use a tool that you've used before with the same arguments
- To perform multiple actions, perform just one, wait for the response, then perform the next
- When you are confident that you have enough information needed to answer the query, choose 'none'
- If you have been instructed to modify the codebase choose 'none'
- If after making a path search the query can be answered by the existance of the paths, and there are more than 5 paths, choose 'none'
- Only refer to path aliases that are under the PATHS heading above
- Use the tools to find information related to the query, until all relevant information has been found.
- If after attempting to gather information you are still unsure how to answer the query, choose 'none'
- Always respond according to the schema of the tool that you want to use
- Output a list of [name, *args] to use a tool. For example to use codeSearch, output: ["code","my search query"]. To use processFiles, output: ["proc", "how does X work", [3,6]]
- Do NOT answer the user's query directly. You MUST use one of the tools above

"#);
    s
}

pub fn file_explanation(question: &str, path: &str, code: &str) -> String {
    format!(
        r#"Below are some lines from the file /{path}. Each line is numbered.

#####

{code}

#####

Your job is to perform the following tasks:
1. Find all the relevant line ranges of code.
2. DO NOT cite line ranges that you are not given above
3. You MUST answer with only line ranges. DO NOT answer the question

Q: find Kafka auth keys
A: [[12,15]]

Q: find where we submit payment requests
A: [[37,50]]

Q: auth code expiration
A: [[486,501],[520,560],[590,631]]

Q: library matrix multiplication
A: [[68,74],[82,85],[103,107],[187,193]]

Q: how combine result streams
A: []

Q: {question}
A: "#
    )
}

pub fn final_explanation_prompt(context: &str, query: &str, query_history: &str) -> String {
    struct Rule<'a> {
        title: &'a str,
        description: &'a str,
        note: &'a str,
        schema: &'a str,
        example: Option<&'a str>,
    }

    let rules = [
        Rule {
            title: "Cite a line range from a file",
            description: "COMMENT should refer to the code in in the START LINE and END LINE range. The COMMENT should answer the query with respect to the given line range. It should NOT include information that is not in the code. If the code does not help answer the query, then do not include it in a citation.",
            schema: "[\"cite\",PATH ALIAS:INT,COMMENT:STRING,START LINE:INT,END LINE:INT]",
            note: "This object can occur multiple times",
            example: None,
        },
        Rule {
            title: "Cite a single directory from the codebase",
            description: "When you wish to cite every file in a directory, use this to directly cite the directory instead. The COMMENT should answer the query with respect to the given directory.",
            schema: "[\"dir\",PATH:STRING,COMMENT:STRING]",
            note: "This object can occur multiple times",
            example: Some(r#"The path is a relative path, with no leading slash. You must generate a trailing slash, for example: server/bleep/src/webserver/. On Windows, generate backslash separated components, for example: server\bleep\src\webserver\"#),
        },
        Rule {
            title: "Write a new code file",
            description: "Write a new code file that satisfies the query. Do not use this to demonstrate updating an existing file.",
            schema: "[\"new\",LANGUAGE:STRING,CODE:STRING]",
            note: "This object can occur multiple times",
            example: None,
        },
        Rule {
            title: "Update the code in an existing file",
            description: "Edit an existing code file by generating the diff between old and new versions. Changes should be as small as possible.",
            schema: "[\"mod\",PATH ALIAS:INT,LANGUAGE:STRING,GIT DIFF:STRING]",
            note: "This object can occur multiple times",
            example: Some(r#"Where GIT DIFF describes the diff chunks for the file, including the git diff header.
For example:
@@ -1 +1 @@
-this is a git diff test example
+this is a diff example"#),
        },
        Rule {
            title: "Cite line ranges from the file",
            description: "START LINE and END LINE should focus on the code mentioned in the COMMENT. COMMENT should be a detailed explanation.",
            schema: "[\"cite\",PATH ALIAS:INT,COMMENT:STRING,START LINE:INT,END LINE:INT]",
            note: "This object can occur multiple times",
            example: None,
        },
        Rule {
            title: "Conclusion",
            description: "Summarise your previous steps. Provide as much information as is necessary to answer the query. If you do not have enough information needed to answer the query, do not make up an answer.",
            schema: "[\"con\",SUMMARY:STRING]",
            note: "This is mandatory and must appear once at the end",
            example: None,
        },
    ];

    let output_rules_str = rules
        .into_iter()
        .zip(1..)
        .map(|(r, i)| {
            let Rule {
                title,
                description,
                schema,
                note,
                example,
                ..
            } = r;
            format!(
                "{i}. {title}\n{description}\n{schema}\n{note}\n{}\n",
                example.unwrap_or("")
            )
        })
        .collect::<String>();

    format!(
        r#"{context}Your job is to answer a query about a codebase using the information above. 
Your answer should be an array of arrays, where each element in the array is an instance of one of the following objects:

{output_rules_str}
Respect these rules at all times:
- Refer to directories by their full paths, surrounded by single backticks
- Your answer should always be an array of arrays, even when you only generate a conclusion

#####

Examples:

Show all the analytics events

[
  ["cite", 27, "Track 'Search' event in useAnalytics.ts", 7, 12],
  ["con", "I've found three analytics events"]
]

Where is the webserver code located

[
  ["dir","server/bleep/src/webserver/","This directory contains the webserver module"],
  ["con","The webserver code is located under the server directory"]
]

What's the value of MAX_FILE_LEN?

[
  ["con": "None of files in the context contain the value of MAX_FILE_LEN"]
]

#####

{query_history}

Above is the query and answer history. The user can see the previous queries and answers on their screen, but not anything else.
Based on this history, answer the question: {query}

#####

Output only JSON."#
    )
}
