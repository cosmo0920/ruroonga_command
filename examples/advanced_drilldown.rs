extern crate ruroonga_command as ruroonga;
use ruroonga::selectable::labeled_drilldown::LabeledDrilldown;
use ruroonga::commandable::Commandable;
use ruroonga::queryable::Queryable;
use ruroonga::dsl::*;
use ruroonga::selectable::pseudo_table::PseudoTable;

fn advanced_drilldown_to_command() {
    let select = select("Memos".to_string());
    let drilldown_label1 = LabeledDrilldown::new("label1".to_string())
        .table("label2".to_string())
        .keys(vec![("category".to_string())])
        .output_columns(vec![("_key".to_string()),
                             ("_nsubrecs".to_string())]);
    let drilldown_label2 = LabeledDrilldown::new("label2".to_string())
        .keys(vec![("tag".to_string())])
        .output_columns(vec![("_key".to_string()),
                             ("_nsubrecs".to_string()),
                             ("category".to_string())]);
    let builder = (select + vec![(drilldown_label1),
                                 (drilldown_label2)]).to_command();
    println!("advanced drilldown: {}", builder);
}


fn advanced_drilldown_to_query() {
    let select = select("Memos".to_string());
    let drilldown_label1 = LabeledDrilldown::new("label1".to_string())
        .table("label2".to_string())
        .keys(vec![("category".to_string())])
        .output_columns(vec![("_key".to_string()),
                             ("_nsubrecs".to_string())]);
    let drilldown_label2 = LabeledDrilldown::new("label2".to_string())
        .keys(vec![("tag".to_string())])
        .output_columns(vec![("_key".to_string()),
                             ("_nsubrecs".to_string()),
                             ("category".to_string())]);
    let builder = (select + vec![(drilldown_label1),
                                 (drilldown_label2)]).to_query();
    println!("advanced drilldown: {}", builder);
}

fn main() {
    advanced_drilldown_to_command();
    advanced_drilldown_to_query();
}
