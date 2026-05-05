<script lang="ts">
  import DesignSystem from "$lib/components/DesignSystem.svelte";
</script>

<svelte:head>
  <title>Design System — Glacier</title>
</svelte:head>

<DesignSystem />

<style>
  :global(.app-shell > .nav) {
    display: none;
  }

  :global(.app-shell .main-content) {
    width: 100%;
    max-width: 100%;
    padding: 0;
    margin: 0;
    overflow-x: hidden;
  }

  :global(html) {
    overflow-x: hidden;
  }

  :global(body) {
    overflow-x: hidden;
  }
</style>
