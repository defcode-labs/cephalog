defmodule FrontendWeb.MonitorLive do
  use FrontendWeb, :live_view
  alias Frontend.RustApi

  def mount(_params, _session, socket) do
    logs = case RustApi.fetch_logs() do
      {:ok, logs} -> logs |> Enum.map(fn log -> Map.new(log, fn {k, v} -> {String.to_atom(k), v} end) end)
      {:error, _} -> []
    end

    {:ok, assign(socket, logs: logs, expanded_log_id: nil)}
  end

  # Handle row click
  def handle_event("toggle_log", %{"id" => id}, socket) do
    parsed_id = String.to_integer(id)
    expanded_log_id = if socket.assigns.expanded_log_id == parsed_id, do: nil, else: parsed_id

    {:noreply, assign(socket, expanded_log_id: expanded_log_id)}
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
                <th class="px-4 py-2">Time</th>
                <th class="px-4 py-2">Sourec IP</th>
                <th class="px-4 py-2">Country /ASN</th>
                <th class="px-4 py-2">Event Type</th>
                <th class="px-4 py-2">Targeted Service</th>
                <th class="px-4 py-2">Targeted Endpoint/Port</th>
                <th class="px-4 py-2">Request/Command</th>
                <th class="px-4 py-2">Status</th>
                <th class="px-4 py-2">Action Taken</th>
                <th class="px-4 py-2">Threat Level</th>
              </tr>
            </thead>
            <tbody id="log-entries">
              <%= for log <- @logs do %>
                <tr class="border-t border-gray-700 cursor-pointer hover:bg-gray-700" phx-click="toggle_log" phx-value-id={Integer.to_string(:erlang.phash2(log))}>
                  <td class="px-4 py-2 text-gray-300"><%= log.time %></td>
                  <td class="px-4 py-2 text-gray-300"><%= log.source_ip %></td>
                  <td class="px-4 py-2 text-gray-300"><%= log.country %></td>
                  <td class="px-4 py-2 text-gray-300"><%= log.event_type %></td>
                  <td class="px-4 py-2 text-gray-300"><%= log.targeted_service %></td>
                  <td class="px-4 py-2 text-gray-300"><%= log.targeted_endpoint %></td>
                  <td class="px-4 py-2 text-gray-300"><%= log.request %></td>
                  <td class="px-4 py-2 text-gray-300"><%= log.status %></td>
                  <td class="px-4 py-2 text-gray-300"><%= log.action_taken %></td>
                  <td class="px-4 py-2 font-semibold {@log_class(log.threat_level)}"><%= log.threat_level %></td>
                </tr>

                <!-- Expanded Row
                class={"border-t border-gray-700 bg-gray-900 expanded-row #{if @expanded_log_id == :erlang.phash2(log), do: "show"}"}
                -->
                <%= if @expanded_log_id == :erlang.phash2(log) do %>
                  <tr class="border-t border-gray-700 bg-gray-900">
                    <td colspan="10" class="p-4 text-gray-300">
                    <div class="overflow-hidden transition-all duration-500 ease-in-out" style={"max-height: #{if @expanded_log_id == :erlang.phash2(log), do: "300px", else: "0px"}"}>
                      <p><strong>Time:</strong> <%= log.time %></p>
                      <p><strong>Source IP:</strong> <%= log.source_ip %></p>
                      <p><strong>Country:</strong> <%= log.country %></p>
                      <p><strong>Event Type:</strong> <%= log.event_type %></p>
                      <p><strong>Targeted Service:</strong> <%= log.targeted_service %></p>
                      <p><strong>Endpoint:</strong> <%= log.targeted_endpoint %></p>
                      <p><strong>Request:</strong> <code class="bg-gray-700 p-1 rounded"><%= log.request %></code></p>
                      <p><strong>Status:</strong> <%= log.status %></p>
                      <p><strong>Action Taken:</strong> <%= log.action_taken %></p>
                      <p><strong>Threat Level:</strong> <%= log.threat_level %></p>
                      </div>
                    </td>
                  </tr>
                <% end %>
              <% end %>
            </tbody>
          </table>
        </div>
      </div>
    </div>
    """
  end
end
