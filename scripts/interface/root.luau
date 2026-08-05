const Element = require("@interface/element")
const Scheduler = require("@lib/scheduler")
const UDim2 = require("@lib/udim2")

const RootUI = {}

const container = Element:new()
container.size = UDim2(0, 1280, 0, 720)

function RootUI:add<T>(child: T): T
    return container:add(child)
end

function RootUI:remove<T>(child: T)
    container:remove(child)
end

const function on_update(dt: number)
    container:update(dt)
end

const function on_draw()
    container:draw()
end

Scheduler.add_on_update(on_update, 10)
Scheduler.add_on_draw(on_draw, 10)

return RootUI
