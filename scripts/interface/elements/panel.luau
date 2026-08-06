const Element = require("@interface/element")
const UDim2 = require("@lib/udim2")
type UDim2 = UDim2.UDim2

export type UIPanelProps = {
  pos: UDim2?,
  size: UDim2?,
  is_relative: boolean?,
  anchor: vector?,
  rot: number?,
  zindex: number?,
  texture: string?,
  color: Color?,
  npatch_info: NPatchInfo?
}

export type UIPanel = Element.UIElement & {
  texture: string?,
  color: Color,
  rot: number,
  npatch_info: NPatchInfo?
}

const Panel = Element:extend()

function Panel.new(props: UIPanelProps?): UIPanel
  props = props or {} :: UIPanelProps

  const self = Element.new(Panel) :: UIPanel
  self.pos = props.pos or UDim2(0, 0, 0, 0)
  self.size = props.size or UDim2(0, 0, 0, 0)
  self.zindex = props.zindex or 0
  self.anchor = props.anchor or vector.create(0, 0)
  self.is_relative = props.is_relative or false
  self.texture = props.texture
  self.color = props.color or Color.new("WHITE") 
  self.rot = props.rot or 0
  self.npatch_info = props.npatch_info

  return self
end

function Panel.draw(self: UIPanel)
  self = self :: UIPanel -- ??? this fixes an error??

  const size = self:get_absolute_size()
  const pos = self:get_absolute_position()
  const origin = self:get_anchor_offset()

  if self.npatch_info and self.texture then
    const dest = {
      x = pos.x, y = pos.y,
      width = size.x, height = size.y
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
    local draw_pos = self:get_top_left_position()
    if self.rot ~= 0 then
      const rad = math.rad(self.rot)
      const cos, sin = math.cos(rad), math.sin(rad)

      draw_pos = vector.create(
        pos.x - (origin.x * cos - origin.y * sin),
        pos.y - (origin.x * sin + origin.y * cos)
      )
    end

    engine.graphics.draw_texture_ex(
      self.texture,
      draw_pos,
      self.rot,
      1,
      self.color
    )
  else
    const render_pos = self:get_top_left_position()

    engine.graphics.draw_rectangle(
      {
        x = render_pos.x,
        y = render_pos.y,
        width = size.x,
        height = size.y
      },
      self.color
    )
  end

  Element.draw(self)
end

return Panel
