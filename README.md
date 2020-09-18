# skyrim-mouse-holder
This program centers the mouse while playing Skyrim to avoid accidentally scrolling on secondary monitors.

___Only Supports Windows for now.___

### Installation
Just check the releases on the right and download and run the exe before or after launching your game!

If you don't trust strange exe files, you can clone this repo and build the tool for yourself.

### Developer notes
This program doesn't do any fancy exe/ddl checking, just checks the focused window's title to see if it matches "Skyrim" or "Skyrim Special Edition" and centers the mouse on the screen if it does. So it probably won't work on non-English versions of the game, sorry. But it's easy enough to change, just change the strings in the main.rs file and rebuild your own exe!

I have it move the mouse back to the center of the screen a few times a second, so you could be able to scroll if you scroll while also jerking the mouse to another monitor, but just don't do that. I know I could properly intercept the MouseEvent and recenter based on that, but Rust libraries around that area aren't that great yet and I've already done as much native Windows programming as I want to for now so it will be a task I will think about later.
