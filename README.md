# Co-unit

> Co-unit: The ultimate cross-project collaborator 🤝🚀, adding a dash of smarts to your teamwork! Get ready for
> intelligent insights and decision-making magic ✨🧠!


features:

- chat with API, and design for API integration.
- design for new service.
- share for code copy.

## APIs

### TEXT EMBEDDING

GET http://127.0.0.1:8765/api/text-embedding?q="create arch system"

### Query API

GET http://127.0.0.1:8765/api/query?q="create arch system"&repo_ref=archguard&query_type=HttpApi

### Upload Data by ArchGuard

POST http://127.0.0.1:8765/scanner/:systemId/reporting/class-items

POST http://127.0.0.1:8765/scanner/:systemId/reporting/openapi

POST http://127.0.0.1:8765/scanner/:systemId/reporting/container-services

POST http://127.0.0.1:8765/scanner/:systemId/reporting/datamap-relations


## License

The Co-Unit index is licensed under the Apache 2.0 license based
on [https://github.com/BloopAI/bloop](https://github.com/BloopAI/bloop) . See `LICENSE`
in [counit-index](./counit-index).

This code is distributed under the MPL 2.0 license. See `LICENSE` in this directory.
