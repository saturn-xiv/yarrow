extern crate serde_json;
extern crate yarrow;

use yarrow::wu_xing::WU_XING;

#[test]
fn it_wu_xing() {
    for it in WU_XING.iter() {
        println!(
            "{it:?}\t{it}\t{json}",
            it = it,
            json = serde_json::to_string_pretty(&it).unwrap()
        )
    }
}
