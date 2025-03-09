defmodule FrontendWeb.MonitorLive do
  use FrontendWeb, :live_view
  alias Frontend.RustApi

  def mount(_params, _session, socket) do
    # Simulated logs for now, replace with actual Rust API call later
    # logs = [
    #   %{timestamp: "2025-03-09 12:30:00", level: "INFO", message: "System startup complete."},
    #   %{timestamp: "2025-03-09 12:31:05", level: "WARNING", message: "Unusual activity detected on port 22."},
    #   %{timestamp: "2025-03-09 12:31:30", level: "ERROR", message: "Failed SSH login attempt from 192.168.1.100."}
    # ]
    logs = case RustApi.fetch_logs() do
      {:ok, logs} -> logs
      {:error, _} -> []
    end

    {:ok, assign(socket, logs: logs)}
  end

  def render(assigns) do
    ~H"""
    <div class="flex h-screen bg-gray-900 text-gray-200">
      <!-- Sidebar -->
      <div class="w-64 bg-gray-800 p-5">
        <div class="text-xl font-bold mb-5 text-green-400">Cephalog</div>
        <nav class="space-y-2">
          <a href="#" class="block px-3 py-2 rounded bg-gray-700 text-white">Dashboard</a>
          <a href="#" class="block px-3 py-2 rounded bg-gray-700 text-white">Logs</a>
          <a href="#" class="block px-3 py-2 rounded bg-gray-700 text-white">Alerts</a>
          <a href="#" class="block px-3 py-2 rounded bg-gray-700 text-white">Settings</a>
        </nav>
      </div>

      <!-- Main Content -->
      <div class="flex-1 p-6">
        <h1 class="text-2xl font-semibold text-green-300">Live Log Monitoring</h1>

        <div class="mt-5 bg-gray-800 rounded p-4 shadow-md">
          <p class="text-gray-400">Watching logs in real-time...</p>
        </div>

        <!-- Log Table -->
        <div class="mt-5">
          <table class="w-full table-auto bg-gray-800 rounded">
            <thead>
              <tr class="text-left bg-gray-700">
                <th class="px-4 py-2">Timestamp</th>
                <th class="px-4 py-2">Level</th>
                <th class="px-4 py-2">Message</th>
              </tr>
            </thead>
            <tbody id="log-entries">
              <%= for log <- @logs do %>
                <tr class="border-t border-gray-700">
                  <td class="px-4 py-2 text-gray-300"><%= log.timestamp %></td>
                  <td class="px-4 py-2 font-semibold {@log_class(log.level)}"><%= log.level %></td>
                  <td class="px-4 py-2"><%= log.message %></td>
                </tr>
              <% end %>
            </tbody>
          </table>
        </div>
      </div>
    </div>
    """
  end

  defp log_class("INFO"), do: "text-blue-400"
  defp log_class("WARNING"), do: "text-yellow-400"
  defp log_class("ERROR"), do: "text-red-400"
end
