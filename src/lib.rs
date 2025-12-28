//! # ReasonKit Memory Infrastructure
//!
//! Long-term memory, retrieval, and hybrid search infrastructure for ReasonKit.
//! This crate provides the "Hippocampus" - memory & retrieval capabilities.
//!
//! ## Architecture
//!
//! ```text
//! reasonkit-mem/
//! ├── storage/      # Qdrant vector + file-based storage
//! ├── embedding/    # Dense vector embeddings (BGE-M3, OpenAI)
//! ├── retrieval/    # Hybrid search, fusion, reranking
//! ├── raptor/       # RAPTOR hierarchical tree structure
//! ├── indexing/     # BM25/Tantivy sparse indexing
//! └── rag/          # RAG pipeline orchestration
//! ```
//!
//! ## Features
//!
//! - **Hybrid Search**: Dense (Qdrant) + Sparse (Tantivy BM25) fusion
//! - **RAPTOR Trees**: Hierarchical retrieval for long-form QA
//! - **Cross-Encoder Reranking**: Precision boosting for final results
//! - **Embedded Mode**: Zero-config development with Qdrant embedded
//!
//! ## Usage
//!
//! ```rust,ignore
//! use reasonkit_mem::{Storage, EmbeddingService, HybridRetriever};
//!
//! // Create storage backend
//! let storage = Storage::new_embedded("./data").await?;
//!
//! // Index documents
//! storage.index_documents(&docs).await?;
//!
//! // Hybrid search
//! let results = retriever.search("query", 10).await?;
//! ```

#![allow(missing_docs)]
#![warn(clippy::all)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Re-export commonly used types
pub use error::{MemError, MemResult};
pub use types::*;

/// Error types for memory operations
pub mod error;

/// Core types shared across all modules
pub mod types;

/// Alias for backward compatibility
pub type Error = MemError;
/// Result alias
pub type Result<T> = MemResult<T>;

/// Vector and file-based storage backends
///
/// Provides:
/// - Qdrant vector storage (embedded and cluster modes)
/// - File-based fallback storage
/// - Document and chunk management
pub mod storage;

/// Dense vector embedding services
///
/// Supports:
/// - Local embeddings (BGE-M3 via ONNX)
/// - Remote embeddings (OpenAI, Anthropic, etc.)
/// - Caching and batching
pub mod embedding;

/// Hybrid retrieval with fusion and reranking
///
/// Implements:
/// - Dense + Sparse hybrid search
/// - Reciprocal Rank Fusion (RRF)
/// - Cross-encoder reranking
/// - Query expansion
pub mod retrieval;

/// RAPTOR hierarchical tree structure
///
/// Based on RAPTOR paper (Sarthi et al. 2024):
/// - Multi-level summarization
/// - Recursive clustering
/// - Long-form QA optimization
pub mod raptor;

/// BM25/Tantivy sparse indexing
///
/// Provides:
/// - Full-text search indexing
/// - Custom analyzers
/// - Incremental updates
pub mod indexing;

/// RAG pipeline orchestration
///
/// Coordinates:
/// - Query processing
/// - Multi-stage retrieval
/// - Context assembly
pub mod rag;

/// Prelude for convenient imports
pub mod prelude {
    pub use crate::error::{MemError, MemResult};
    pub use crate::types::*;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_lib_compiles() {
        // Basic compilation test
        assert!(true);
    }
}
