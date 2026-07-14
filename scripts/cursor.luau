const Test = {}

input.set_cursor_visible(false)

const texture = assets.load_texture("assets/cursor.png")

function Test.on_update(dt: number)
    const pos = input.get_mouse_position()
    draw.texture(texture, pos.x, pos.y, "WHITE")
end

return Test
