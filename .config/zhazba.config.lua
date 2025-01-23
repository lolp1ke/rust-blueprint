Config:add_insert_buffer_mode("i")


Config:keymap("ctrl-s", "n", KeyAction:Single(Action:Save()))
Config:keymap("ctrl-s", "i", KeyAction:Single(Action:Save()))

Config:keymap("i", "n", KeyAction:Single(Action:EnterMode("i")))
Config:keymap("esc", "i", KeyAction:Single(Action:EnterMode("n")))


Config:keymap("q", "n", KeyAction:Single(Action:Quit(false)))
Config:keymap("shiftctrl-q", "n", KeyAction:Single(Action:Quit(true)))
