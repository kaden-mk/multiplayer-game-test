const Button = require("@interface/button")
const RootUI = require("@interface/root")

const r = Rectangle.new(640, 360, 40, 30)

const button = RootUI.container:add(Button.new({
    pos = vector.create(r.x, r.y),
    size = vector.create(r.width, r.height)
}))

button.on_click:connect(function(button)
    print(button)
end)

button.on_hover:connect(function(hovered)
    print(hovered)
end)

const Test = {}

const white = Color.new("WHITE")
function Test.on_update(dt: number)
    engine.graphics.draw_rectangle(r, white)

    RootUI.on_update(dt)
end

return Test
