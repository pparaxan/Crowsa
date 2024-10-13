require "./webview"
require "./macro"

def run_app(config_path : String)
  config = ConfigReader.read(config_path)
  app_instance = App.new
  WebviewRunner.run(config, app_instance)
end
