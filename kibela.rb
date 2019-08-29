require 'graphql/client'
require 'graphql/client/http'
require 'json'

module KibelaClient
    KIBELA_TEAM_NAME = ENV["SYAKUSI_KIBELA_TEAM_NAME"]
    KIBELA_ACCESS_TOKEN = ENV["SYAKUSI_KIBELA_ACCESS_TOKEN"]

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
            }
            id
            title
            url
            publishedAt
            contentSummaryHtml
        }
    }
    GRAPHQL

    def self.note(id)
        Client.query(NoteQuery, variables: { path: "/notes/#{id}" }).data.note
    end
end
