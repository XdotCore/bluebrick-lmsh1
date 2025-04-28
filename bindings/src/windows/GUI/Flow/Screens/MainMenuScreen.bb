class
mod Lego::GUI
use Lego::Events

fields = [
    0x00 = page: ptr GUI2Page,
    0x04 = bitfield_0x4: u8,
    0x08 = menu_entries: arr 12 ptr GUI2MenuEntry,
    0x38 = new_game_menu_entry: ptr GUI2MenuEntry,
    0x3c = freeplay_menu_entry: ptr GUI2MenuEntry,
    0x40 = select_level_menu_entry: ptr GUI2MenuEntry,
    0x44 = show_controls_menu_entry: ptr GUI2MenuEntry,
    0x48 = xbox_live_marketplace_menu_entry: ptr GUI2MenuEntry,
    0x4c = coming_soon_menu_entry: ptr GUI2MenuEntry,
    0x50 = load_game_menu_entry: ptr GUI2MenuEntry,
    0x54 = network_game_menu_entry: ptr GUI2MenuEntry,
    0x58 = options_menu_entry: ptr GUI2MenuEntry,
    0x5c = quit_to_windows_menu_entry: ptr GUI2MenuEntry,
    0x60 = language_menu_entry: ptr GUI2MenuEntry,
    0x64 = help_menu_entry: ptr GUI2MenuEntry,
    0x68 = selected_menu_entry: ptr GUI2MenuEntry,
    0x6c = _: i16,
    0x70 = menu_entry_text_overrides: arr 12 GUI2MenuEntryTextOverrides,
    0xd0 = _: bool,
    0xd4 = _: i32,
    0xd8 = _: i32,
    0xdc = _: i32,
    0xe0 = _: i32,
    0xe8 = _: bool,
    0xec = _: i32,
    0xf0 = _: i32,
    0xf4 = _: i32,
]
implements = [
    FlowPageHandler2 = 0xDD5948,
    IEventListener = 0xDD5938,
]
overrides = [
    FlowPageHandler2 = {
        GUI2PageHandler_dtor,
        Update,
    },
    IEventListener = {
        IEventListener_dtor,
        RecieveEvent,
    },
]
virtuals = [
    0 = thiscall GetGUI2PageHandler_int_0xc(self) i32,
    1 = stdcall Return2() i32,
    2 = thiscall SetGUI2PageHandler_int_0xc(self, val: i32) void,
]
