pub fn functions(add_proc: bool) -> serde_json::Value {
    let mut funcs = serde_json::json!(
        [
            {
                "name": "code",
                "description":  "Search the contents of files in a codebase semantically. Results will not necessarily match search terms exactly, but should be related.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "query": {
                            "type": "string",
                            "description": "The query with which to search. This should consist of keywords that might match something in the codebase, e.g. 'react functional components', 'contextmanager', 'bearer token'"
                        }
                    },
                    "required": ["query"]
                }
            },
            {
                "name": "path",
                "description": "Search the pathnames in a codebase. Results may not be exact matches, but will be similar by some edit-distance. Use when you want to find a specific file or directory.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "query": {
                            "type": "string",
                            "description": "The query with which to search. This should consist of keywords that might match a path, e.g. 'server/src'."
                        }
                    },
                    "required": ["query"]
                }
            }
        ]);

    funcs
}