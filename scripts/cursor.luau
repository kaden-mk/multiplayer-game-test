const asset_store = require("./util/asset_store")

const Cursor = {}

engine.input.set_cursor_visible(false)

const white = Color.new("WHITE")

function Cursor.on_update(dt: number)
    const pos = engine.input.get_mouse_position()
    engine.graphics.draw_texture_ex(asset_store.cursor, pos, 0, 0.85, white)
end

return Cursor
