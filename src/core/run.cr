require "webview"
require "base64"
require "json"

def run_app(config_path : String)
  config = JSON.parse(File.read("config.json"))
  app_name = config["main"]["name"].as_s
  width = config["main"]["width"].as_i
  height = config["main"]["height"].as_i
  icon_path = config["package"]["icon"].as_s

  app_instance = App.new
  html_content = app_instance.to_s
  encoded_html = Base64.strict_encode(html_content)

  debug_mode = {% if flag?(:release) %}
                 false
               {% else %}
                 true
               {% end %}

  webview = Webview.window(width, height, Webview::SizeHints::NONE, app_name, "data:text/html;base64,#{encoded_html}", debug = debug_mode)
  webview.run

  if debug_mode
    puts "Press \x1b[1;36mEnter\x1b[0m or do \x1b[1;36mCtrl + C\x1b[0m to fully quit."
    gets
  end

  webview.destroy
end
