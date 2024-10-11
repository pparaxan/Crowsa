require "base64"

macro encode(contents)
  {% if contents.is_a?(StringLiteral) %}
    %file_path = {{contents}}
    %file_content = File.read(%file_path)
    %extension = File.extname(%file_path).downcase
    %mime_type = case %extension
      when ".png"  then "image/png"
      when ".jpg"  then "image/jpeg"
      when ".jpeg" then "image/jpeg"
      when ".svg"  then "image/svg+xml"
      when ".css"  then "text/css"
      when ".js"   then "text/javascript"
      else              "text/plain"
    end
    "data:#{%mime_type};base64,#{Base64.strict_encode(%file_content)}"
  {% else %}
    "data:text/plain;base64,#{Base64.strict_encode({{contents}})}"
  {% end %}
end
