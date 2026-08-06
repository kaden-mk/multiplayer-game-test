const Element = require("@interface/element")
const math_util = require("@lib/math_util")
const Signal = require("@lib/signal")
const UDim2 = require("@lib/udim2")
type UDim2 = UDim2.UDim2

export type UIButtonProps = {
    pos: UDim2?,
    size: UDim2?,
    is_relative: boolean?,
    anchor: vector?
}

export type UIButton = Element.UIElement & {
    on_click: Signal.Signal<MouseButton>,
    on_hover: Signal.Signal<boolean>,
    hovering: boolean,
}

const Button = Element:extend()

function Button.new(props: UIButtonProps?): UIButton
    props = props or {} :: UIButtonProps

    const self = Element.new(Button) :: UIButton
    self.pos = props.pos or UDim2(0, 0, 0, 0)
    self.size = props.size or UDim2(0, 0, 0, 0)
    self.anchor = props.anchor or vector.create(0, 0)
    self.is_relative = props.is_relative or false
    self.hovering = false
    self.on_click = Signal.new()
    self.on_hover = Signal.new()

    return self
end

function Button.update(self: UIButton, dt: number)
    const mouse_pos = engine.input.get_mouse_position()

    const pos = self:get_top_left_position()
    const size = self:get_absolute_size()

    -- HOVERING
    const old_hover = self.hovering
    self.hovering = math_util.is_point_in_rect(mouse_pos, pos, size)

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
