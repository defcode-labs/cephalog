defmodule Frontend.LogEntry do
  defstruct [
    :time,
    :source_ip,
    :country,
    :asn,
    :event_type,
    :targeted_service,
    :targeted_endpoint,
    :request,
    :status,
    :action_taken,
    :threat_level
  ]
end
