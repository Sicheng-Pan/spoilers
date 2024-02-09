use anyhow::Result;
use spoilers::{
    model::NLLBAdapter,
    translator::{Language, Translator},
};

fn example() -> Result<()> {
    let nllb = NLLBAdapter::new_from_web("facebook/nllb-200-distilled-1.3B").unwrap();
    let translator =
        Translator::new_default(Box::new(nllb), "target/ct2-nllb-200-distilled-1.2B-int8")?;
    let result = translator.translate("無免許で電動キックボードに乗って歩行者と衝突し、大けがをさせたまま逃げたとして、愛知県警は8日、名古屋市中区新栄1丁目の無職松崎和則容疑者（44）を自動車運転死傷処罰法違反（無免許危険運転致傷）と道路交通法違反（ひき逃げ）の疑いで逮捕した。ひき逃げ容疑は認める一方、「免許が必要だと思っていなかった」と一部を否認しているという。", Language::Japanese, Language::Chinese)?;
    println!("{result}");
    Ok(())
}

fn main() {
    example().unwrap()
}
