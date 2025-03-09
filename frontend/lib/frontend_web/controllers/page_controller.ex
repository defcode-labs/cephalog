defmodule FrontendWeb.PageController do
  use FrontendWeb, :controller

  def home(conn, _params) do
    # The home page is often custom made,
    # so skip the default app layout.
    render(conn, :home, layout: false)
  end

  def dashboard(conn, _params) do
    # The dashboard page is often custom made,
    # so skip the default app layout.
    render(conn, :dashboard, layout: false)
  end
end
