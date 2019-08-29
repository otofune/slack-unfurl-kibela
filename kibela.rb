# frozen_string_literal: true

require 'graphql/client'
require 'graphql/client/http'
require 'json'

KIBELA_TEAM_NAME = ENV["SYAKUSI_KIBELA_TEAM_NAME"]
KIBELA_ACCESS_TOKEN = ENV["SYAKUSI_KIBELA_ACCESS_TOKEN"]

class KibelaClient
    HTTP = GraphQL::Client::HTTP.new("https://#{KIBELA_TEAM_NAME}.kibe.la/api/v1") do
        def headers(context)
            { "Authorization": "Bearer #{KIBELA_ACCESS_TOKEN}",
              "Accept": "application/json" }
        end
    end
    Schema = GraphQL::Client.load_schema(HTTP)
    Client = GraphQL::Client.new(schema: Schema, execute: HTTP)

    NoteQuery = Client.parse <<-'GRAPHQL'
    query($path: String!) {
        note: noteFromPath(path: $path) {
            author {
                id
                account
                avatarImage {
                    url
                }
                url
            }
            id
            title
            url
            publishedAt
            summary: contentSummaryHtml
        }
    }
    GRAPHQL

    def get_note path
        Client.query(NoteQuery, variables: { path: path })&.data&.note
    end
end
