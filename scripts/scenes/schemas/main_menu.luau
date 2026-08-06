const Root = require("@interface/root")
const Manager = require("@scenes/manager")
const Text = require("@interface/elements/text")
const MainMenuButton = require("@interface/components/main_menu_button")
const UDim2 = require("@lib/udim2")

const track = engine.assets.load_music_stream("music/main_menu.mp3")
engine.audio.set_music_stream_volume(track, 0.2)

const font = engine.assets.load_font("fonts/PixeloidMono.ttf")

local title
local button

const function reset_menu()
    if title then
        title:destroy()
    end

    if button then
        button:destroy()
    end
end

const function create_menu()
    reset_menu()

    button = MainMenuButton {
      pos = UDim2(0.5, 0, 0.6, 0),
      text = "PLAY"
    }

    title = Text {
      text = "MULTIPLAYER GAME TEST",
      font = font,
      font_size = 9 * 5,
      pos = UDim2(0.5, 0, 0.5, 0),
      is_relative = true,
      anchor = vector.create(0.5, 0.5)
    }

    Root:add(title)
    Root:add(button)
end

return {
  enter = function()
      create_menu()
      
      engine.audio.play_music_stream(track)
  end,

  on_update = function(dt: number)
      engine.audio.update_music_stream(track)
  end,

  on_draw = function()

  end,

  exit = reset_menu
} :: Manager.Scene
