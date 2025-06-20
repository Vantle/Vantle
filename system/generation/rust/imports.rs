//! Import management utilities for generated test modules.
//!
//! This module provides functionality to deduplicate and group import statements
//! using a trie data structure, ensuring clean and organized imports in generated code.

use proc_macro2::{Ident, Span};
use quote::quote;
use std::collections::{BTreeMap, BTreeSet};
use syn::{ItemUse, UseTree};

/// A trie structure for organizing and deduplicating import paths.
///
/// This structure helps group related imports together and avoid duplicates
/// when generating test modules.
#[derive(Debug, Default)]
pub struct ImportTrie {
    /// Child nodes indexed by path segment.
    children: BTreeMap<String, ImportTrie>,
    /// Terminal items at this level (e.g., specific types or functions).
    items: BTreeSet<String>,
}

impl ImportTrie {
    /// Insert a fully-qualified import path into the trie.
    ///
    /// # Example
    /// ```ignore
    /// let mut trie = ImportTrie::default();
    /// trie.insert("std::collections::HashMap");
    /// trie.insert("std::collections::HashSet");
    /// ```
    pub fn insert(&mut self, path: &str) {
        let segments: Vec<&str> = path.split("::").collect();
        self.insert_segments(&segments);
    }

    /// Recursively insert path segments into the trie.
    fn insert_segments(&mut self, segments: &[&str]) {
        match segments {
            [] => {}
            [item] => {
                self.items.insert((*item).to_string());
            }
            [namespace, rest @ ..] => {
                self.children
                    .entry((*namespace).to_string())
                    .or_default()
                    .insert_segments(rest);
            }
        }
    }

    /// Convert the trie into a sorted list of `use` statements.
    ///
    /// This method generates clean, grouped import statements with
    /// standard library imports appearing before crate imports.
    pub fn to_use_statements(&self) -> Vec<ItemUse> {
        let mut paths = Vec::new();
        self.collect_paths(&mut Vec::new(), &mut paths);

        // Sort paths: std imports first, then others
        paths.sort_by(|a, b| {
            let a_is_std = a.first().is_some_and(|s| s == "std");
            let b_is_std = b.first().is_some_and(|s| s == "std");

            match (a_is_std, b_is_std) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.cmp(b),
            }
        });

        paths
            .into_iter()
            .map(|segments| Self::create_use_statement(&segments))
            .collect()
    }

    /// Recursively collect all import paths from the trie.
    fn collect_paths(&self, prefix: &mut Vec<String>, out: &mut Vec<Vec<String>>) {
        // Add terminal items at this level
        for item in &self.items {
            let mut path = prefix.clone();
            path.push(item.clone());
            out.push(path);
        }

        // Recurse into children
        for (segment, child) in &self.children {
            prefix.push(segment.clone());
            child.collect_paths(prefix, out);
            prefix.pop();
        }
    }

    /// Create a `use` statement from a path of segments.
    fn create_use_statement(segments: &[String]) -> ItemUse {
        assert!(
            !segments.is_empty(),
            "Cannot create use statement from empty path"
        );

        let tree = Self::build_use_tree(segments);

        syn::parse_quote! {
            use #tree;
        }
    }

    /// Build a `UseTree` from path segments.
    fn build_use_tree(segments: &[String]) -> UseTree {
        let idents: Vec<Ident> = segments
            .iter()
            .map(|s| Ident::new(s, Span::call_site()))
            .collect();

        // Build the path expression
        let tree_tokens = quote! {
            #(#idents)::*
        };

        syn::parse2(tree_tokens).expect("Failed to parse generated use tree")
    }
}
