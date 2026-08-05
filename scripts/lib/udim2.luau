export type UDim2 = {
  x: { scale: number, offset: number },
  y: { scale: number, offset: number }
}

const function udim2(sx: number, ox: number, sy: number, oy: number): UDim2
  return {
    x = { scale = sx, offset = ox },
    y = { scale = sy, offset = oy }
  }
end

return udim2
