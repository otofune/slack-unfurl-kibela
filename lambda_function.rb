# MIT No Attribution

# Permission is hereby granted, free of charge, to any person obtaining a copy of this
# software and associated documentation files (the "Software"), to deal in the Software
# without restriction, including without limitation the rights to use, copy, modify,
# merge, publish, distribute, sublicense, and/or sell copies of the Software, and to
# permit persons to whom the Software is furnished to do so.

# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
# INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
# PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
# HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
# OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
# SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

# memo(otofune): Based on https://github.com/aws-samples/serverless-sinatra-sample/blob/71c8e849a619a8fea169e328b93a7434054e86fa/lambda.rb

require 'json'
require 'rack'
require 'base64'

$app ||= Rack::Builder.parse_file("#{__dir__}/config.ru").first
ENV['RACK_ENV'] ||= 'production'

def lambda_handler(event:, context:)
  body = if event['isBase64Encoded']
    Base64.decode64 event['body'] if event['body']
  else
    event['body']
  end || ''

  headers = event.fetch 'headers', {}

  env = {
    'REQUEST_METHOD' => event.fetch('httpMethod'),
    'SCRIPT_NAME' => '',
    'PATH_INFO' => '/', # dirty
    'QUERY_STRING' => Rack::Utils.build_query(event['queryStringParameters'] || {}),
    'SERVER_NAME' => headers.fetch('Host', 'localhost'),
    'SERVER_PORT' => headers.fetch('X-Forwarded-Port', 443).to_s,

    'rack.version' => Rack::VERSION,
    'rack.url_scheme' => headers.fetch('CloudFront-Forwarded-Proto') { headers.fetch('X-Forwarded-Proto', 'https') },
    'rack.input' => StringIO.new(body),
    'rack.errors' => $stderr,
  }

  headers.each_pair do |key, value|
    name = key.upcase.gsub '-', '_'
    header = case name
      when 'CONTENT_TYPE', 'CONTENT_LENGTH'
        name
      else
        "HTTP_#{name}"
    end
    env[header] = value.to_s
  end

  begin
    status, headers, body = $app.call env

    body_content = ""
    body.each do |item|
      body_content += item.to_s
    end

    response = {
      'statusCode' => status,
      'headers' => headers,
      'body' => body_content
    }
  rescue Exception => exception
    response = {
      'statusCode' => 500,
      'body' => exception.message
    }
  end

  response
end
