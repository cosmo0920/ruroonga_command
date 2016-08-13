extern crate ruroonga_command as ruroonga;
use ruroonga::selectable::drilldown::Drilldown;
use ruroonga::commandable::Commandable;
use ruroonga::queryable::Queryable;
use ruroonga::dsl::*;

fn simple_example() {
    let select = select("Entries".to_string()).filter("content @ \"fast\"".to_string());
    let drilldown = Drilldown::new().drilldown(vec![("tag".to_string())]);
    let builder = (select + drilldown).build();
    println!("simple: {:?}", builder);
}

fn to_query_example() {
    let select = select("Entries".to_string()).filter("content @ \"fast\"".to_string());
    let drilldown = Drilldown::new().drilldown(vec![("tag".to_string())]);
    let builder = (select + drilldown).to_query();
    println!("query: {}", builder);
}

fn to_command_example() {
    let select = select("Entries".to_string()).filter("content @ \"fast\"".to_string());
    let drilldown = Drilldown::new().drilldown(vec![("tag".to_string())]);
    let builder = (select + drilldown).to_command();
    println!("command line: {}", builder);
}

fn main() {
    simple_example();
    to_query_example();
    to_command_example();
}
