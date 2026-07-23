const Element = require("@interface/element")

const RootUI = {}
const container = Element.new()

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
