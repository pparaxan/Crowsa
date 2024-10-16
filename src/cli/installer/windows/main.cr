require "process"
require "file_utils"
require "./ico_conversion"

module Windows
  extend self

  def package(app_name : String, app_target : String, icon_path : String)
    puts "Packaging for Windows..."

    puts "[*] Getting icon..."
    ico_path = if icon_path.ends_with?(".ico")
                 icon_path
               else
                 convert_ico(icon_path)
               end

    if ico_path.nil?
      puts "[-] Failed to convert icon to .ico format"
      return
    end

    puts "[+] Building your application in release mode"
    system "shards build #{app_target} --release -o \"#{app_name}.exe\""

    puts "[*] Creating a NSIS installer script"
    File.write("install.nsi", <<-NSIS
      Name "#{app_name}"
      OutFile "#{app_name} Installer.exe"
      InstallDir "$PROGRAMFILES\\#{app_name}"
      Icon "#{ico_path}"

      Section "Install"
        SetOutPath $INSTDIR
        File "#{app_name}.exe"
        File "config.json"
        File "#{ico_path}"
        CreateShortCut "$DESKTOP\\#{app_name}.lnk" "$INSTDIR\\#{app_name}.exe" "" "$INSTDIR\\#{File.basename(ico_path)}"
      SectionEnd
    NSIS
    )

    puts "[+] Building the .exe"
    system "makensis install.nsi"
    puts "[*] Installer created, \e[1;37m#{app_name} Installer.exe\e[0m"

    puts "[*] Removing the \e[0;34minstall.nsi\e[0m file"
    File.delete("install.nsi")
  end
end
