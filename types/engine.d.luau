export type Color = "BLACK" | "WHITE"
export type KeyboardKey = "A" | "B" | "C"

declare draw: {
    clear: (color: Color) -> (),
    text: (text: string, x: number, y: number, font_size: number, color: Color) -> ()
}

declare input: {
    is_key_down: (key: KeyboardKey) -> boolean
}
