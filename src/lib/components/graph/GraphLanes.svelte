<script lang="ts">
  import type { CommitNode, GraphEdge } from "$lib/types";
  import { laneColor, laneX, ROW_HEIGHT, NODE_RADIUS } from "$lib/utils/graph-layout";

  let {
    nodes,
    edges,
    maxColumn,
  }: {
    nodes: CommitNode[];
    edges: GraphEdge[];
    maxColumn: number;
  } = $props();

  let svgWidth = $derived(maxColumn * 20 + 32);
  let svgHeight = $derived(nodes.length * ROW_HEIGHT);

  // Build a lookup from oid to node for edge rendering
  let nodeMap = $derived(
    new Map(nodes.map((n) => [n.oid, n])),
  );

  // Compute visible edges with coordinates
  let visibleEdges = $derived(
    edges
      .map((edge) => {
        const from = nodeMap.get(edge.from_oid);
        const to = nodeMap.get(edge.to_oid);
        if (!from || !to) return null;

        return {
          x1: laneX(from.column),
          y1: from.row * ROW_HEIGHT + ROW_HEIGHT / 2,
          x2: laneX(to.column),
          y2: to.row * ROW_HEIGHT + ROW_HEIGHT / 2,
          color: laneColor(edge.color_index),
        };
      })
      .filter((e): e is NonNullable<typeof e> => e !== null),
  );
</script>

<svg
  class="pointer-events-none absolute left-0 top-0"
  width={svgWidth}
  height={svgHeight}
>
  {#each visibleEdges as edge}
    {#if edge.x1 === edge.x2}
      <!-- Straight vertical line (same lane) -->
      <line
        x1={edge.x1}
        y1={edge.y1}
        x2={edge.x2}
        y2={edge.y2}
        stroke={edge.color}
        stroke-width="1.5"
        opacity="0.6"
      />
    {:else}
      <!-- Curved line for merge/fork (different lanes) -->
      <path
        d="M {edge.x1} {edge.y1} C {edge.x1} {edge.y1 + (edge.y2 - edge.y1) / 2}, {edge.x2} {edge.y1 + (edge.y2 - edge.y1) / 2}, {edge.x2} {edge.y2}"
        fill="none"
        stroke={edge.color}
        stroke-width="1.5"
        opacity="0.6"
      />
    {/if}
  {/each}

  {#each nodes as node}
    <circle
      cx={laneX(node.column)}
      cy={node.row * ROW_HEIGHT + ROW_HEIGHT / 2}
      r={NODE_RADIUS}
      fill={laneColor(node.column % 8)}
      stroke="var(--color-bg-primary)"
      stroke-width="1.5"
    />
  {/each}
</svg>
