<script lang="ts">
	const { idx, usage }: { idx: number; usage: number; } = $props();
</script>

<div class="thread">
    <p class="thread__num">{idx}</p>
	
	<div class="thread__usage__progress progress-indicator segmented">
		<span class="progress-indicator-bar" style={`width: ${usage}%;`}></span>
	</div>
</div>

<style>
    .thread {
	  display: grid;
	  grid-template-columns: 1rem 1fr;
	  gap: var(--padding-m);
	}
  
	.thread__num {
	  text-align: right;
	}
	.thread__usage__progress {
		height: 100%;
	}

	.progress-indicator > .progress-indicator-bar {
		background-image: unset;
		background-color: var(--color-4);
	}
</style>