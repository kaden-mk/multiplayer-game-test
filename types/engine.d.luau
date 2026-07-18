export type Color = "BLACK" | "WHITE"
export type KeyboardKey = "A" | "B" | "C"

declare draw: {
    clear: (color: Color) -> (),
    text: (text: string, x: number, y: number, font_size: number, color: Color) -> (),
    texture: (texture: string, x: number, y: number, tint: Color) -> (),
    texture_ex: (texture: string, pos: vector, rot: number, size: number, tint: Color) -> (),
}

declare input: {
    is_key_down: (key: KeyboardKey) -> boolean,
    set_cursor_visible: (visible: boolean) -> (),
    get_mouse_position: () -> vector,
}

declare assets: {
    load_texture: (filepath: string) -> string
}
