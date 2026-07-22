export type UIElement = {
    pos: vector, size: vector,
    scale: number,
    visible: boolean,
    children: { UIElement },
    parent: UIElement?,

    add: <T>(self: UIElement, child: T) -> T,
    remove: (self: UIElement, child: UIElement) -> (),
    update: (self: UIElement, dt: number) -> (),
    draw: (self: UIElement) -> ()
}

const Element = {}
Element.__index = Element

function Element.new(): UIElement
    const self = setmetatable({}, Element)
    self.pos = vector.create(0, 0)
    self.size = vector.create(0, 0)
    self.scale = 1
    self.visible = true
    self.children = {}
    self.parent = nil

    return self :: UIElement
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

return Element
