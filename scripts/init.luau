const root = require("@interface/root")
const systems = discover("scripts/systems")

for _, system in systems do
    execute(system)
end

function engine.on_update(dt: number)
    root.on_update(dt)
end

function engine.on_draw()
    root.on_draw()
end
