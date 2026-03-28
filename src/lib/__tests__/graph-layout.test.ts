import { describe, it, expect } from "vitest";
import {
  LANE_COLORS,
  ROW_HEIGHT,
  LANE_WIDTH,
  NODE_RADIUS,
  GRAPH_PADDING,
  laneColor,
  laneX,
} from "$lib/utils/graph-layout";

describe("graph-layout", () => {
  describe("constants", () => {
    it("has 8 lane colors", () => {
      expect(LANE_COLORS).toHaveLength(8);
    });

    it("has valid hex colors", () => {
      for (const color of LANE_COLORS) {
        expect(color).toMatch(/^#[0-9a-f]{6}$/);
      }
    });

    it("has reasonable dimensions", () => {
      expect(ROW_HEIGHT).toBeGreaterThan(0);
      expect(LANE_WIDTH).toBeGreaterThan(0);
      expect(NODE_RADIUS).toBeGreaterThan(0);
      expect(GRAPH_PADDING).toBeGreaterThan(0);
    });
  });

  describe("laneColor", () => {
    it("returns the correct color for index 0", () => {
      expect(laneColor(0)).toBe("#f97316");
    });

    it("wraps around for indices beyond the array length", () => {
      expect(laneColor(8)).toBe(laneColor(0));
      expect(laneColor(9)).toBe(laneColor(1));
    });
  });

  describe("laneX", () => {
    it("returns padding + half lane width for column 0", () => {
      expect(laneX(0)).toBe(GRAPH_PADDING + LANE_WIDTH / 2);
    });

    it("increases by LANE_WIDTH for each column", () => {
      const x0 = laneX(0);
      const x1 = laneX(1);
      const x2 = laneX(2);
      expect(x1 - x0).toBe(LANE_WIDTH);
      expect(x2 - x1).toBe(LANE_WIDTH);
    });
  });
});
