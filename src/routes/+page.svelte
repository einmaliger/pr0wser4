<script lang="ts">
  import { getMatches } from '@tauri-apps/api/cli';
  import { invoke } from '@tauri-apps/api/tauri';
  import type Scene from '$lib/scene';

  const emptyScene: Scene = {
    id: -1,
    file_name: '',
    directory: '',
    tags: '',
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
    });
  });
</script>

<div style="width:30%; float:left; height: 100%; overflow-y: scroll;">
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
<div style="width:70%; float:right;">
  {#if selected > -1}
    <img src={`https://thumbnail../?id=${selectedScenes[selected]?.id}`} alt="Thumbnail" />
  {/if}
  <div>
    <p>fileName: {selectedScenes[selected]?.file_name}</p>
    <p>Path: {selectedScenes[selected]?.directory}</p>
    <!-- TODO: absolute path including file name -->
    <p>Year: {selectedScenes[selected]?.year || '(unknown)'}</p>
    <p>WebSite: {selectedScenes[selected]?.website || '(unknown)'}</p>
  </div>
</div>

<style>
  img {
    max-width: 100%;
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
