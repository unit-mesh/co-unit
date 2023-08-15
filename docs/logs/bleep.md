2023-08-15T01:32:05.834132Z DEBUG bleep::query::execute: compiled query as [Query { open: Some(true), case_sensitive: None, global_regex: Some(false), org: None, repo: Some(Plain("github.com/phodal/coca")), path: None, lang: None, branch: Some(Plain("origin/master")), target: None }]
2023-08-15T01:32:05.967708Z DEBUG bleep::query::execute: compiled query as [Query { open: Some(true), case_sensitive: None, global_regex: Some(false), org: None, repo: Some(Plain("github.com/phodal/coca")), path: Some(Plain("README.md")), lang: None, branch: Some(Plain("origin/master")), target: None }]
2023-08-15T01:32:13.345845Z DEBUG bleep::agent: executing next action action=Query("How to analyser Go lang ?") self.thread_id=81b8d023-c8f5-464d-b3a4-bebd4c7a5813
2023-08-15T01:32:15.073106Z  INFO bleep::analytics: sent analytics event...
2023-08-15T01:32:18.123093Z  INFO bleep::analytics: sent analytics event...
2023-08-15T01:32:18.123141Z DEBUG bleep::agent: executing next action action=Code { query: "Go lang analyzer" } self.thread_id=81b8d023-c8f5-464d-b3a4-bebd4c7a5813
2023-08-15T01:32:18.123154Z DEBUG bleep::agent: executing semantic query query=SemanticQuery { repos: {Plain("phodal/coca")}, paths: {}, langs: {}, branch: {Plain("origin/master")}, target: Some(Plain("Go lang analyzer")) } self.thread_id=81b8d023-c8f5-464d-b3a4-bebd4c7a5813
2023-08-15T01:32:20.597999Z  INFO bleep::agent::tools::code: got hyde doc doc="go\npackage main\n\nimport (\n\t\"fmt\"\n\t\"go/ast\"\n\t\"go/parser\"\n\t\"go/token\"\n)\n\nfunc main() {\n\tsrc := `\n\tpackage main\n\n\timport \"fmt\"\n\n\tfunc main() {\n\t\tfmt.Println(\"Hello, world!\")\n\t}\n\t`\n\n\tfset := token.NewFileSet()\n\tf, err := parser.ParseFile(fset, \"\", src, 0)\n\tif err != nil {\n\t\tfmt.Println(err)\n\t\treturn\n\t}\n\n\tast.Print(fset, f)\n}"
2023-08-15T01:32:20.598043Z DEBUG bleep::agent: executing semantic query query=SemanticQuery { repos: {Plain("phodal/coca")}, paths: {}, langs: {}, branch: {Plain("origin/master")}, target: Some(Plain("go\npackage main\n\nimport (\n\t\"fmt\"\n\t\"go/ast\"\n\t\"go/parser\"\n\t\"go/token\"\n)\n\nfunc main() {\n\tsrc := `\n\tpackage main\n\n\timport \"fmt\"\n\n\tfunc main() {\n\t\tfmt.Println(\"Hello, world!\")\n\t}\n\t`\n\n\tfset := token.NewFileSet()\n\tf, err := parser.ParseFile(fset, \"\", src, 0)\n\tif err != nil {\n\t\tfmt.Println(err)\n\t\treturn\n\t}\n\n\tast.Print(fset, f)\n}")) } self.thread_id=81b8d023-c8f5-464d-b3a4-bebd4c7a5813
2023-08-15T01:32:21.223306Z  INFO bleep::analytics: sent analytics event...
2023-08-15T01:32:27.048310Z  INFO bleep::analytics: sent analytics event...
2023-08-15T01:32:27.048415Z DEBUG bleep::agent: executing next action action=Proc { query: "Go lang analyzer", paths: [1, 2, 3, 4, 7, 8, 14] } self.thread_id=81b8d023-c8f5-464d-b3a4-bebd4c7a5813
2023-08-15T01:32:27.048448Z DEBUG bleep::agent::tools::proc: invoking proc query="Go lang analyzer" paths=["pkg/application/evaluate/analyser_test.go", "analysis/golang/app/analysis.go", "analysis/golang/main.go", "languages/g4/GoLexer.g4", "languages/g4/GoParser.g4", "pkg/infrastructure/ast/ast_go/cocago_builder.go", "pkg/infrastructure/ast/ast_go/cocago_parser.go"]
2023-08-15T01:32:27.048493Z DEBUG bleep::agent::tools::proc: reading file path="pkg/application/evaluate/analyser_test.go"
2023-08-15T01:32:27.048504Z DEBUG bleep::agent: executing file search self.repo_ref=github.com/phodal/coca path="pkg/application/evaluate/analyser_test.go" branch=Some("origin/master") self.thread_id=81b8d023-c8f5-464d-b3a4-bebd4c7a5813
2023-08-15T01:32:27.114098Z DEBUG bleep::agent::tools::proc: reading file path="analysis/golang/app/analysis.go"
2023-08-15T01:32:27.114118Z DEBUG bleep::agent: executing file search self.repo_ref=github.com/phodal/coca path="analysis/golang/app/analysis.go" branch=Some("origin/master") self.thread_id=81b8d023-c8f5-464d-b3a4-bebd4c7a5813
2023-08-15T01:32:27.173194Z DEBUG bleep::agent::tools::proc: reading file path="analysis/golang/main.go"
2023-08-15T01:32:27.173212Z DEBUG bleep::agent: executing file search self.repo_ref=github.com/phodal/coca path="analysis/golang/main.go" branch=Some("origin/master") self.thread_id=81b8d023-c8f5-464d-b3a4-bebd4c7a5813
2023-08-15T01:32:27.233467Z DEBUG bleep::agent::tools::proc: reading file path="languages/g4/GoLexer.g4"
2023-08-15T01:32:27.233488Z DEBUG bleep::agent: executing file search self.repo_ref=github.com/phodal/coca path="languages/g4/GoLexer.g4" branch=Some("origin/master") self.thread_id=81b8d023-c8f5-464d-b3a4-bebd4c7a5813
2023-08-15T01:32:27.295269Z DEBUG bleep::agent::tools::proc: reading file path="languages/g4/GoParser.g4"
2023-08-15T01:32:27.295290Z DEBUG bleep::agent: executing file search self.repo_ref=github.com/phodal/coca path="languages/g4/GoParser.g4" branch=Some("origin/master") self.thread_id=81b8d023-c8f5-464d-b3a4-bebd4c7a5813
2023-08-15T01:32:27.354566Z DEBUG bleep::agent::tools::proc: reading file path="pkg/infrastructure/ast/ast_go/cocago_builder.go"
2023-08-15T01:32:27.354586Z DEBUG bleep::agent: executing file search self.repo_ref=github.com/phodal/coca path="pkg/infrastructure/ast/ast_go/cocago_builder.go" branch=Some("origin/master") self.thread_id=81b8d023-c8f5-464d-b3a4-bebd4c7a5813
2023-08-15T01:32:27.415270Z DEBUG bleep::agent::tools::proc: reading file path="pkg/infrastructure/ast/ast_go/cocago_parser.go"
2023-08-15T01:32:27.415290Z DEBUG bleep::agent: executing file search self.repo_ref=github.com/phodal/coca path="pkg/infrastructure/ast/ast_go/cocago_parser.go" branch=Some("origin/master") self.thread_id=81b8d023-c8f5-464d-b3a4-bebd4c7a5813
2023-08-15T01:32:27.473759Z DEBUG bleep::agent::tools::proc: calling chat API on file path="pkg/application/evaluate/analyser_test.go"
2023-08-15T01:32:27.473823Z DEBUG bleep::agent::tools::proc: calling chat API on file path="analysis/golang/app/analysis.go"
2023-08-15T01:32:27.473899Z DEBUG bleep::agent::tools::proc: calling chat API on file path="analysis/golang/main.go"
2023-08-15T01:32:27.474012Z DEBUG bleep::agent::tools::proc: calling chat API on file path="languages/g4/GoLexer.g4"
2023-08-15T01:32:27.474176Z DEBUG bleep::agent::tools::proc: calling chat API on file path="languages/g4/GoParser.g4"
2023-08-15T01:32:29.023902Z DEBUG bleep::agent::tools::proc: calling chat API on file path="pkg/infrastructure/ast/ast_go/cocago_builder.go"
2023-08-15T01:32:29.679472Z DEBUG bleep::agent::tools::proc: calling chat API on file path="pkg/infrastructure/ast/ast_go/cocago_parser.go"
2023-08-15T01:32:32.152724Z  INFO bleep::analytics: sent analytics event...
2023-08-15T01:32:36.923800Z  INFO bleep::analytics: sent analytics event...
2023-08-15T01:32:36.923880Z DEBUG bleep::agent: executing next action action=Answer { paths: [1, 2, 3, 4, 7, 8, 14] } self.thread_id=81b8d023-c8f5-464d-b3a4-bebd4c7a5813
2023-08-15T01:32:36.923890Z DEBUG bleep::agent::tools::answer: creating article response aliases=[1, 2, 3, 4, 7, 8, 14]
2023-08-15T01:32:36.923897Z DEBUG bleep::agent::tools::answer: created filtered path alias list paths=["go.sum", "pkg/application/evaluate/analyser_test.go", "analysis/golang/app/analysis.go", "analysis/golang/main.go", "languages/g4/GoLexer.g4", "pkg/application/analysis/common_analysis.go", "analysis/dep/main.go", "languages/g4/GoParser.g4", "pkg/infrastructure/ast/ast_go/cocago_builder.go", "cmd/cmd_util/file_rw_helper.go", "languages/ts/typescript_base_parser.go", "languages/python/python_base_lexer.go", "pkg/infrastructure/ast/ast_go/cocago_parser_test.go", "analysis/java/main.go", "pkg/infrastructure/ast/ast_go/cocago_parser.go"] aliases=[1, 2, 3, 4, 7, 8, 14]
2023-08-15T01:32:36.923919Z DEBUG bleep::agent::tools::answer: canonicalizing code chunks aliases=[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]
2023-08-15T01:32:36.978129Z DEBUG bleep::agent::tools::answer: expanding spans spans_by_path={"analysis/golang/main.go": [1..13, 1..6], "pkg/infrastructure/ast/ast_go/cocago_parser_test.go": [135..161], "pkg/infrastructure/ast/ast_go/cocago_builder.go": [133..159, 1..31, 62..135, 153..173, 188..208, 236..284], "analysis/golang/app/analysis.go": [45..59, 24..38], "languages/ts/typescript_base_parser.go": [1..30], "pkg/infrastructure/ast/ast_go/cocago_parser.go": [309..333, 1..205], "analysis/java/main.go": [1..13], "languages/python/python_base_lexer.go": [1..31], "analysis/dep/main.go": [1..13], "pkg/application/analysis/common_analysis.go": [1..23, 1..23], "languages/g4/GoParser.g4": [28..77, 31..34], "languages/g4/GoLexer.g4": [28..68, 28..68, 31..34], "cmd/cmd_util/file_rw_helper.go": [1..24], "go.sum": [138..141, 122..125, 27..31, 116..119], "pkg/application/evaluate/analyser_test.go": [1..21, 1..112]}
2023-08-15T01:32:36.978170Z DEBUG bleep::agent: executing file search self.repo_ref=github.com/phodal/coca path="analysis/golang/main.go" branch=Some("origin/master") self.thread_id=81b8d023-c8f5-464d-b3a4-bebd4c7a5813
2023-08-15T01:32:36.979827Z DEBUG bleep::agent: executing file search self.repo_ref=github.com/phodal/coca path="pkg/infrastructure/ast/ast_go/cocago_parser_test.go" branch=Some("origin/master") self.thread_id=81b8d023-c8f5-464d-b3a4-bebd4c7a5813
2023-08-15T01:32:36.981255Z DEBUG bleep::agent: executing file search self.repo_ref=github.com/phodal/coca path="pkg/infrastructure/ast/ast_go/cocago_builder.go" branch=Some("origin/master") self.thread_id=81b8d023-c8f5-464d-b3a4-bebd4c7a5813
2023-08-15T01:32:36.982681Z DEBUG bleep::agent: executing file search self.repo_ref=github.com/phodal/coca path="analysis/golang/app/analysis.go" branch=Some("origin/master") self.thread_id=81b8d023-c8f5-464d-b3a4-bebd4c7a5813
2023-08-15T01:32:36.983995Z DEBUG bleep::agent: executing file search self.repo_ref=github.com/phodal/coca path="languages/ts/typescript_base_parser.go" branch=Some("origin/master") self.thread_id=81b8d023-c8f5-464d-b3a4-bebd4c7a5813
2023-08-15T01:32:36.985208Z DEBUG bleep::agent: executing file search self.repo_ref=github.com/phodal/coca path="pkg/infrastructure/ast/ast_go/cocago_parser.go" branch=Some("origin/master") self.thread_id=81b8d023-c8f5-464d-b3a4-bebd4c7a5813
2023-08-15T01:32:36.986530Z DEBUG bleep::agent: executing file search self.repo_ref=github.com/phodal/coca path="analysis/java/main.go" branch=Some("origin/master") self.thread_id=81b8d023-c8f5-464d-b3a4-bebd4c7a5813
2023-08-15T01:32:36.987418Z DEBUG bleep::agent: executing file search self.repo_ref=github.com/phodal/coca path="languages/python/python_base_lexer.go" branch=Some("origin/master") self.thread_id=81b8d023-c8f5-464d-b3a4-bebd4c7a5813
2023-08-15T01:32:36.988472Z DEBUG bleep::agent: executing file search self.repo_ref=github.com/phodal/coca path="analysis/dep/main.go" branch=Some("origin/master") self.thread_id=81b8d023-c8f5-464d-b3a4-bebd4c7a5813
2023-08-15T01:32:36.989190Z DEBUG bleep::agent: executing file search self.repo_ref=github.com/phodal/coca path="pkg/application/analysis/common_analysis.go" branch=Some("origin/master") self.thread_id=81b8d023-c8f5-464d-b3a4-bebd4c7a5813
2023-08-15T01:32:36.990378Z DEBUG bleep::agent: executing file search self.repo_ref=github.com/phodal/coca path="languages/g4/GoParser.g4" branch=Some("origin/master") self.thread_id=81b8d023-c8f5-464d-b3a4-bebd4c7a5813
2023-08-15T01:32:36.991545Z DEBUG bleep::agent: executing file search self.repo_ref=github.com/phodal/coca path="languages/g4/GoLexer.g4" branch=Some("origin/master") self.thread_id=81b8d023-c8f5-464d-b3a4-bebd4c7a5813
2023-08-15T01:32:36.992799Z DEBUG bleep::agent: executing file search self.repo_ref=github.com/phodal/coca path="cmd/cmd_util/file_rw_helper.go" branch=Some("origin/master") self.thread_id=81b8d023-c8f5-464d-b3a4-bebd4c7a5813
2023-08-15T01:32:36.993737Z DEBUG bleep::agent: executing file search self.repo_ref=github.com/phodal/coca path="go.sum" branch=Some("origin/master") self.thread_id=81b8d023-c8f5-464d-b3a4-bebd4c7a5813
2023-08-15T01:32:36.994519Z DEBUG bleep::agent: executing file search self.repo_ref=github.com/phodal/coca path="pkg/application/evaluate/analyser_test.go" branch=Some("origin/master") self.thread_id=81b8d023-c8f5-464d-b3a4-bebd4c7a5813
2023-08-15T01:32:37.007209Z DEBUG bleep::agent::tools::answer: expanded spans spans_by_path={"analysis/golang/main.go": [1..13], "pkg/infrastructure/ast/ast_go/cocago_parser_test.go": [135..161], "pkg/infrastructure/ast/ast_go/cocago_builder.go": [1..31, 62..173, 188..208, 236..284], "analysis/golang/app/analysis.go": [24..38, 45..59], "languages/ts/typescript_base_parser.go": [1..30], "pkg/infrastructure/ast/ast_go/cocago_parser.go": [1..205, 309..333], "analysis/java/main.go": [1..13], "languages/python/python_base_lexer.go": [1..31], "analysis/dep/main.go": [1..13], "pkg/application/analysis/common_analysis.go": [1..23], "languages/g4/GoParser.g4": [28..77], "languages/g4/GoLexer.g4": [28..68], "cmd/cmd_util/file_rw_helper.go": [1..24], "go.sum": [27..31, 116..119, 122..125, 138..141], "pkg/application/evaluate/analyser_test.go": [1..112]}
2023-08-15T01:32:37.152522Z DEBUG bleep::agent::tools::answer: 7024
2023-08-15T01:32:37.152669Z DEBUG bleep::agent::tools::answer: 6844
2023-08-15T01:32:37.152799Z DEBUG bleep::agent::tools::answer: 6654
2023-08-15T01:32:37.152931Z DEBUG bleep::agent::tools::answer: 6467
2023-08-15T01:32:37.153091Z DEBUG bleep::agent::tools::answer: 6224
2023-08-15T01:32:37.153296Z DEBUG bleep::agent::tools::answer: 6038
2023-08-15T01:32:37.153648Z DEBUG bleep::agent::tools::answer: 5765
2023-08-15T01:32:37.153958Z DEBUG bleep::agent::tools::answer: 5482
2023-08-15T01:32:37.154177Z DEBUG bleep::agent::tools::answer: 5283
2023-08-15T01:32:37.154260Z DEBUG bleep::agent::tools::answer: 5208
2023-08-15T01:32:37.154495Z DEBUG bleep::agent::tools::answer: 4984
2023-08-15T01:32:37.154574Z DEBUG bleep::agent::tools::answer: 4911
2023-08-15T01:32:37.154815Z DEBUG bleep::agent::tools::answer: 4684
2023-08-15T01:32:37.156542Z DEBUG bleep::agent::tools::answer: 2948
2023-08-15T01:32:37.156748Z DEBUG bleep::agent::tools::answer: 2746
2023-08-15T01:32:37.156929Z DEBUG bleep::agent::tools::answer: 2572
2023-08-15T01:32:37.157093Z DEBUG bleep::agent::tools::answer: Breaking at 2572 tokens...
2023-08-15T01:33:09.341626Z  INFO bleep::analytics: sent analytics event...
2023-08-15T01:33:09.341923Z  INFO bleep::webserver::answer::conversations: writing conversation phodal-81b8d023-c8f5-464d-b3a4-bebd4c7a5813
