export type Color = "BLACK" | "WHITE"
export type KeyboardKey = "A" | "B" | "C"

declare engine: {
    graphics: {
        clear_background: (color: Color) -> (),
        draw_text: (text: string, x: number, y: number, font_size: number, color: Color) -> (),
        draw_texture: (texture: string, x: number, y: number, tint: Color) -> (),
        draw_texture_ex: (texture: string, pos: vector, rot: number, size: number, tint: Color) -> (),
    },

    input: {
        is_key_down: (key: KeyboardKey) -> boolean,
        set_cursor_visible: (visible: boolean) -> (),
        get_mouse_position: () -> vector,
    },

    assets: {
        load_texture: (filepath: string) -> string
    },
}
