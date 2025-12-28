# REASONKIT-MEM PROJECT CONTEXT

> Memory & Retrieval Infrastructure for ReasonKit ("Hippocampus")
> "The Long-Term Memory Layer"

**LICENSE:** Apache 2.0
**STATUS:** Active Development (extracted from reasonkit-core)
**REPOSITORY:** https://github.com/ReasonKit/reasonkit-mem

---

## WHAT REASONKIT-MEM IS

**ReasonKit-Mem is the memory and retrieval infrastructure layer for ReasonKit.**

It provides:
- **Vector Storage**: Qdrant-based dense vector storage with embedded mode
- **Hybrid Search**: Dense (Qdrant) + Sparse (Tantivy BM25) fusion
- **RAPTOR Trees**: Hierarchical retrieval for long-form QA
- **Embeddings**: Local (BGE-M3) and remote (OpenAI) embedding support
- **Reranking**: Cross-encoder reranking for precision

### Architecture

```
reasonkit-mem/
├── src/
│   ├── storage/      # Qdrant vector + file-based storage
│   ├── embedding/    # Dense vector embeddings (BGE-M3, OpenAI)
│   ├── retrieval/    # Hybrid search, fusion, reranking
│   ├── raptor/       # RAPTOR hierarchical tree structure
│   ├── indexing/     # BM25/Tantivy sparse indexing
│   ├── rag/          # RAG pipeline orchestration
│   ├── types.rs      # Shared types (Document, Chunk, etc.)
│   └── error.rs      # MemError types
└── Cargo.toml
```

---

## SEPARATION FROM REASONKIT-CORE

**reasonkit-core** = PURE REASONING ENGINE (ThinkTools, protocols, evaluation)
**reasonkit-mem** = MEMORY INFRASTRUCTURE (storage, retrieval, embeddings, RAPTOR)

This separation allows:
- Independent scaling of memory vs reasoning
- Clear architectural boundaries
- Potential separate release/versioning

---

## TECHNOLOGY STACK

| Component | Technology | Purpose |
|-----------|------------|---------|
| **Qdrant** | qdrant-client 1.10+ | Dense vector storage |
| **Tantivy** | tantivy 0.22+ | BM25 sparse search |
| **RAPTOR** | Custom Rust | Hierarchical retrieval |
| **Embeddings** | BGE-M3 / OpenAI | Dense representations |
| **Reranking** | Cross-encoder | Final precision boost |

---

## HYBRID SEARCH ARCHITECTURE

```
Query → [Dense Encoder] → Qdrant ANN Search → Top-K Dense
      → [BM25 Tokenizer] → Tantivy Search → Top-K Sparse
                               ↓
                    [Reciprocal Rank Fusion]
                               ↓
                    [Cross-Encoder Rerank]
                               ↓
                         Final Results
```

---

## USAGE

```rust
use reasonkit_mem::{
    storage::{Storage, EmbeddedStorageConfig},
    embedding::EmbeddingProvider,
    retrieval::HybridRetriever,
    Document, RetrievalConfig,
};

// Create storage backend
let config = EmbeddedStorageConfig::default();
let storage = Storage::new_embedded(config).await?;

// Index documents
storage.store_document(&doc).await?;

// Hybrid search
let retriever = HybridRetriever::new(storage.clone());
let results = retriever.search("query", &RetrievalConfig::default()).await?;
```

---

## QUALITY STATUS

| Gate | Status | Notes |
|------|--------|-------|
| **Build** | ✅ PASS | `cargo build --release` |
| **Clippy** | ✅ PASS | No warnings |
| **Tests** | ✅ PASS | 49 tests passing |
| **Format** | ✅ PASS | `cargo fmt --check` |

---

## CONSTRAINTS

| Constraint | Details |
|------------|---------|
| Rust-only | All core code in Rust |
| Performance | All hot paths optimized |
| No ThinkTools | ThinkTools stay in reasonkit-core |
| API Stability | Breaking changes require version bump |

---

*reasonkit-mem v0.1.0 | Memory Infrastructure | Apache 2.0*
*Extracted from reasonkit-core: 2025-12-25*
