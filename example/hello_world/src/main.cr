require "blueprint/html"
require "crowsa/main"
require "./css"
require "./javascript"

class App
  include Blueprint::HTML

  private def blueprint
    meta charset: "UTF-8"
    style { LocalCSS }
    script src: encode(LocalJavascipt)

    div class: "container" {
      h1 { "Hello World!" }
      button(id: "greetButton") { "Click Me" }
      div id: "message"
    }
  end
end

run_app("config.json")
