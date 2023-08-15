# Co-unit

<p align="center">
  <img src="docs/counit.svg" width="128px" height="128px" />
</p>

> CoUnit: 是一个基于 LLM 的团队接口人（API），通过让大模型处理文档、知识库、SDK和 API 等矢量化，智能化团队间对接与协作，助力团队释放创造力。

features:

- chat with API, and design for API integration.
- design for new service.
- share for code copy.

## Development

1. install Qdrant by Docker:
```bash
docker pull qdrant/qdrant
docker run -p 6333:6333 qdrant/qdrant
```
2. Run CoUnit-Server

## APIs

### 1. Upload Data by ArchGuard

1. Download ArchGuard CLI (scanner_cli-2.x.x-all.jar) from: [https://github.com/archguard/archguard/releases]
2. Run ArchGuard CLI to upload data to Co-Unit:

```bash
Usage: runner [OPTIONS]

  scanner cli

Options:
  --type [SOURCE_CODE|GIT|DIFF_CHANGES|SCA|RULE|ARCHITECTURE|ESTIMATE|OPENAPI]
  --system-id TEXT                 system id
  --server-url TEXT                the base url of the archguard api server
  --workspace TEXT                 the workspace directory
  --path TEXT                      the path of target project
  --output TEXT                    http, csv, json, console
  --output-dir TEXT                output directory
  --analyser-spec TEXT             Override the analysers via json.
  --slot-spec TEXT                 Override the slot via json.
  --language TEXT                  language: Java, Kotlin, TypeScript, CSharp,
                                   Python, Golang.
  --rules TEXT                     rules: webapi, test, sql
  --features TEXT                  features: apicalls, datamap.
  --repo-id TEXT                   repository id used for git analysing
  --branch TEXT                    repository branch
  --started-at INT                 TIMESTAMP, the start date of the scanned
                                   commit
  --since TEXT                     COMMIT ID, the specific revision of the
                                   baseline
  --until TEXT                     COMMIT ID, the specific revision of the
                                   target
  --depth INT                      INTEGER, the max loop depth
  --with-function-code             BOOLEAN, whether to include the function
                                   code
  -h, --help                       Show this message and exit
```

For example:

```bash
java -jar scanner_cli-2.0.4-all.jar --language=Kotlin --path=your_path_to_code --server-url=http://localhost:8765 --repo-id="archguard" --with-function-code --output=http  --features=apicalls
```

OpenAPI example:

```bash
java -jar scanner_cli-2.0.4-all.jar --language=Kotlin --path=your_swagger_3_file --server-url=http://localhost:8765 --repo-id="payment" --output=http 
```

ArchGuard APIs:

```http request
### ArchGuard Code datastrcuture
POST http://127.0.0.1:8765/scanner/:systemId/reporting/class-items

### ArchGuard OpenAPI structure
POST http://127.0.0.1:8765/scanner/:systemId/reporting/openapi

### ArchGuard Service Datamap
POST http://127.0.0.1:8765/scanner/:systemId/reporting/container-services

### ArchGuard Datamap 
POST http://127.0.0.1:8765/scanner/:systemId/reporting/datamap-relations
```


### 2. Get Data by Co-Unit

### Query API

GET http://127.0.0.1:8765/api/query?q=create%20arch%20system&repo_ref=archguard&query_type=HttpApi

### 3. TEXT EMBEDDING

GET http://127.0.0.1:8765/api/text-embedding?q=create%20arch%20system


## License

The Co-Unit index is licensed under the Apache 2.0 license based
on [https://github.com/BloopAI/bloop](https://github.com/BloopAI/bloop) . See `LICENSE`
in [counit-index](./counit-index).

This code is distributed under the MPL 2.0 license. See `LICENSE` in this directory.
