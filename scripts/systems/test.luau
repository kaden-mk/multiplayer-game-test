const MainMenuButton = require("@interface/components/main_menu_button")
const text = require("@interface/elements/text")
const UDim2 = require("@lib/udim2")
const Root = require("@interface/root")
const scheduler = require("@lib/scheduler")

const track = engine.assets.load_music_stream("assets/main_menu.mp3")
engine.audio.play_music_stream(track)
engine.audio.set_music_stream_volume(track, 0.2)

scheduler.add_on_update(function(dt: number)
    engine.audio.update_music_stream(track)
end, 20)

const font = engine.assets.load_font("assets/PixeloidMono.ttf")
const title = text {
  text = "MULTIPLAYER GAME TEST",
  font = font,
  font_size = 9 * 5,
  pos = UDim2(0.5, 0, 0.5, 0),
  is_relative = true,
  anchor = vector.create(0.5, 0.5)
}

const button = MainMenuButton {
  pos = UDim2(0.5, 0, 0.6, 0),
  text = "PLAY"
}

Root:add(title)
Root:add(button)
