defmodule Frontend.RustApi do
  require Logger
  alias Frontend.LogEntry
  @rust_api_url "http://localhost:3000/api/v1/logs" # Adjust to match Rust API

  def fetch_logs do
    Logger.info("Fetching logs from Rust API...")
    case HTTPoison.get(@rust_api_url, [], recv_timeout: 5000) do
      {:ok, %HTTPoison.Response{status_code: 200, body: body}} ->
        Logger.info("Rust API Response: #{body}")
        {:ok, Jason.decode!(body)}

      {:ok, %HTTPoison.Response{status_code: code}} ->
        Logger.error("Rust API returned status: #{code}")
        {:error, "Rust API returned status: #{code}"}

      {:error, reason} ->
        Logger.error("Rust API request failed: #{inspect(reason)}")
        {:error, reason}
    end
  end
end
