//! Provides drilldown related procedures.
//! This module will be used with `SelectCommand` type by `+` operation
//! to concatinate `select` and `drilldown` queries.
//!
//! Also this module support `pseudo_table` and `staged_columns` manupilation.
pub mod drilldown;
pub mod labeled_drilldown;
pub mod drilldownable;
pub mod labeled_drilldownable;
pub mod labeled_drilldown_sequencable;
pub mod drilldown_builder;
pub mod labeled_drilldown_builder;
pub mod labeled_drilldown_sequence_builder;
/// `fragmentable` module provides making query fragment operation.
/// It is useful to make composable query elements.
/// It makes `select` query to be composable against drilldown related query and `staged_columns` query.
pub mod fragmentable;
/// `pseudo_table` is used for tablable state which is appeared in "drilldown in drilldown".
pub mod pseudo_table;
pub mod staged_columns;
pub mod staged_columns_builder;
pub mod staged_columnable;
