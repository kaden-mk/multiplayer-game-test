-- GENERATED FROM RUST (DO NOT EDIT) --

export type ColorName = "LIGHTGRAY" | "GRAY" | "DARKGRAY" | "YELLOW" | "GOLD" | "ORANGE" | "PINK" | "RED" | "MAROON" | "GREEN" | "LIME" | "DARKGREEN" | "SKYBLUE" | "BLUE" | "DARKBLUE" | "PURPLE" | "VIOLET" | "DARKPURPLE" | "BEIGE" | "BROWN" | "DARKBROWN" | "WHITE" | "BLACK" | "BLANK" | "MAGENTA" | "RAYWHITE"
export type Color = { r: number?, g: number?, b: number?, a: number? }
export type KeyboardKey = "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J" | "K" | "L" | "M" | "N" | "O" | "P" | "Q" | "R" | "S" | "T" | "U" | "V" | "W" | "X" | "Y" | "Z" | "ZERO" | "ONE" | "TWO" | "THREE" | "FOUR" | "FIVE" | "SIX" | "SEVEN" | "EIGHT" | "NINE" | "UP" | "DOWN" | "LEFT" | "RIGHT" | "SPACE" | "ENTER" | "ESCAPE" | "TAB" | "BACKSPACE" | "DELETE" | "INSERT" | "HOME" | "END" | "PAGE_UP" | "PAGE_DOWN" | "LEFT_SHIFT" | "LEFT_CONTROL" | "LEFT_ALT" | "RIGHT_SHIFT" | "RIGHT_CONTROL" | "RIGHT_ALT" | "GRAVE" | "CAPS_LOCK" | "F1" | "F2" | "F3" | "F4" | "F5" | "F6" | "F7" | "F8" | "F9" | "F10" | "F11" | "F12"
export type NPatchLayout = "NINE_PATCH" | "THREE_PATCH_HORIZONTAL" | "THREE_PATCH_VERTICAL"
export type Rectangle = { x: number, y: number, width: number, height: number }

export type NPatchInfo = { source: Rectangle, top: number, bottom: number, right: number, left: number, layout: NPatchLayout }

declare Color: {
	new: ((name: ColorName) -> Color)
	   & ((r: number) -> Color)
	   & ((r: number, g: number, b: number) -> Color)
	   & (() -> Color)
	   & ((color: Color) -> Color)
	   & (({ number }) -> Color)
	   & ((r: number, g: number, b: number, a: number) -> Color),
}

declare Rectangle: {
	new: (x: number, y: number, width: number, height: number) -> Rectangle,
}

declare NPatchInfo: {
	new: (data: NPatchInfo) -> NPatchInfo,
}

declare engine: {
	assets: {
		load_texture: (filename: string) -> string,
	},
	graphics: {
		draw_texture_npatch: (texture: string, n_patch_info: NPatchInfo, dest_rec: Rectangle, origin: vector, rotation: number, tint: Color) -> (),
		draw_texture: (texture: string, x: number, y: number, tint: Color) -> (),
		draw_texture_ex: (texture: string, pos: vector, rot: number, scale: number, tint: Color) -> (),
		draw_text: (text: string, x: number, y: number, font_size: number, color: Color) -> (),
		clear_background: (color: Color) -> (),
	},
	input: {
		set_cursor_visible: (visible: boolean) -> (),
		get_mouse_position: () -> vector,
		is_key_down: (key: string) -> boolean,
	},
}
