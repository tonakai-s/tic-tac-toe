# tic-tac-toe

This is a simple tic-tac-toe game to run on terminal and play with another person on the same computer or the same network (LAN).

## Purpose

Learn more about Rust files structure, furter I wanna change all clones on this code to smart pointers and learn more about it.
Some code can be maked with another approach on this project, but I created this implementation by myself, so I'm happy with the result.

This game is played on the LAN (Two machines on the same network).

Just execute the game (With cargo run or the builded file) passing `nick` argument.
```bash
$ cargo run -- --nick=renas
```

When make this, the WebSocket server(Wich control and manage the Game State) will be created and stay running on a separated thread, leaving the main to run for the Host client player.
So the game will show to you some argument which the Guest player need to pass to join the game.

### Notes
- Draw match not implemented
- A LOT of errors when someone disconnects are not handled
- I would like to implement spectators clients, but Idk if I will do this. Because I stop progressing this project. I don't see future on still make this useless thing, and I still care about other people's comments... So, it is what it is...

Thanks for watching! (ღ˘⌣˘ღ)
