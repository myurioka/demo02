# demo02
Rust WebAssembly Demo02 (edition:2024)
=======
demo02 🎨
========
Programming mini game for Demo in Rust & WebAssembly

[![screenshot](screen.png)](https://myurioka.github.io/demo02/)

[Play in browser](https://myurioka.github.io/demo02)

### How to play (Control)

  * Click Rectangle: Get number

### Requirement
  * Rust, Cargo
  * WASM

### How to Build & Run

  ```sh
  $ cd demo02
  $ pnpm build-wasm
  $ pnpm dev --open
  ```
  Browse http://localhost:5173

### Sequence Diagram

```mermaid
sequenceDiagram
    autonumber
    participant B as Browser
    participant H as heap
    participant R as Rust
    R->>H: Game impl Trait + 'static
    note over H: Game
    R->>H: Point(RefCell::new(Point))
    note over H: Point
    R->>H: Closure::wrap(Box::new(|_time:f64|()))
    H->>B: requestnimation()
    loop callback GAME.on_animation_frame
    B->>H: callback
    H->>H: Game.update()
    H->>H: Game.draw()
    H->>B: requestnimation()
    end
    R->>H: Closure::wrap(Box::new(|MouseEvnet|)())
    H->>B: add_event_listner_with_callback("mousedown")
    alt callback GAME.on_click
    H->>H: forget()
    B->>H: callback
    H->>H: Update Point(MouseEvent)
    end
```
<br>
<ol>
<li>Game Object impl Trait + 'static for deprecation of global static mut👍 </li>
<li>set interface function(closure) for requestAnimationFrame <br/> 👍RefCell< T > and the Interior Mutablilly Pattern<br/> After the first requestAnimationFrame call, the closure is disappointed</li>
<li>requestAnimationFrame(interface function for callback) in first</li>
<li>callback → Closure::wrap(Box::new(|_time:f64|())) 
<li>game update</li>
<li>game draw</li>
<li>requestAnimationFrame(interface function for callback) in loop</li>
<li>set interface function(closure) for MouseEvent at MouseDown</li>
<li>canvas.add_event_listener_with_callback("moudsedown")</li>
<li>forget() to keep interface function(closure)👍</li>
<li>MouseEvent(callback)</li>
<li>game set click position</li>
</ol>