export type Color = "BLACK" | "WHITE"
export type KeyboardKey = "A" | "B" | "C"

declare rl: {
    clear_background: (color: Color) -> (),
    draw_text: (text: string, x: number, y: number, font_size: number, color: Color) -> (),
    is_key_down: (key: KeyboardKey) -> boolean
}
