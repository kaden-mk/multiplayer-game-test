const Scheduler = require("@lib/scheduler")
const Panel = require("@interface/panel")
const Root = require("@interface/root")

const x, y, w, h = 1280 / 2, 720 / 2, 30, 30
const rect = Rectangle.new(0, 0, 64, 64)
const npatch_info = NPatchInfo.new({
  source = rect,
  top = 16,
  bottom = 16,
  right = 16,
  left = 16,
  layout = "NINE_PATCH"
})

const panel = Panel.new({
  pos = vector.create(x, y),
  size = vector.create(240, 56),
  anchor = vector.create(0.5, 0.5),
  texture = "assets/button.png",
  npatch_info = npatch_info
})

Root:add(panel)
