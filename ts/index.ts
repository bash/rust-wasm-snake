const module = import('../wasm/rust_wasm_snake')

module.then((module) => {
  const $canvas = document.getElementById('game-canvas') as HTMLCanvasElement
  const context = $canvas.getContext('2d')
  const width = window.innerWidth
  const height = window.innerHeight

  Object.assign($canvas, { width, height })

  const game = module.Game.new(width, height)

  const tick = () => {
    const sleepDuration = game.tick(context)

    setTimeout(tick, sleepDuration)
  }

  tick()
})