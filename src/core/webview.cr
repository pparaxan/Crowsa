require "base64"
require "hyaline"
require "json"

class ConfigReader
  def self.read(config_path : String)
    JSON.parse(File.read(config_path))
  end
end

class WebviewRunner
  def self.run(config, app_instance)
    app_name = config["main"]["name"].as_s
    width = config["main"]["width"].as_i
    height = config["main"]["height"].as_i
    icon_path = config["package"]["icon"].as_s

    encoded_html_content = Base64.strict_encode(app_instance.to_s)
    debug_mode = {% if flag?(:release) %}
                   false
                 {% else %}
                   true
                 {% end %}

    webview = Webview.window(width, height, Webview::SizeHints::NONE, app_name, "data:text/html;base64,#{encoded_html_content}", debug = debug_mode)
    webview.run
    webview.destroy
  end
end
