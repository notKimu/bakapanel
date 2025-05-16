<script lang="ts">
	const { used, max }: { used: number; max: number; } = $props();

	const usage = (used / max) * 100;
	const toGb = (bytes: number) => (bytes / (2 ** 30)).toFixed(1);
</script>

<h4>RAM: {toGb(used)}Gb / {toGb(max)}Gb</h4>

<div class="ram-usage">
	<div class="ram-usage__progress progress-indicator segmented">
		<span class="progress-indicator-bar" style={`width: ${usage}%;`}></span>
	</div>
</div>


<style>
	.ram-usage {
		width: 100%;
	}
	
	.ram-usage__progress {
		height: 1.5rem;
	}
</style>