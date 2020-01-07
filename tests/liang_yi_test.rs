extern crate serde_json;
extern crate yarrow;

use yarrow::liang_yi::LIANG_YI;

#[test]
fn it_liang_yi() {
    for it in LIANG_YI.iter() {
        println!(
            "{it:?}\t{it}\t{json}",
            it = it,
            json = serde_json::to_string_pretty(&it).unwrap()
        )
    }
}
