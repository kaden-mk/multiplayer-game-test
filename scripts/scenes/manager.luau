const Scheduler = require("@lib/scheduler")

local pending_scene: Scene? = nil
local current_scene: Scene? = nil

export type Scene = {
  enter: () -> (),
  exit: () -> (),
  on_update: (dt: number) -> (),
  on_draw: () -> ()
}

const SceneManager = {}

function SceneManager.set(scene: Scene)
    pending_scene = scene
end

const function on_update(dt: number)
    if pending_scene then
        if current_scene then current_scene.exit() end

        current_scene = pending_scene
        pending_scene = nil
        (current_scene :: Scene).enter()
    end
  
    if current_scene then
        current_scene.on_update(dt)
    end
end

const function on_draw()
    if current_scene then
        current_scene.on_draw()
    end
end

Scheduler.add_on_update(on_update, 100)
Scheduler.add_on_draw(on_draw, 100)

return SceneManager
