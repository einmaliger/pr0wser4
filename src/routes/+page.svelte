<script lang="ts">
  import { getMatches } from '@tauri-apps/api/cli';
  import { invoke } from '@tauri-apps/api/tauri';
  import type Scene from '$lib/scene';
  import SceneInfo from './sceneinfo.svelte';

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
  <SceneInfo {selection} />
</div>

<style>
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
