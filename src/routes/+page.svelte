<script lang="ts">
  import { getMatches } from '@tauri-apps/api/cli';
  import { invoke } from '@tauri-apps/api/tauri';
  import type Scene from '$lib/scene';

  const emptyScene: Scene = {
    id: -1,
    file_name: '',
    directory: '',
    tags: [],
    num_girls: -1,
    num_boys: -1,
    score: 0
  };

  let selected = -1;

  let selectedScenes: Scene[] = [];

  let filename = 'test.pr0';

  getMatches().then((matches) => {
    if (matches.args.database.occurrences > 0) {
      filename = matches.args.database.value as string;
      console.log('found argument: ' + filename);
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
          id="sc{scene.id}"
          bind:group={selected}
        /><label for="sc{scene.id}">{scene.name || scene.file_name}</label>
      </li>
    {/each}
  </ul>
</div>
<div style="width: 60%; position: fixed; right: 0; top: 0;">
  {#if selected > -1}
    <img src={`https://thumbnail../?id=${selectedScenes[selected]?.id}`} alt="Thumbnail" />
  {/if}
  <div>
    <div style="display: flex;">
      <div>{selectedScenes[selected]?.name || selectedScenes[selected]?.file_name}</div>
      <div style="margin-left: auto;">
        {#if selectedScenes[selected]?.score >= 20}<img
            src="star.svg"
            width="16"
            height="16"
            alt="*"
          />{/if}
        {#if selectedScenes[selected]?.score >= 40}<img
            src="star.svg"
            width="16"
            height="16"
            alt="*"
          />{/if}
        {#if selectedScenes[selected]?.score >= 60}<img
            src="star.svg"
            width="16"
            height="16"
            alt="*"
          />{/if}
        {#if selectedScenes[selected]?.score >= 75}<img
            src="star.svg"
            width="16"
            height="16"
            alt="*"
          />{/if}
        {#if selectedScenes[selected]?.score >= 90}<img
            src="star.svg"
            width="16"
            height="16"
            alt="*"
          />{/if}
      </div>
    </div>
    <table>
      <tr>
        <td>File name:</td>
        <td>{selectedScenes[selected]?.file_name}</td>
      </tr>
      <tr>
        <td>Path:</td>
        <td>{selectedScenes[selected]?.directory}</td>
        <!-- TODO: absolute path including file name -->
      </tr>
      <tr>
        <td>Year:</td>
        <td>{selectedScenes[selected]?.year || '(unknown)'}</td>
      </tr>
      <tr>
        <td>Website:</td>
        <td>{selectedScenes[selected]?.website || '(unknown)'}</td>
      </tr>
      <tr>
        <td>Featuring:</td>
        <td>{selectedScenes[selected]?.actors || '(unknown)'}</td>
      </tr>
      <tr>
        <td>Girls/Boys:</td>
        <td
          >{numX(selectedScenes[selected]?.num_girls)} / {numX(
            selectedScenes[selected]?.num_boys
          )}</td
        >
      </tr>
      <tr>
        <td>Tags:</td>
        <td>{selectedScenes[selected]?.tags.join(', ')}</td>
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
