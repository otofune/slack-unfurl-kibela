require 'sinatra/base'
require 'net/http'

require './kibela'
SLACK_TOKEN = ENV["SYAKUSI_SLACK_BOT_USER_TOKEN"]

class SyakusiApp < Sinatra::Base
    post '/slack-events' do
        data = JSON.parse request.body.read
        "#{data["id"]}"
    end

    run! if app_file == $0
end
