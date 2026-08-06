const Element = require("@interface/element")
const Root = require("@interface/root")
const Scheduler = require("@lib/scheduler")
const UDim2 = require("@lib/udim2")

local pending_scene: Scene? = nil
local current_scene: Scene? = nil
local current_container: Element.UIElement? = nil

export type Scene = {
  enter: (container: Element.UIElement) -> (),
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
        const scene = pending_scene
        pending_scene = nil

        if current_scene then
            current_scene.exit()

            if current_container then
                current_container:destroy()
                current_container = nil
            end
        end

        current_container = Element:new()
        current_container.is_relative = true
        current_container.size = UDim2(1, 0, 1, 0)
        Root:add(current_container)
        
        current_scene = scene
        scene.enter(current_container)
    end
  
    if current_scene then
        current_scene.on_update(dt)
    end
end

const function on_draw()
    if current_scene and current_container then
        current_scene.on_draw()
    end
end

Scheduler.add_on_update(on_update, 100)
Scheduler.add_on_draw(on_draw, 100)

return SceneManager
