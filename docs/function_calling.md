## Function Calling example

```json
[
  {
    "name": "code",
    "description": "Search the contents of files in a codebase semantically. Results will not necessarily match search terms exactly, but should be related.",
    "parameters": {
      "type": "object",
      "properties": {
        "query": {
          "type": "string",
          "description": "The query with which to search. This should consist of keywords that might match something in the codebase, e.g. 'react functional components', 'contextmanager', 'bearer token'"
        }
      },
      "required": [
        "query"
      ]
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
      "required": [
        "query"
      ]
    }
  },
  {
    "name": "none",
    "description": "You have enough information to answer the user's query. This is the final step, and signals that you have enough information to respond to the user's query. Use this if the user has instructed you to modify some code.",
    "parameters": {
      "type": "object",
      "properties": {
        "paths": {
          "type": "array",
          "items": {
            "type": "integer",
            "description": "The indices of the paths to answer with respect to. Can be empty if the answer is not related to a specific path."
          }
        }
      },
      "required": [
        "paths"
      ]
    }
  }
]
```

## Explain code

```rust
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
```