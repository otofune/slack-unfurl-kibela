# frozen_string_literal: true

require 'sinatra/base'
require 'net/http'
require 'net/https'
require 'uri'
require 'time'
require 'json'

require_relative 'config'
require_relative 'kibela'

class SyakusiApp < Sinatra::Base
    post '/' do
        params = JSON.parse request.body.read

        case params['type']
        when 'url_verification'
            challenge = params['challenge']
            return { challenge: challenge }.to_json
        when 'event_callback'
            return unless params.dig('event', 'type') == 'link_shared'

            channel = params.dig('event', 'channel')
            ts = params.dig('event', 'message_ts')
            links = params.dig('event', 'links')

            unfurls = {
                channel: channel,
                ts: ts,
                unfurls: {}
            }
            links.each do |link|
                url = URI.parse link['url']
                next unless url.path.start_with?("/notes/") || url.path.start_with?("/@")

                note = KibelaClient.get_note(url.path)

                if url.fragment&.start_with?('comment_')
                    match = /^comment_(?<id>\d+)/i.match url.fragment
                    if not match.nil?
                        id = match.named_captures['id']
                        comment = KibelaClient.get_comment(id)
                        attachment = {
                            author_link: comment.author.url,
                            author_name: "@#{comment.author.account}",
                            title: "「#{note.title}」へのコメント",
                            title_link: link['url'],
                            # Kibela サイトでの挙動 (ブラウザーがホワイトスペースを詰める) にあわせる
                            text: comment.summary.gsub(/\s+/, ' '),
                            footer: 'Kibela',
                            footer_icon: 'https://cdn.kibe.la/assets/shortcut_icon-99b5d6891a0a53624ab74ef26a28079e37c4f953af6ea62396f060d3916df061.png',
                            ts: Time.parse(note.published_at).to_i
                        }
                        unfurls[:unfurls][url] = attachment
                        next
                    end
                end

                attachment = {
                    author_link: note.author.url,
                    author_name: "@#{note.author.account}",
                    title: note.title,
                    title_link: note.url,
                    # Kibela サイトでの挙動 (ブラウザーがホワイトスペースを詰める) にあわせる
                    text: note.summary.gsub(/\s+/, ' '),
                    footer: 'Kibela',
                    footer_icon: 'https://cdn.kibe.la/assets/shortcut_icon-99b5d6891a0a53624ab74ef26a28079e37c4f953af6ea62396f060d3916df061.png',
                    ts: Time.parse(note.published_at).to_i
                }
                unfurls[:unfurls][url] = attachment
            end

            req = Net::HTTP::Post.new(Config::SLACK_UNFURL_API.path)
            req.body = unfurls.to_json
            req['Content-Type'] = 'application/json'
            req['Authorization'] = "Bearer #{Config::SLACK_TOKEN}"

            https = Net::HTTP.new(Config::SLACK_UNFURL_API.host, Config::SLACK_UNFURL_API.port)
            https.use_ssl = true
            res = https.request(req)
        else
            halt 400, {}.to_json
        end

        return {}.to_json
    end

    run! if app_file == $0
end
