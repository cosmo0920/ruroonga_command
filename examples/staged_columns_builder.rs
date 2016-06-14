extern crate ruroonga_command as ruroonga;
use ruroonga::selectable::staged_columns::StagedColumns;
use ruroonga::commandable::Commandable;
use ruroonga::dsl::*;
use ruroonga::types::data_type::DataType;
use ruroonga::types::column_flag_type::ColumnFlagType;

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
                                            vec![(ColumnFlagType::Scalar)],
                                            value.clone());
    let builder = (select + staged_columns).to_command();
    println!("staged columns: {}", builder);
}

fn main() {
    staged_columns_to_command();
}
