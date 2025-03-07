// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

//! This module provides a logical query plan enum that can describe queries. Logical query
//! plans can be created from a SQL statement or built programmatically via the Table API.
//!
//! Logical query plans can then be optimized and executed directly, or translated into
//! physical query plans and executed.

pub(crate) mod builder;
mod dfschema;
mod display;
mod expr;
mod expr_rewriter;
mod expr_schema;
mod expr_simplier;
mod expr_visitor;
mod extension;
mod operators;
pub mod plan;
mod registry;
pub mod window_frames;
pub use builder::{
    build_join_schema, union_with_alias, LogicalPlanBuilder, UNNAMED_TABLE,
};
pub use datafusion_expr::expr_fn::binary_expr;
pub use dfschema::{DFField, DFSchema, DFSchemaRef, ToDFSchema};
pub use display::display_schema;
pub use expr::{
    abs, acos, and, approx_distinct, approx_percentile_cont, array, ascii, asin, atan,
    avg, bit_length, btrim, call_fn, case, ceil, character_length, chr, coalesce, col,
    columnize_expr, combine_filters, concat, concat_expr, concat_ws, concat_ws_expr, cos,
    count, count_distinct, create_udaf, create_udf, date_part, date_trunc, digest, exp,
    exprlist_to_fields, floor, in_list, initcap, left, length, lit, lit_timestamp_nano,
    ln, log10, log2, lower, lpad, ltrim, max, md5, min, now, now_expr, nullif,
    octet_length, or, random, regexp_match, regexp_replace, repeat, replace, reverse,
    right, round, rpad, rtrim, sha224, sha256, sha384, sha512, signum, sin, split_part,
    sqrt, starts_with, strpos, substr, sum, tan, to_hex, to_timestamp_micros,
    to_timestamp_millis, to_timestamp_seconds, translate, trim, trunc, unalias, upper,
    when, Column, Expr, ExprSchema, Literal,
};
pub use expr_rewriter::{
    normalize_col, normalize_cols, replace_col, rewrite_sort_cols_by_aggs,
    unnormalize_col, unnormalize_cols, ExprRewritable, ExprRewriter, RewriteRecursion,
};
pub use expr_schema::ExprSchemable;
pub use expr_simplier::{ExprSimplifiable, SimplifyInfo};
pub use expr_visitor::{ExprVisitable, ExpressionVisitor, Recursion};
pub use extension::UserDefinedLogicalNode;
pub use operators::Operator;
pub use plan::{provider_as_source, source_as_provider};
pub use plan::{
    CreateCatalog, CreateCatalogSchema, CreateExternalTable, CreateMemoryTable,
    CrossJoin, DropTable, EmptyRelation, FileType, JoinConstraint, JoinType, Limit,
    LogicalPlan, Partitioning, PlanType, PlanVisitor, Repartition, TableScan, Union,
    Values,
};
pub(crate) use plan::{StringifiedPlan, ToStringifiedPlan};
pub use registry::FunctionRegistry;
