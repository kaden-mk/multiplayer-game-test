use std::{collections::BTreeMap, fmt::Write};

use multiplayer_game_test::core::{
    typegen::FnSig,
    util::{COLORS, KEYS, NPATCH_LAYOUTS},
};

fn luau_type(rust: &str) -> String {
    match rust.trim() {
        "String" | "&str" => "string".into(),
        "bool" => "boolean".into(),
        "()" => "()".into(),
        "LuaVector" => "vector".into(),
        "LuaColor" => "Color".into(),
        "LuaRect" => "Rectangle".into(),
        "LuaNPatchInfo" => "NPatchInfo".into(),
        t if t.starts_with("LuaResult<") => luau_type(&t[10..t.len() - 1]),
        t if "iuf".contains(&t[..1])
            && t[1..]
                .chars()
                .all(|c| c.is_ascii_digit() || c == 's' || c == 'i' || c == 'z' || c == 'e') =>
        {
            "number".into()
        }
        other => other.into(),
    }
}

fn main() {
    let mut mods: BTreeMap<&str, Vec<&FnSig>> = BTreeMap::new();
    for s in inventory::iter::<FnSig> {
        let module = s.module.rsplit("::").next().unwrap_or(s.module);
        mods.entry(module).or_default().push(s);
    }

    let mut out = String::from("-- GENERATED FROM RUST (DO NOT EDIT) --\n\n");

    // Colors
    let names = COLORS
        .iter()
        .map(|(n, _)| format!("\"{n}\""))
        .collect::<Vec<_>>()
        .join(" | ");

    writeln!(out, "export type ColorName = {names}").unwrap();
    writeln!(
        out,
        "export type Color = {{ r: number?, g: number?, b: number?, a: number? }}"
    )
    .unwrap();

    // Keyboard Keys
    let names = KEYS
        .iter()
        .map(|(n, _)| format!("\"{n}\""))
        .collect::<Vec<_>>()
        .join(" | ");
    writeln!(out, "export type KeyboardKey = {names}").unwrap();

    // NPatch Layouts
    let names = NPATCH_LAYOUTS
        .iter()
        .map(|(n, _)| format!("\"{n}\""))
        .collect::<Vec<_>>()
        .join(" | ");
    writeln!(out, "export type NPatchLayout = {names}").unwrap();

    // Rectangle type
    writeln!(
        out,
        "export type Rectangle = {{ x: number, y: number, width: number, height: number }}\n"
    )
    .unwrap();

    // NPatchInfo type
    writeln!(
        out,
        "export type NPatchInfo = {{ source: Rectangle, top: number, bottom: number, right: number, left: number, layout: NPatchLayout }}\n"
    )
    .unwrap();

    // Color.new()
    writeln!(out, "declare Color: {{").unwrap();
    writeln!(out, "\tnew: ((name: ColorName) -> Color)").unwrap();
    writeln!(out, "\t   & ((r: number) -> Color)").unwrap();
    writeln!(out, "\t   & ((r: number, g: number, b: number) -> Color)").unwrap();
    writeln!(out, "\t   & (() -> Color)").unwrap();
    writeln!(out, "\t   & ((color: Color) -> Color)").unwrap();
    writeln!(out, "\t   & (({{ number }}) -> Color)").unwrap();
    writeln!(
        out,
        "\t   & ((r: number, g: number, b: number, a: number) -> Color),"
    )
    .unwrap();
    writeln!(out, "}}\n").unwrap();

    // Rectangle.new()
    writeln!(out, "declare Rectangle: {{").unwrap();
    writeln!(
        out,
        "\tnew: (x: number, y: number, width: number, height: number) -> Rectangle,"
    )
    .unwrap();
    writeln!(out, "}}\n").unwrap();

    // NPatchInfo.new()
    writeln!(out, "declare NPatchInfo: {{").unwrap();
    writeln!(out, "\tnew: (data: NPatchInfo) -> NPatchInfo,").unwrap();
    writeln!(out, "}}\n").unwrap();

    // Engine
    writeln!(out, "declare engine: {{").unwrap();
    for (module, sigs) in &mods {
        writeln!(out, "\t{module}: {{").unwrap();
        for s in sigs {
            let params = s
                .args
                .iter()
                .map(|(n, t)| format!("{n}: {}", luau_type(t)))
                .collect::<Vec<_>>()
                .join(", ");

            writeln!(out, "\t\t{}: ({params}) -> {},", s.name, luau_type(s.ret)).unwrap();
        }
        writeln!(out, "\t}},").unwrap();
    }
    writeln!(out, "}}").unwrap();
    std::fs::write("types/engine.d.luau", out).unwrap();
}
