const Manager = require("@scenes/manager")
const Text = require("@interface/elements/text")
const MainMenuButton = require("@interface/components/main_menu_button")
const Element = require("@interface/element")
const UDim2 = require("@lib/udim2")

const track = engine.assets.load_music_stream("music/main_menu.mp3")
engine.audio.set_music_stream_volume(track, 0.2)

const font = engine.assets.load_font("fonts/PixeloidMono.ttf")

const function create_menu(container: Element.UIElement)
    container:add(MainMenuButton {
      pos = UDim2(0.5, 0, 0.6, 0),
      text = "PLAY"
    })

    container:add(Text {
      text = "MULTIPLAYER GAME TEST",
      font = font,
      font_size = 9 * 5,
      pos = UDim2(0.5, 0, 0.5, 0),
      is_relative = true,
      anchor = vector.create(0.5, 0.5)
    })
end

return {
  enter = function(container: Element.UIElement)
      create_menu(container)
      engine.audio.play_music_stream(track)
  end,

  on_update = function(dt: number)
      engine.audio.update_music_stream(track)
  end,

  on_draw = function()

  end,

  exit = function()
      engine.audio.stop_music_stream(track)
  end
} :: Manager.Scene
