extern crate ruroonga_command as ruroonga;
use ruroonga::selectable::staged_columns::StagedColumns;
use ruroonga::commandable::Commandable;
use ruroonga::dsl::*;
use ruroonga::types::data_type::DataType;
use ruroonga::types::column_flag_type::ColumnFlagType;
use ruroonga::selectable::staged_columns::WindowableColumn::{Window, Value};
use ruroonga::selectable::staged_columns::WindowFunction::RecordNumber;

fn staged_columns_to_command() {
    let label = "filtered".to_string();
    let stage = "filtered".to_string();
    let value = "'_id'".to_string();
    let select = select("Items".to_string())
        .filter("price < 1200".to_string())
        .output_columns(vec![("_id".to_string()),
                             ("_key".to_string()),
                             ("price".to_string()),
                             ("filtered".to_string())]);
    let staged_columns = StagedColumns::new(label.clone(),
                                            stage.clone(),
                                            DataType::UInt32,
                                            Value(value.clone()))
        .flags(vec![(ColumnFlagType::Scalar)]);
    let builder = (select + staged_columns).to_command();
    println!("staged columns: {}", builder);
}

fn staged_columns_with_window_function() {
    let label = "nth_record".to_string();
    let stage = "initial".to_string();
    let select = select("Items".to_string()).output_columns(vec![("_id".to_string()),
                                                                 ("_key".to_string()),
                                                                 ("price".to_string()),
                                                                 ("filtered".to_string())]);
    let staged_columns = StagedColumns::new(label.clone(),
                                            stage.clone(),
                                            DataType::UInt32,
                                            Window(RecordNumber))
        .flags(vec![(ColumnFlagType::Scalar)])
        .sort_keys(vec![("-price".to_string())]);
    let builder = (select + staged_columns).to_command();
    println!("staged columns with window function: {}", builder);
}

fn main() {
    staged_columns_to_command();
    staged_columns_with_window_function();
}
