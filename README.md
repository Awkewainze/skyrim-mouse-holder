# skyrim-mouse-holder
This program centers the mouse while playing Skyrim to avoid accidently scrolling on secondary monitors.

_Only Supports Windows for now._

It doesn't do any fancy exe/ddl checking, just checks the focused window's title to see if it matches "Skyrim" or "Skyrim Special Edition" and centers the mouse on the screen if it does.

I have it move the mouse back to the center of the screen a few times a second, so you could be able to scroll if you scroll wheel while also jerk the move to another monitor, but just don't do that.

I know I could properly intercept the MouseEvent and recenter based on that, but Rust libraries around that area aren't that great and I've already done as much native Windows programming as I want to for now. Maybe later.
