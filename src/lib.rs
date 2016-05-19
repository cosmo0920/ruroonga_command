#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

//! `ruroonga_command` provides extensible Groonga command query builder and
//! generator. It reduces runtime errors about Groonga queries.

extern crate url;

mod util;
/// `queryable` provides a shortcut to create URI style Groonga query.
///
/// Note that if you want to change URL prefix from `/d/`,
/// you have to use `CommandQuery` directly and
/// specify it with `set_prefix` in `CommandQuery`.
pub mod queryable;
mod commandlinable;
/// `commandable` provides a shortcut to create command line Groonga query.
pub mod commandable;
/// `extendable` provides appending user-defined key-value pair query functionality trait.
///
/// Note that this provided trait method is marked as `unsafe`.
/// You should use this trait **at your own risk**.
#[macro_use]
pub mod extendable;
/// `request_cancellable` provides appending `request_id` functionality trait.
#[macro_use]
pub mod request_cancellable;
/// `request_timeoutable` provides appending `timeout` functionality trait.
#[macro_use]
pub mod request_timeoutable;
pub mod types;
/// Type variant definitions for Groonga commands.
pub mod command;
/// An entity type for `queryable` trait.
pub mod command_query;
/// An entity type for `commandable` trait.
pub mod command_line;
pub mod dump;
/// `SelectCommand` type creates `select` query.
///
/// This type is **one of the most important type** in this crate.
/// Because this type is to be able to combine with `drilldown` related types.
///
/// It recommends to refer to the documents under `selectable` module.
pub mod select;
pub mod selectable;
pub mod load;
pub mod status;
pub mod cache_limit;
pub mod delete;
pub mod log_level;
pub mod log_put;
pub mod log_reopen;
pub mod table_create;
pub mod table_list;
pub mod table_rename;
pub mod table_remove;
pub mod column_copy;
pub mod column_create;
pub mod column_list;
pub mod column_rename;
pub mod column_remove;
pub mod database_unmap;
/// Using request cancel mechanism.
/// If you use this cancel feature,
/// you should use `request_cancellable` trait to add `request_id` into each request.
pub mod request_cancel;
pub mod object_exist;
pub mod object_inspect;
pub mod object_remove;
pub mod normalize;
pub mod normalizer_list;
pub mod plugin_register;
pub mod plugin_unregister;
pub mod thread_limit;
pub mod tokenize;
pub mod tokenizer_list;
pub mod truncate;
/// This module provides fully user-defined query creation mechanism.
/// This module is only provided for funture extensibility.
/// You should use this module **at your own risk**.
pub mod extension;
pub mod schema;
/// `dsl` provides its `XXXCommand` returning dsl syntax like this:
///
/// ## DSL Example
///
/// ```
/// use ruroonga_command::dsl::*;
/// let select = select("Entries".to_string())
///                  .filter("content @ \"fast\"".to_string());
/// ```
pub mod dsl;
