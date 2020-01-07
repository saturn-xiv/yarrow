extern crate serde_json;
extern crate yarrow;

use yarrow::ba_gua::XIAN_TIAN;

#[test]
fn it_ba_gua() {
    for it in XIAN_TIAN.iter() {
        println!(
            "{it:?}\t{it}\n{json}",
            it = it,
            json = serde_json::to_string_pretty(&it).unwrap()
        )
    }
}
