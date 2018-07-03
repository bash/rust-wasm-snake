const module = import('../wasm/rust_wasm_snake')

module.then((module) => {
  const { KeyboardButton, Game } = module

  const mapKeyToButton = (key: string) => {
    switch (key) {
      case 'ArrowUp':
        return KeyboardButton.Up
      case 'ArrowRight':
        return KeyboardButton.Right
      case 'ArrowDown':
        return KeyboardButton.Down
      case 'ArrowLeft':
        return KeyboardButton.Left
      case 'Escape':
        return KeyboardButton.Escape
    }
  }

  const $canvas = document.getElementById('game-canvas') as HTMLCanvasElement
  const context = $canvas.getContext('2d')
  const width = window.innerWidth
  const height = window.innerHeight

  Object.assign($canvas, { width, height })

  const game = Game.new(width, height, context)

  const tick = () => {
    const sleepDuration = game.tick()

    setTimeout(tick, sleepDuration)
  }

  document.addEventListener('keydown', (event) => {
    const button = mapKeyToButton(event.key)

    if (button != null) {
      game.on_key_press(button)
      event.preventDefault()
    }
  })

  tick()
})