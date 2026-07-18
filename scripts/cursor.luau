const Cursor = {}

input.set_cursor_visible(false)

const texture = assets.load_texture("assets/cursor.png")

function Cursor.on_update(dt: number)
    const pos = input.get_mouse_position()
    draw.texture_ex(texture, pos, 0, 0.85, "WHITE")
end

return Cursor
