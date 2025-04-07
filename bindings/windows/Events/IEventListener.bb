class
mod Lego::Events

fields = [
    0x0: ptr void,
    0x4: ptr void,
]
virtuals = [
    0: thiscall IEventListener_dtor(self) void,
    1: thiscall RecieveEvent(self, event: ptr Event, data: ptr NuEventData),
    2: thiscall vfunction3(self, unk: ptr void),
]