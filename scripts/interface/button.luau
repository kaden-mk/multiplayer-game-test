const Element = require("@interface/element")
const math_util = require("@lib/math_util")
const Signal = require("@lib/signal")

export type UIButtonProps = {
    pos: vector?,
    size: vector?
}

export type UIButton = Element.UIElement & {
    on_click: Signal.Signal<MouseButton>,
    on_hover: Signal.Signal<boolean>,
    hovering: boolean,
}

const Button = setmetatable({}, { __index = Element })
Button.__index = Button

function Button.new(props: UIButtonProps): UIButton
    const self = setmetatable(Element.new(), Button) :: UIButton
    self.pos = props.pos or vector.create(0, 0)
    self.size = props.size or vector.create(0, 0)
    self.on_click = Signal.new()
    self.on_hover = Signal.new()
    self.hovering = false

    return self
end

function Button.update(self: UIButton, dt)
    const mouse_pos = engine.input.get_mouse_position()

    -- HOVERING
    const old_hover = self.hovering
    self.hovering = math_util.is_point_in_rect(mouse_pos, self.pos, self.size)

    if old_hover ~= self.hovering then
        self.on_hover:fire(self.hovering)
    end

    -- CLICKING
    if self.hovering then
        if engine.input.is_mouse_button_released("BUTTON_LEFT") then
            self.on_click:fire("BUTTON_LEFT")
        end
    end

    Element.update(self, dt)
end

return Button
