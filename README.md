# ballbounce
### Terminal ball bouncing in Rust

I'm learning Rust, this is a small and fun project just to challenge myself.

It works!

![demo](img/ballbounce.gif)

What I feel I could do next (in order of complexity):

- ~random initial ball position and velocity~ done in `v0.2.0`
- ~several balls at the same time~ done in `v0.3.0`
- ~random balls color and representation~ done in `v0.4.0`
    => known issue: when more than one ball is at the same cell, only the first one is drawn
- make the first ball always a red ◉, and remove duplications
- detect collisions and paint differently
- dynamically insert or remove balls
- make the balls leave a trail
- include CLI arguments (clap or structopt) for board size, initial number of balls and fps
- implement trail behaviors, like decaying and following (snake!)
- implement collision behaviors, like destroy and explode (more balls are generated)
- walls and other obstacles inside the board
- maybe at this point it could even turn into a game! a breakout or pong of some sort
- make the balls collide with each other
- balls of different shapes (n x m chars)

Enjoy 😊
