require 'graphql/client'
require 'graphql/client/http'
require 'json'
require 'pry'

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

    NotesQuery = Client.parse <<-'GRAPHQL'
    query($first: Int!) {
        notes(first: $first, orderBy: {field: PUBLISHED_AT, direction: ASC}) {
            nodes {
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
            }
        }
    }
    GRAPHQL

    def self.notes_first(first)
        Client.query(NotesQuery, variables: { first: first })
    end

    def self.note(id)
        r = self.notes_first id
        return r.data.notes.nodes.find{ |n| n.url.end_with? "/#{id}" }
    end
end

if __FILE__ == $0
    puts "YEE"
    binding.pry
    puts "YEE"
end
