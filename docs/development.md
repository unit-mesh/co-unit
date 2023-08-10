# Development

## Setup

https://qdrant.tech/documentation/quick-start/

```bash
docker pull qdrant/qdrant
```

Start:

```bash
docker run -p 6333:6333 \
    -v $(pwd)/qdrant_storage:/qdrant/storage \
    qdrant/qdrant
```



