# frozen_string_literal: true

require 'graphql/client'
require 'graphql/client/http'
require 'json'
require 'base64'
require_relative 'config'

class KibelaClient
    HTTP = GraphQL::Client::HTTP.new("https://#{Config::KIBELA_TEAM_NAME}.kibe.la/api/v1") do
        def headers(context)
            { "Authorization": "Bearer #{Config::KIBELA_ACCESS_TOKEN}",
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

    CommentQuery = Client.parse <<-'GRAPHQL'
    query($id: ID!) {
        comment: comment(id: $id) {
            author {
                id
                account
                avatarImage {
                    url
                }
                url
            }
            id
            publishedAt
            summary: contentSummaryHtml
        }
    }
    GRAPHQL

    def self.get_note path
        Client.query(NoteQuery, variables: { path: path })&.data&.note
    end

    def self.get_comment id
        Client.query(CommentQuery, variables: { id: Base64.strict_encode64("Comment/#{id}") })&.data&.comment
    end
end
