class
mod Lego::Events

fields = [
    0x0 = _: ptr void,
    0x4 = _: ptr void,
]
virtuals = [
    0 = IEventListener_dtor: thiscall () void,
    1 = RecieveEvent: thiscall (event: ptr Event, data: ptr NuEventData) void,
    2 = _: thiscall (unk: ptr void) bool,
]