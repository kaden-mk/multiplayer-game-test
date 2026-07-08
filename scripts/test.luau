const Test = {}

function Test.on_update(dt: number)
    if input.is_key_down("A") then
        draw.text("Yo", 12, 12, 20, "WHITE")
    end
end

return Test
