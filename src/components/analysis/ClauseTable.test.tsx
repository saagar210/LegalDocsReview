import { render, screen, fireEvent } from "@testing-library/react";
import { describe, it, expect } from "vitest";
import ClauseTable from "./ClauseTable";
import type { ExtractedClause } from "@/types";

const sampleClauses: ExtractedClause[] = [
  {
    clause_type: "confidentiality",
    title: "Confidentiality",
    text: "All information shared shall be kept confidential.",
    section_reference: "Section 3",
    importance: "high",
  },
  {
    clause_type: "governing_law",
    title: "Governing Law",
    text: "This agreement is governed by the laws of California.",
    section_reference: null,
    importance: "medium",
  },
];

describe("ClauseTable", () => {
  it("renders clause titles", () => {
    render(<ClauseTable clauses={sampleClauses} />);
    expect(screen.getByText("Confidentiality")).toBeInTheDocument();
    expect(screen.getByText("Governing Law")).toBeInTheDocument();
  });

  it("shows empty state when no clauses", () => {
    render(<ClauseTable clauses={[]} />);
    expect(screen.getByText("No clauses extracted yet")).toBeInTheDocument();
  });

  it("expands clause text on click", () => {
    render(<ClauseTable clauses={sampleClauses} />);
    expect(
      screen.queryByText("All information shared shall be kept confidential."),
    ).not.toBeInTheDocument();

    fireEvent.click(screen.getByText("Confidentiality"));
    expect(
      screen.getByText("All information shared shall be kept confidential."),
    ).toBeInTheDocument();
  });

  it("shows importance badges", () => {
    render(<ClauseTable clauses={sampleClauses} />);
    expect(screen.getByText("high")).toBeInTheDocument();
    expect(screen.getByText("medium")).toBeInTheDocument();
  });
});
