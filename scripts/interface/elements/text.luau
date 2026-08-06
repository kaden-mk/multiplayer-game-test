const Element = require("@interface/element")
const UDim2 = require("@lib/udim2")
type UDim2 = UDim2.UDim2

export type UITextProps = {
  pos: UDim2?,
  is_relative: boolean?,
  color: Color?,
  anchor: vector?,
  spacing: number?,
  font: string?,
  font_size: number,
  text: string,
}

export type UIText = Element.UIElement & {
  font_size: number,
  text: string,
  color: Color,
  font: string?,
  spacing: number
}

const Text = Element:extend()

function Text.new(props: UITextProps): UIText
  props = props or {} :: UITextProps

  if not props.text then error("No text was provided!") end
  if not props.font_size then error("No font size was provided!") end

  const self = Element.new(Text) :: UIText
  self.pos = props.pos or UDim2(0, 0, 0, 0)
  self.font_size = props.font_size
  self.is_relative = props.is_relative or false
  self.text = props.text
  self.color = props.color or Color.new("WHITE")
  self.anchor = props.anchor or vector.create(0, 0)
  self.font = props.font
  self.spacing = props.spacing or 0

  return self
end

function Text.draw(self: UIText)
  if not self.font then
    const width = engine.graphics.measure_text(self.text, self.font_size)
    self.size = UDim2(0, width, 0, self.font_size)

    const render_pos = self:get_top_left_position()

    engine.graphics.draw_text(
      self.text,
      render_pos.x,
      render_pos.y,
      self.font_size,
      self.color
    )
  else
    const bounds = engine.graphics.measure_text_ex(
      self.font,
      self.text,
      self.font_size,
      self.spacing
    )
    self.size = UDim2(0, bounds.x, 0, bounds.y)

    const render_pos = self:get_top_left_position()

    engine.graphics.draw_text_ex(
      self.font,
      self.text,
      render_pos,
      self.font_size,
      self.spacing,
      self.color
    )
  end

  Element.draw(self)
end

return Text
