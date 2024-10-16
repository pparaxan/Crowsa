require "option_parser"
require "json"
require "./cli/installer/windows/main"
require "./core/main"
require "kemal"

module Crowsa
  class CLI
    def arguments
      OptionParser.parse do |parser|
        parser.on "--package", "Creates an installer for your application" do
          package_app
          exit
        end
        parser.on "--hot-reload", "Hot reload your application with Kemal" do
          hot_reload
          exit
        end
        parser.on "-h", "--help", "Gosh help me, i wanna kill myself" do
          puts parser
          exit
        end
      end
    end

    def package_app
      config = JSON.parse(File.read("config.json"))
      app_name = config["main"]["name"].as_s
      app_target = config["main"]["target"].as_s
      icon_path = config["package"]["icon"].as_s

      Windows.package(app_name, app_target, icon_path)
    end

    def hot_reload
      Kemal.config.logging = false

      get "/" do
        puts "Serving request on /"
        App.new.to_s
      end

      Kemal.run(args: nil)
    end
  end
end
