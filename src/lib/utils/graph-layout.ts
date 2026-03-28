export const LANE_COLORS = [
  "#f97316", // orange (accent)
  "#38bdf8", // sky blue
  "#a78bfa", // violet
  "#4ade80", // green
  "#fb7185", // rose
  "#facc15", // yellow
  "#2dd4bf", // teal
  "#f472b6", // pink
];

export const ROW_HEIGHT = 32;
export const LANE_WIDTH = 20;
export const NODE_RADIUS = 4;
export const GRAPH_PADDING = 16;

export function laneColor(index: number): string {
  return LANE_COLORS[index % LANE_COLORS.length];
}

export function laneX(column: number): number {
  return GRAPH_PADDING + column * LANE_WIDTH + LANE_WIDTH / 2;
}
