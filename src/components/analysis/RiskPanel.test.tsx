import { render, screen } from "@testing-library/react";
import { describe, it, expect } from "vitest";
import RiskPanel from "./RiskPanel";
import type { RiskFlag } from "@/types";

const sampleFlags: RiskFlag[] = [
  {
    category: "indemnification",
    severity: "high",
    description: "No indemnification cap specified",
    clause_reference: "Section 5",
    suggestion: "Add a reasonable cap tied to contract value",
  },
  {
    category: "governing_law",
    severity: "medium",
    description: "No governing law clause found",
    clause_reference: null,
    suggestion: null,
  },
];

describe("RiskPanel", () => {
  it("renders risk score", () => {
    render(
      <RiskPanel score={72} level="high" flags={sampleFlags} summary="High risk contract" />,
    );
    expect(screen.getByText("72/100")).toBeInTheDocument();
    expect(screen.getByText("HIGH")).toBeInTheDocument();
  });

  it("renders risk flags", () => {
    render(
      <RiskPanel score={72} level="high" flags={sampleFlags} summary={null} />,
    );
    expect(screen.getByText("No indemnification cap specified")).toBeInTheDocument();
    expect(screen.getByText("No governing law clause found")).toBeInTheDocument();
    expect(screen.getByText("Risk Flags (2)")).toBeInTheDocument();
  });

  it("renders summary when provided", () => {
    render(
      <RiskPanel score={25} level="low" flags={[]} summary="Low risk contract" />,
    );
    expect(screen.getByText("Low risk contract")).toBeInTheDocument();
  });

  it("renders suggestion when available", () => {
    render(
      <RiskPanel score={72} level="high" flags={sampleFlags} summary={null} />,
    );
    expect(
      screen.getByText("Add a reasonable cap tied to contract value"),
    ).toBeInTheDocument();
  });
});
