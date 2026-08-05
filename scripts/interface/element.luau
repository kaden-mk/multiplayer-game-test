const UDim2 = require("@lib/udim2")
type UDim2 = UDim2.UDim2

export type UIElement = {
    pos: UDim2,
    size: UDim2,
    anchor: vector,
    visible: boolean,
    is_relative: boolean,
    children: { UIElement },
    parent: UIElement?,

    add: <T>(self: UIElement, child: T) -> T,
    remove: (self: UIElement, child: UIElement) -> (),
    update: (self: UIElement, dt: number) -> (),
    draw: (self: UIElement) -> (),
    get_anchor_offset: (self: UIElement) -> vector,
    get_top_left_position: (self: UIElement) -> vector,
    get_parent_size: (self: UIElement) -> vector,
    get_absolute_position: (self: UIElement) -> vector,
    get_absolute_size: (self: UIElement) -> vector
}

const Element = {}
Element.__index = Element

function Element.new(self): UIElement
    const element = setmetatable({}, self)
    element.pos = UDim2(0, 0, 0, 0)
    element.size = UDim2(0, 0, 0, 0)
    element.anchor = vector.create(0, 0)
    element.visible = true
    element.is_relative = false
    element.children = {}
    element.parent = nil

    return element
end

function Element.add<T>(self: UIElement, child: T)
    const element = child :: UIElement
    table.insert(self.children, element)
    element.parent = self

    return child
end

function Element.remove(self: UIElement, child: UIElement)
    const index = table.find(self.children, child)
    if not index then
        error("Child does not exist!")
    end

    table.remove(self.children, index)
end

function Element.get_anchor_offset(self: UIElement): vector
  const size = self:get_absolute_size()
  return vector.create(
    size.x * self.anchor.x,
    size.y * self.anchor.y
  )
end

function Element.get_top_left_position(self: UIElement): vector
  const pos = self:get_absolute_position()
  const offset = self:get_anchor_offset()
  return vector.create(pos.x - offset.x, pos.y - offset.y)
end

function Element.get_parent_size(self: UIElement): vector
  if self.parent then
    return self.parent:get_absolute_size()
  end

  return vector.create(1280, 720) -- HARDCODED FOR NOW, WILL ADD SCREEN FUNCTION
end

function Element.get_absolute_size(self: UIElement): vector
  const parent_size = self:get_parent_size()
  
  return vector.create(
    (parent_size.x * self.size.x.scale) + self.size.x.offset,
    (parent_size.y * self.size.y.scale) + self.size.y.offset
  )
end

function Element.get_absolute_position(self: UIElement): vector
  const parent_size = self:get_parent_size()

  const local_x = (parent_size.x * self.pos.x.scale) + self.pos.x.offset
  const local_y = (parent_size.y * self.pos.y.scale) + self.pos.y.offset

  if self.is_relative and self.parent then
    const pos = self.parent:get_top_left_position() 
    
    return vector.create(
      pos.x + local_x,
      pos.y + local_y
    )
  end

  return vector.create(local_x, local_y)
end

function Element.update(self: UIElement, dt: number)
    for _, child in self.children do
        if child.visible then
            child:update(dt)
        end
    end
end

function Element.draw(self: UIElement)
    for _, child in self.children do
        if child.visible then
            child:draw()
        end
    end
end

function Element:extend()
  const class = {}
  class.__index = class

  setmetatable(class, {
    __index = self,
    __call = function(c, ...)
      return c.new(...)
    end
  })

  return class
end

return Element
