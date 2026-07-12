export type Color = "BLACK" | "WHITE"
export type KeyboardKey = "A" | "B" | "C"

declare draw: {
    clear: (color: Color) -> (),
    text: (text: string, x: number, y: number, font_size: number, color: Color) -> (),
    texture: (texture: String, x: number, y: number, tint: Color) -> ()
}

declare input: {
    is_key_down: (key: KeyboardKey) -> boolean
}

declare assets: {
    load_texture: (filepath: string) -> string
}
