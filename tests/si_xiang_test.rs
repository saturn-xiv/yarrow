extern crate serde_json;
extern crate yarrow;

use yarrow::si_xiang::SI_XIANG;

#[test]
fn it_si_xiang() {
    for it in SI_XIANG.iter() {
        println!(
            "{it:?}\t{it}\n{json}",
            it = it,
            json = serde_json::to_string_pretty(&it).unwrap()
        )
    }
}
