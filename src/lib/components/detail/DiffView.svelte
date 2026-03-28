<script lang="ts">
  import type { FileDiff } from "$lib/types";

  let { files }: { files: FileDiff[] } = $props();
</script>

<div class="p-3">
  {#each files as file}
    {#if !file.is_binary && file.hunks.length > 0}
      <div class="mb-4">
        <div class="mb-1 rounded-t bg-bg-tertiary px-3 py-1 text-xs font-medium text-text-secondary">
          {file.path}
        </div>

        {#each file.hunks as hunk}
          <div class="border border-border rounded-b overflow-hidden mb-2">
            <div class="bg-bg-tertiary px-3 py-0.5 text-[10px] font-mono text-text-muted">
              {hunk.header}
            </div>

            <div class="font-mono text-xs leading-5">
              {#each hunk.lines as line}
                <div
                  class="flex px-1"
                  class:bg-diff-add-bg={line.kind === "add"}
                  class:bg-diff-remove-bg={line.kind === "remove"}
                >
                  <span class="w-[40px] shrink-0 select-none text-right pr-2 text-text-muted">
                    {line.old_line_no ?? ""}
                  </span>
                  <span class="w-[40px] shrink-0 select-none text-right pr-2 text-text-muted">
                    {line.new_line_no ?? ""}
                  </span>
                  <span class="w-4 shrink-0 select-none text-center"
                    class:text-diff-add={line.kind === "add"}
                    class:text-diff-remove={line.kind === "remove"}
                  >
                    {line.kind === "add" ? "+" : line.kind === "remove" ? "-" : " "}
                  </span>
                  <span class="flex-1 whitespace-pre-wrap break-all text-text-primary">
                    {line.content}
                  </span>
                </div>
              {/each}
            </div>
          </div>
        {/each}
      </div>
    {/if}
  {/each}
</div>
