const Element = require("@interface/element")

export type UIPanelProps = {
  pos: vector?,
  size: vector?,
  anchor: vector?,
  scale: number?,
  rot: number?,
  texture: string?,
  color: Color?,
  npatch_info: NPatchInfo?
}

export type UIPanel = Element.UIElement & {
  texture: string?,
  color: Color,
  scale: number,
  rot: number,
  anchor: vector,
  npatch_info: NPatchInfo?
}

const Panel = setmetatable({}, { __index = Element })
Panel.__index = Panel

function Panel.new(props: UIPanelProps): UIPanel
  const self = setmetatable(Element.new(), Panel) :: UIPanel
  self.pos = props.pos or vector.create(0, 0)
  self.size = props.size or vector.create(0, 0)
  self.anchor = props.anchor or vector.create(0, 0)
  self.texture = props.texture and engine.assets.load_texture(props.texture)
  self.color = props.color or Color.new("WHITE") 
  self.scale = props.scale or 1
  self.rot = props.rot or 0
  self.npatch_info = props.npatch_info

  return self
end

function Panel.draw(self: UIPanel)
  const origin = vector.create(
    self.size.x * self.anchor.x,
    self.size.y * self.anchor.y
  )

  if self.npatch_info then
    const dest = {
      x = self.pos.x, y = self.pos.y,
      width = self.size.x, height = self.size.y
    }

    engine.graphics.draw_texture_npatch(
      self.texture,
      self.npatch_info,
      dest,
      origin,
      self.rot,
      self.color
    )
  elseif self.texture then
    local draw_pos = self.pos
    if self.rot ~= 0 then
      const rad = math.rad(self.rot)
      const cos, sin = math.cos(rad), math.sin(rad)
      draw_pos = vector.create(
        self.pos.x - (origin.x * cos - origin.y * sin),
        self.pos.y - (origin.x * sin + origin.y * cos)
      )
    else
      draw_pos = vector.create(
        self.pos.x - origin.x,
        self.pos.y - origin.y
      )
    end

    engine.graphics.draw_texture_ex(
      self.texture,
      draw_pos,
      self.rot,
      self.scale,
      self.color
    )
  else
    engine.graphics.draw_rectangle(
      {
        x = self.pos.x - origin.x,
        y = self.pos.y - origin.y,
        width = self.size.x,
        height = self.size.y
      },
      self.color
    )
  end

  Element.draw(self)
end

return Panel
