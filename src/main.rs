use std::collections::HashMap;
use std::path::PathBuf;

use circom_parser::run_parser_cached;
use circom_structure::error_definition::Report;

fn main() {
    let mut sample_cache = HashMap::<PathBuf, String>::new();
    sample_cache.insert(
        PathBuf::from("/test/a.circom"),
        String::from(concat!(
            "pragma circom 2.1.5;\n",
            "include \"./b.circom\";\n",
            "include \"./c.circom\";\n",
            "\n",
            "template A() {\n",
            "    signal output out;\n",
            "    component b = B();\n",
            "    component c = C();\n",
            "    out <== b.out + c.out;\n",
            "}\n",
            "\n",
            "component main = A();"
        ))
    );

    sample_cache.insert(
        PathBuf::from("/test/b.circom"),
        String::from(concat!(
            "pragma circom 2.1.5;\n",
            "\n",
            "template B() {\n",
            "    signal output out <== 1;\n",
            "}",
        ))
    );

    sample_cache.insert(
        PathBuf::from("/test/c.circom"),
        String::from(concat!(
            "pragma circom 2.1.5;\n",
            "\n",
            "template C() {\n",
            "    signal output out <== 2;\n",
            "}",
        ))
    );

    let result = run_parser_cached(String::from("/test/a.circom"), "2.1.5", vec![], &sample_cache, true);
    let (file_library, reports) = match result {
        Ok((archive, reports)) => (archive.file_library, reports),
        Err((file_library, reports)) => (file_library, reports)
    };

    if reports.is_empty() {
        println!("Working as intended!");
    } else {
        Report::print_reports(&reports, &file_library);
    }
}
