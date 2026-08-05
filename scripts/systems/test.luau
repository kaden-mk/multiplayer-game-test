const MainMenuButton = require("@interface/components/main_menu_button")
const UDim2 = require("@lib/udim2")
const Root = require("@interface/root")

const button = MainMenuButton {
  pos = UDim2(0.5, 0, 0.5, 0),
  text = "PLAY"
}

Root:add(button)
