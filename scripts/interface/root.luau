const Element = require("@interface/element")

const RootUI = {}
RootUI.container = Element.new()

function RootUI.on_update(dt)
    RootUI.container:update(dt)
    RootUI.container:draw()
end

return RootUI
