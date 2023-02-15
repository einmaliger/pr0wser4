<script lang="ts">
  import type { Scene } from '$lib/scenedatabase';

  export let base_dir: string | null;
  export let selection: Scene;

  // Return displayable version of num_girls / num_boys
  function numX(n: number): string | number {
    switch (n) {
      case -1:
        return 'unknown';
      case 9999:
        return 'many';
      default:
        return n;
    }
  }
</script>

<img src={`https://thumbnail../?id=x`} alt="Thumbnail" />
<div style="display: flex;">
  <div>{selection.name || selection.file_name}</div>
  <div style="margin-left: auto;">
    {#if selection.score >= 20}<img src="star.svg" width="16" height="16" alt="*" />{/if}
    {#if selection.score >= 40}<img src="star.svg" width="16" height="16" alt="*" />{/if}
    {#if selection.score >= 60}<img src="star.svg" width="16" height="16" alt="*" />{/if}
    {#if selection.score >= 75}<img src="star.svg" width="16" height="16" alt="*" />{/if}
    {#if selection.score >= 90}<img src="star.svg" width="16" height="16" alt="*" />{/if}
  </div>
</div>
<table>
  <tr>
    <td>File name:</td>
    <td>{selection.file_name}</td>
  </tr>
  <tr>
    <td>Path:</td>
    <td>{(base_dir || '' ) + '/' + selection.directory}</td>
  </tr>
  <tr>
    <td>Year:</td>
    <td>{selection.year || '(unknown)'}</td>
  </tr>
  <tr>
    <td>Website:</td>
    <td>{selection.website || '(unknown)'}</td>
  </tr>
  <tr>
    <td>Featuring:</td>
    <td>{selection.actors || '(unknown)'}</td>
  </tr>
  <tr>
    <td>Girls/Boys:</td>
    <td>
      {numX(selection.num_girls)} /
      {numX(selection.num_boys)}
    </td>
  </tr>
  <tr>
    <td>Tags:</td>
    <td>{selection.tags.join(', ')}</td>
  </tr>
</table>

<style>
  img {
    max-width: 100%;
  }
</style>
