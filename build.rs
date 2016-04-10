extern crate skeptic;

fn readme_testing() {
    skeptic::generate_doc_tests(&["README.md"]);
}

fn main() {
    readme_testing();
}
