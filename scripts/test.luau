const Test = {}

const texture = assets.load_texture("assets/cursor.png")

function Test.on_update(dt: number)
    if input.is_key_down("A") then
        draw.texture(texture, 12, 12, "WHITE")
    end
end

return Test
