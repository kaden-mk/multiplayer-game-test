const asset_store = require("./util/asset_store")

const Button = {}

const patch_source = Rectangle.new(0, 0, 64, 64)
const patch = NPatchInfo.new({
    source = patch_source,
    left = 16,
    right = 16,
    top = 16,
    bottom = 16,
    layout = "NINE_PATCH"
})

const white = Color.new("WHITE")
const rect = Rectangle.new(100, 100, 240, 56)
const origin = vector.create(0, 0)

function Button.on_update(dt: number)
    engine.graphics.draw_texture_npatch(
        asset_store.button,
        patch,
        rect,
        origin,
        0,
        white
    )
end

return Button
