# ballbounce
### Terminal ball bouncing in Rust

I'm learning Rust, this is a small and fun project just to challenge myself.

It works! This is the first version, with fixed integer position and velocity:
![ballbounce-v1](img/ballbounce.gif)

How I feel I can challenge myself even more (in order of complexity):

- ~first version, with fixed integer position and velocity~ done in `v0.1.0`
    - defined project structure, with data, main loop and animation steps;
    - implemented Display.
- ~random initial ball position and velocity~ done in `v0.2.0`
    - this has introduced floating point positions and velocity, enabling much more freedom of movements.
- ~several balls at the same time~ done in `v0.3.0`
    - this tests every cell for any ball in there, but Rust is FAST;
    - caches the ball positions in i32s, with only one allocation, to try to reduce impact of the above;
    - I've included a frame counter, to see if the terminal was actually refreshing (it was hard to find those balls! 😅).
- ~random balls color and representation~ done in `v0.4.0`
    - introduces a cool new color system using a macro;
    - removed the ball positions cache, as I needed the actual balls with their positions;
        - switched for a method that does the cast on demand for each ball for each cell (have I said Rust is FAST?);
    - removed the frame counter;
    - known issue: when more than one ball is at the same cell, only the first found one is drawn.
- make the ball #1 always a red ◉, and remove duplications
- detect overlaps and paint differently
- commands to dynamically insert and remove balls
- command to reset colors and formats, maintaining the board
- make the balls leave a trail
- include CLI arguments (clap or structopt) for board size, initial number of balls and fps
- implement trail behaviors, like decaying and following (snake!)
- implement collision behaviors, like destroy and explode (more balls are generated)
- implement movement behaviors, like acceleration, parabola and even wander
- walls and other obstacles inside the board
- maybe at this point it could even turn into a game! a breakout or pong or snake or asteroid of some sort
- make the balls collide with each other
- balls of different shapes (n x m chars)

Enjoy 😊
