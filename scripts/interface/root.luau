const Element = require("@interface/element")
const UDim2 = require("@lib/udim2")

const RootUI = {}

const container = Element.new()
container.size = UDim2(0, 1280, 0, 720)

function RootUI:add<T>(child: T): T
    return container:add(child)
end

function RootUI:remove<T>(child: T)
    container:remove(child)
end

function RootUI.on_update(dt)
    container:update(dt)
end

function RootUI.on_draw()
    container:draw()
end

return RootUI
