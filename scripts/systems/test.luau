const Scheduler = require("@lib/scheduler")
const Panel = require("@interface/panel")
const Text = require("@interface/text")
const Root = require("@interface/root")
const Button = require("@interface/button")
const UDim2 = require("@lib/udim2")

const rect = Rectangle.new(0, 0, 64, 64)
const npatch_info = NPatchInfo.new({
  source = rect,
  top = 16,
  bottom = 16,
  right = 16,
  left = 16,
  layout = "NINE_PATCH"
})

const texture = engine.assets.load_texture("assets/button.png")
const panel = Panel.new({
  pos = UDim2(0.5, 0, 0.5, 0),
  size = UDim2(0, 240, 0, 56),
  anchor = vector.create(0.5, 0.5),
  is_relative = true,
  texture = texture,
  npatch_info = npatch_info
})

const button = Button.new({
  pos = UDim2(0.5, 0, 0.5, 0),
  size = UDim2(1, 0, 1, 0),
  is_relative = true,
  anchor = vector.create(0.5, 0.5)
})
panel:add(button)

button.on_click:connect(function(button_type)
  print(button_type)
end)

const font = engine.assets.load_font("assets/PixeloidMono.ttf")
const text = Text.new({
  text = "PLAY",
  pos = UDim2(0.5, 0, 0.5, 0),
  anchor = vector.create(0.5, 0.5),
  is_relative = true,
  font_size = 27,
  font = font,
  spacing = 1
})

panel:add(text)
Root:add(panel)
