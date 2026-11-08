import { render, screen } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { MemoryRouter } from "react-router";
import App from "@/App";

describe("App shell", () => {
  it("renders the navigation and the library page by default", () => {
    render(
      <QueryClientProvider client={new QueryClient()}>
        <MemoryRouter>
          <App />
        </MemoryRouter>
      </QueryClientProvider>,
    );
    for (const label of ["Library", "Search", "TorBox", "Downloads", "Settings"]) {
      expect(screen.getAllByText(label).length).toBeGreaterThan(0);
    }
    expect(screen.getByText("Nothing playing")).toBeInTheDocument();
  });
});
