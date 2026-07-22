const math_util = {}

function math_util.is_point_in_rect(p: vector, pos: vector, size: vector): boolean
    return p.x >= pos.x and p.x <= pos.x + size.x
       and p.y >= pos.y and p.y <= pos.y + size.y
end

return math_util
