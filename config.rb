module Config
    KIBELA_TEAM_NAME = ENV["SYAKUSI_KIBELA_TEAM_NAME"]
    KIBELA_ACCESS_TOKEN = ENV["SYAKUSI_KIBELA_ACCESS_TOKEN"]
    SLACK_TOKEN = ENV['SYAKUSI_SLACK_BOT_USER_TOKEN']
    SLACK_UNFURL_API = URI.parse('https://slack.com/api/chat.unfurl')

    if KIBELA_TEAM_NAME.nil? || KIBELA_ACCESS_TOKEN.nil? || SLACK_TOKEN.nil?
        raise ArgumentError, "This app requires some environment variables."
    end
end
