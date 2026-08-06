const Panel = require("@interface/elements/panel")
const Root = require("@interface/root")
const UDim2 = require("@lib/udim2")

engine.input.set_cursor_visible(false)

local pos = engine.input.get_mouse_position()

const cursor = Panel {
  pos = UDim2(0, pos.x, 0, pos.y),
  texture = engine.assets.load_texture("assets/cursor.png"),
  zindex = 400
}

cursor.on_update = function()
    pos = engine.input.get_mouse_position()
    cursor.pos = UDim2(0, pos.x, 0, pos.y)
end

Root:add(cursor)
