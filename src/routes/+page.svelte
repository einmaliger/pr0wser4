<script lang="ts">
  import type Scene from '$lib/scene';
  import selectedScenes from '$lib/testdata';

  const emptyScene: Scene = {
    id: -1,
    fileName: '',
    directory: '',
    tags: '',
    numGirls: -1,
    numBoys: -1,
    score: 0
  };

  let selected = -1;
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
        /><label for="sc{scene.id}">{scene.name || scene.fileName}</label>
      </li>
    {/each}
  </ul>
</div>
<div style="width:70%; float:right;">
  {#if selected > -1}
    <img src={`https://thumbnail../?id=${selectedScenes[selected]?.id}`} alt="Thumbnail" />
  {/if}
  <div>
    <p>fileName: {selectedScenes[selected]?.fileName}</p>
    <p>Path: {selectedScenes[selected]?.directory}</p>
    <!-- TODO: absolute path including file name -->
    <p>Year: {selectedScenes[selected]?.year || '(unknown)'}</p>
    <p>WebSite: {selectedScenes[selected]?.webSite || '(unknown)'}</p>
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
