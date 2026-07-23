const root = require("@interface/root")

function engine.on_update(dt: number)
    root.on_update(dt)
end

function engine.on_draw()
    root.on_draw()
end
