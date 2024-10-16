require "crymagick"

def convert_ico(image_path : String) : String?
  puts "[*] Converting icon to .ico format"

  if image_path.ends_with?(".png")
    image_format = "png"
  elsif image_path.ends_with?(".jpg") || image_path.ends_with?(".jpeg")
    image_format = "jpg"
  else
    raise "[-] Unsupported image format. Only \e[1;35m.png\e[0m and \e[1;35m.jpg/.jpeg\e[0m are supported."
  end

  ico_path = image_path.sub(/\.#{image_format}$/, ".ico")

  status = Process.run("magick", [image_path, "-resize", "256x256", ico_path], nil)

  if status.success?
    puts "[+] Icon converted successfully"
    ico_path
  else
    puts "[-] Failed to convert icon"
  end
end
