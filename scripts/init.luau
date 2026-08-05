const scheduler = require("@lib/scheduler")

const systems = discover("scripts/systems")

for _, system in systems do
    execute(system)
end

function engine.on_update(dt: number)
    scheduler.update(dt)
end

function engine.on_draw()
    scheduler.draw()
end
