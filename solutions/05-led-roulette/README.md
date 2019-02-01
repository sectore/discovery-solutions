
# Challenge `05-led-roulette`

Challenge description: https://docs.rust-embedded.org/discovery/05-led-roulette/the-challenge.html

## How to run

From `/tmp` folder:
```bash
openocd \
  -f interface/stlink-v2-1.cfg \
  -f target/stm32f3x.cfg
```

From project folder (in another Terminal):
```
cargo run --target thumbv7em-none-eabihf
```

## Result

![Screencast](./screencast.gif)