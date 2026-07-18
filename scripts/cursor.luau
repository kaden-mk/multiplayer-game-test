const Assets = require("./assets")
const Cursor = {}

input.set_cursor_visible(false)

function Cursor.on_update(dt: number)
    const pos = input.get_mouse_position()
    draw.texture_ex(Assets.cursor, pos, 0, 0.85, "WHITE")
end

return Cursor
