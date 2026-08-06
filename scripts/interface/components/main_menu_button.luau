const Button = require("@interface/elements/button")
const Text = require("@interface/elements/text")
const Panel = require("@interface/elements/panel")
const UDim2 = require("@lib/udim2")
const Spring = require("@lib/spring")
const udim2 = require("@lib/udim2")

const rect = Rectangle.new(0, 0, 64, 64)
const texture = engine.assets.load_texture("assets/button.png")
const font = engine.assets.load_font("assets/PixeloidMono.ttf")

const npatch_info = NPatchInfo.new({
  source = rect,
  top = 16,
  bottom = 16,
  right = 16,
  left = 16,
  layout = "NINE_PATCH"
})

const MIN_X = 220
const MAX_X = 230
const MIN_Y = 56
const MAX_Y = 62

type Props = {
  pos: udim2.UDim2,
  text: string
}

return function(props: Props)
    props = props or {} :: Props

    if not props.pos then error("No position was declared") end
    if not props.text then error("No text was declared") end

    const x = Spring.new(MIN_X, { frequency = 18, damping = 0.35 })
    const y = Spring.new(MIN_Y, { frequency = 18, damping = 0.35 })

    const button = Button {
      pos = props.pos,
      size = UDim2(0, x:get_position(), 0, y:get_position()),
      anchor = vector.create(0.5, 0.5),
      is_relative = true,
    }

    const panel = Panel {
      pos = UDim2(0.5, 0, 0.5, 0),
      size = UDim2(1, 0, 1, 0),
      anchor = vector.create(0.5, 0.5),
      is_relative = true,
      texture = texture,
      npatch_info = npatch_info
    }   
    button:add(panel)

    const text = Text {
      text = props.text,
      pos = UDim2(0.5, 0, 0.5, 0),
      anchor = vector.create(0.5, 0.5),
      is_relative = true,
      font_size = 9 * 4,
      font = font,
      spacing = 1
    }
    button:add(text)

    button.on_update = function(self, dt: number)
        x:update(dt)
        y:update(dt)
        self.size = UDim2(0, x:get_position(), 0, y:get_position())
    end

    button.on_hover:connect(function(entered)
        if entered then
            x:set_goal(MAX_X)
            y:set_goal(MAX_Y)

            x:impulse(90)
            y:impulse(45)
        else
            x:set_goal(MIN_X)
            y:set_goal(MIN_Y)

            x:impulse(-60)
            y:impulse(-30)
        end
    end)

    button.on_click:connect(function(button_type)
        if button_type ~= "BUTTON_LEFT" then return end

        x:impulse(-500)
        y:impulse(-250)
    end)

    return button
end
