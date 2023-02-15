<script lang="ts">
  import { getMatches } from '@tauri-apps/api/cli';
  import { invoke } from '@tauri-apps/api/tauri';
  import type Scene from '$lib/scene';

  const emptyScene: Scene = {
    file_name: '',
    directory: '',
    tags: [],
    num_girls: -1,
    num_boys: -1,
    score: 0
  };

  let selected = -1,
    selectedScenes: Scene[] = [],
    selection: Scene;

  $: selection = selected >= 0 ? selectedScenes[selected] : emptyScene;

  let filename = 'test.pr0';

  getMatches().then((matches) => {
    const database = matches.args.database;
    if (database.occurrences > 0) {
      filename = database.value as string;
    }
    invoke('load', { path: filename }).then((r) => {
      selectedScenes = r as Scene[];
      selectedScenes.sort((a, b) => {
        const aname = a.name || a.file_name;
        const bname = b.name || b.file_name;
        return aname.localeCompare(bname);
      });
    });
  });

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

<div style="width: 40%;">
  <ul>
    {#each selectedScenes as scene, index}
      <li>
        <input
          type="radio"
          name="scenelist"
          value={index}
          id="sc{index}"
          bind:group={selected}
        /><label for="sc{index}">{scene.name || scene.file_name}</label>
      </li>
    {/each}
  </ul>
</div>
<div style="width: 60%; position: fixed; right: 0; top: 0;">
  {#if selected > -1}
    <img src={`https://thumbnail../?id=x`} alt="Thumbnail" />
  {/if}
  <div>
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
        <td>{selection.directory}</td>
        <!-- TODO: absolute path including file name -->
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
  </div>
</div>

<style>
  img {
    max-width: 100%;
  }
  ul {
    padding: 0;
  }
  li {
    list-style: none;
  }
  input[type='radio'] {
    appearance: none;
    -webkit-appearance: none;
  }
  input:checked + label {
    background: #888;
  }
</style>
